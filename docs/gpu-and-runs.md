# GPU capacity, run naming, and the training TODO

Operational notes for training runs. Surveyed **2026-08-15**.

---

## 1. Where we run

### boogiebonjour (primary)

```
kubeconfig : ~/.kube/boogiebonjour
namespace  : default   (context sets none; existing work lives here)
identity   : kubernetes-admin, group kubeadm:cluster-admins
```

> **This is a cluster-admin credential.** The cluster also hosts live inference
> services (flux, llama-3.3-70b, gemma, and others). Create our own objects only —
> never `delete` or `patch` anything we did not create.

Capacity at survey time:

| | |
|---|---|
| nodes | 119 total, 102 schedulable |
| GPU capacity | 816 × H100 80GB |
| allocated | 296 |
| **free** | **520 (63%)** |
| fully-empty untainted 8-GPU nodes | 61 |
| NotReady | 12 |
| cordoned | 2 |
| tainted (`workload`) | `boogiebonjour-052`, `-062` |

**Known-bad nodes** — excluded by the existing production Jobs; reuse the same
`nodeAffinity` blocklist:

```
boogiebonjour-003  -006  -013  -043  -056  -057  -090
             -097  -101  -102  -104  -105  -108
```

### research-common-h100 (do not use)

`kcommon` / `~/.kube/research-common-h100`. Surveyed the same day:

```
21 schedulable nodes | capacity 165 | allocated 165 | free 0 (0%)
preemptible (low priority): 0
```

Fully allocated, nothing preemptible, and node count has dropped from 47 (2026-08-12
snapshot in the `research-common` skill) to 21. **Not usable.** Other clusters with
free H100s exist per KubeRadar (funkyfalcon 710, ragingragnar 481, trickytrout 180,
poutinepenguin 171) but we have no kubeconfig for them.

### Storage — the main operational constraint

The only StorageClass is **`local-path`** (`rancher.io/local-path`, RWO,
`WaitForFirstConsumer`). There is **no shared filesystem** — no Weka, no NFS.

Consequences:

- A PVC binds to whichever node the pod lands on. Data does not move.
- **Checkpoints vanish with the PVC** unless explicitly uploaded somewhere.
- Multi-node training needs a different data strategy entirely. Single-node is fine,
  and everything planned here is single-node.
- Getting data in: `kubectl cp` a tarball. Our dataset is ~10 MB and the repo is
  private, so this avoids putting any credential on the cluster.
- A pod that dies is **re-created on the same node**, because its PVC is there.
  So a node-local transient failure is not escaped by retrying — it burns
  `backoffLimit` in place. Retry *inside* the entrypoint, not by letting the pod
  die; both entrypoints now do.
- The pinning outlives the reason for it. `bok-9b`'s volume was on node 093,
  which filled to 8/8 GPUs while the job was suspended, so its pod became
  permanently unschedulable (`didn't match PersistentVolume's node affinity`).
  **Eval PVCs hold only re-downloadable data, so delete the PVC along with the
  Job when recreating** — that is what lets the scheduler pick a node that still
  has GPUs. Do not do this for training PVCs, which hold the checkpoints.

### `backoffLimit` must be generous *before* anything goes wrong

A Job that reaches `BackoffLimitExceeded` is terminally `Failed`, and **raising
`backoffLimit` afterwards does not revive it**. During the storage outage,
`bok-0`, `bok-1` and `seed-4b` each burned their three attempts on an HF 403
that the client reported as a connection error; the limit was raised to 20 one
minute after they had already failed, so every later action — suspending them,
refreshing their ConfigMaps — was applied to dead Jobs. They had to be recreated
from scratch. Eval Jobs now ship with `backoffLimit: 20`.

The failure count also persists across suspend/resume, so `status.failed` is a
lifetime tally rather than a statement about the current attempt. Alerting on it
directly produces false alarms after any recovery.

### The HF private-storage quota is a hard dependency

Every job fetches its data and pushes its checkpoints through
`jisenli/spec-check-ckpt`, and the account is on the **free tier: 100 GB
private**. Exceeding it does not just block writes — HF answers **403 on every
read of every private repo**, so all training and eval stops at once.

It is easy to exceed by accident and hard to recognise:

- A 4B **full fine-tune** checkpoint is 8 GB, and its `optimizer.pt` is 16 GB.
  Three epochs plus final is ~100 GB from a single run.
- `huggingface_hub` reports the 403 as `LocalEntryNotFoundError`, *"An error
  happened while trying to locate the file on the Hub … Please check your
  connection"*. That wording sent this project's diagnosis to DNS first and to
  token scopes second. **Read the raw HTTP response before believing the
  exception type** — `requests.head(url, headers=…)` returns the real message in
  `x-error-message`.
- Storage is billed on **LFS history, not the current tree**, and only deleting
  the whole repository reclaims it:

  | action | `usedStorage` |
  |---|---|
  | delete 120.82 GB of files | 103.97 GB — unchanged |
  | `super_squash_history()` (history down to 2 commits) | 103.97 GB — unchanged |
  | `delete_repo()` | **freed immediately; reads went 403 → 200** |

  Deleting files and squashing leave the blobs for HF's own GC, which cannot be
  triggered and did not run within the hour. Budget for the quota rather than
  plan to clean up after it — and if it is already blown, the repo has to go, not
  its contents.

**Recovery, when it happens anyway.** Checkpoints also live on the training PVCs
at `/work/out/<run>/`, and those survive the HF repo being deleted — so the
inaccessible copy is the one being removed, which is what makes deletion safe
rather than lossy. `scripts/ckpt_rescue.sh verify|upload` inventories them and
pushes them back, one helper pod per node because `local-path` volumes are only
visible from the node holding them. It also recovered the three runs whose
uploads were cut off mid-quota and which therefore never existed on HF at all.

`scripts/resume_when_quota_clears.sh` polls for reads to work and un-suspends the
eval Jobs when they do. Suspending (`{"spec":{"suspend":true}}`) rather than
deleting is what makes that possible — the Job keeps its identity and its PVC.

---

## 2. Naming rules

**The rule applies to k8s object names, not to personal accounts.** Object names show
up in `kubectl get pods -A`, which everyone on a shared cluster reads constantly.
Personal HF and W&B spaces are private, so names there can be readable.

| Where | Naming | Why |
|---|---|---|
| k8s objects (pod / job / PVC / secret / configmap) | **opaque** `de2-rl-test-*` | continuously visible to every cluster user |
| personal HF repos | descriptive — `jisenli/spec-check-*` | private account; being unrecoverable to yourself is its own cost |
| personal W&B project | descriptive — `jisenli_ai/spec-check` | private entity (`defaultAccess: USER_READ`) |

k8s object names must not contain `spec`, `verus`, `rmm`, `arm`, `cca`, or a personal
username, and must be lowercase (RFC 1123 — `DE2-RL-test-x` is not a legal object
name).

Residual exposure, accepted deliberately: `HF_CKPT_REPO` and `WANDB_PROJECT` are read
by the training job, so they sit in a k8s Secret and a cluster-admin could read them
with `kubectl get secret -o yaml`. That takes deliberate inspection, unlike a pod
name. `make_cluster_secrets.sh` prints a note when it happens rather than blocking.
The dataset repo name never enters k8s at all — data goes in by `kubectl cp`.

### Artifact stores

| | |
|---|---|
| checkpoints | `jisenli/spec-check-ckpt` (private, model) — one repo, one subfolder per run id |
| datasets | `jisenli/spec-check-data` (private, dataset) |
| W&B | `jisenli_ai/spec-check` |

**The run id is the join key across all three systems**: k8s Job
`de2-rl-test-sft2-0` ↔ W&B run `sft2-0` ↔ HF subfolder `sft2-0`. The prefix differs
by system; the suffix does not.

| Purpose | Job / Pod | PVC | Labels |
|---|---|---|---|
| interactive env check | `de2-rl-test-env1` | `de2-rl-test-env1-work` | `owner: de2`, `task: de2-rl-test-env` |
| SFT sweep, run *k* | `de2-rl-test-sft2-<k>` | `de2-rl-test-sft2-<k>-work` | `owner: de2`, `task: de2-rl-test-sft2`, `run: sft2-<k>` |
| eval only | `de2-rl-test-eval2` | `de2-rl-test-eval2-work` | `owner: de2`, `task: de2-rl-test-eval2` |

`sft2` / `eval2` avoid colliding with names already present in `default`
(`de2-rl-test-` + `110`–`114`, `b32-0..7`, `sft-0..7`, `smoke-0..1`, `eval1`,
`stage-20260812`).

> **The `de2-rl-test-` prefix is not ours alone.** Another user (`yjian`) runs
> ~136 GPUs of work under the same prefix in this namespace. Counting our own
> usage by name prefix returned 154 GPUs when the real figure was 18.
>
> **Never select by name prefix — always `-l owner=de2`.** A cleanup like
> `kubectl delete job -n default $(kubectl get job -o name | grep de2-rl-test)`
> would destroy a colleague's running training. The `owner` label is the only
> thing separating the two sets.

Also keep clean of project terms: ConfigMap names, entrypoint script contents
(use `/work/data`, `/work/repo`), and W&B run names. **W&B is off for now** — a run
name would leak more than it is worth.

### Pod spec conventions

Copied from the production training Jobs already running here, not invented:

```yaml
spec:
  runtimeClassName: nvidia
  hostNetwork: true
  hostIPC: true
  restartPolicy: Never
  containers:
  - image: nvcr.io/nvidia/pytorch:25.01-py3
    securityContext: {privileged: true}
    resources:
      limits:   {cpu: "96", memory: 900Gi, nvidia.com/gpu: 8}
      requests: {cpu: "56", memory: 512Gi, nvidia.com/gpu: 8}
    volumeMounts:
    - {name: work,  mountPath: /work}
    - {name: dshm,  mountPath: /dev/shm}
    - {name: ibdev, mountPath: /dev/infiniband}
  volumes:
  - {name: work,  persistentVolumeClaim: {claimName: <job>-work}}
  - {name: dshm,  emptyDir: {medium: Memory, sizeLimit: 256Gi}}
  - {name: ibdev, hostPath: {path: /dev/infiniband}}
```

Jobs use `backoffLimit: 100`, `completions: 1`. PriorityClasses available:
`normal-priority` (10000000), `low-priority` (-100), `eight-gpu` (25000000),
`high-priority` (30000000). Omitting `priorityClassName` is what the existing Jobs do.

Do **not** reuse another user's secrets (`yjian-hf-token`, `de2-datagen-hf-token`).
Ours are created by `scripts/make_cluster_secrets.sh` from a gitignored `.env`:

| Secret | Keys |
|---|---|
| `de2-rl-test-hf` | `token`, `ckpt_repo` |
| `de2-rl-test-wandb` | `token`, `project`, `entity` |

The script refuses to run if `HF_CKPT_REPO` or `WANDB_PROJECT` contains a
project-identifying word, so the naming rule is enforced rather than remembered.
`.env` is never copied into a pod — values arrive via `secretKeyRef`.

## Keeping checkpoints

`local-path` is node-local with `reclaimPolicy: Delete`, so an artifact can be lost
three ways: the PVC is deleted, the node dies, or — the easy one to walk into — the
Job finishes and `kubectl cp` stops working, because it execs into the container and
a `Completed` pod has none.

Three layers, in order of reliability:

1. **Upload to a private HF repo at the end of training** (`HF_CKPT_REPO`). This is
   the cluster's existing convention and the only layer that survives node loss.
2. **Save every epoch** to `/work/out/ep{N}` and write `/work/out/.done` on success.
   LoRA adapters are small (tens of MB at r=16), so per-epoch cost is negligible, and
   a crash mid-run still leaves something.
3. **Rescue pod** if the upload failed. The training Job exits immediately so its
   GPUs are released; a 0-GPU pod pinned to the same node can then mount the same PVC:

   ```bash
   NODE=$(kubectl get pod -n default -l run=sft2-0 \
            -o jsonpath='{.items[0].spec.nodeName}')
   # launch a 0-GPU pod with nodeSelector kubernetes.io/hostname=$NODE
   # mounting claimName de2-rl-test-sft2-0-work, then:
   kubectl cp default/<rescue-pod>:/work/out ./ckpt/sft2-0
   ```

**Never delete a PVC before its artifact is confirmed off the node.**

### Cleanup

An idle GPU pod still holds its GPUs. Delete Jobs and their PVCs when a run is done:

```bash
export KUBECONFIG=~/.kube/boogiebonjour
kubectl get jobs,pods,pvc -n default -l owner=de2
kubectl delete job,pvc -n default -l run=sft2-<k>
```

---

## 3. Run registry

The cluster-side names are opaque; this table is what makes them meaningful.
**Update it in the same commit that launches a run.**

All runs: `dataset_clean` (1310 examples, 293 command), 2 epochs, single node
8×H100, identical hyperparameters apart from the column that varies.

| Run | Base model | Precision | Method | Deps | Isolates | Status |
|---|---|---|---|---|---|---|
| `env1` | — | — | interactive validation | new | — | ✅ G1+G2 passed |
| `sft2-0` | `Qwen/Qwen3-4B` | **bf16** | LoRA r=16 | ngc | — (new baseline) | ✅ trained, uploaded |
| `sft2-1` | `Qwen/Qwen3-4B` | fp16 | LoRA r=16 | ngc | precision | ✅ trained, uploaded |
| `sft2-2` | `Qwen/Qwen3.5-9B` | bf16 | LoRA r=16 | new | base-model capacity | ⏳ running |
| `sft2-3` | `Qwen/Qwen3-4B` | bf16 | full fine-tune | ngc | LoRA rank as bottleneck | ✅ trained, uploaded |
| `sft2-4` | `Qwen/Qwen3.5-9B` | bf16 | full fine-tune | new | capacity × method | ❌ abandoned — OOM |
| `sft2-0-s1337`, `-s2024` | `Qwen/Qwen3-4B` | bf16 | LoRA r=16 | ngc | run-to-run noise | ⏳ running |
| `sft2-1-s1337`, `-s2024` | `Qwen/Qwen3-4B` | fp16 | LoRA r=16 | ngc | run-to-run noise | ⏳ running |
| `sft2-3-s1337`, `-s2024` | `Qwen/Qwen3-4B` | bf16 | full fine-tune | ngc | run-to-run noise | ⏳ running |

The `-s####` runs are **seed replicates, not configurations**: identical data and
hyperparameters, differing only in initialisation and batch order. They exist
because one run per configuration cannot separate a real effect from noise — see
*Statistical power* below. The seed varies training only; the held-out split
stays pinned by `SPLIT_SEED` in `build_dataset.py`, or the replicates would be
scored on different test sets and would not be comparable.

`sft2-4` was dropped rather than fixed. Plain DDP keeps a full copy of weights,
gradients and Adam state on every GPU; for 9B that is ~100 GB per device against
80 GB available, so it needs FSDP/ZeRO sharding. The comparisons that matter —
precision (0 vs 1), capacity (0 vs 2), LoRA vs full (0 vs 3) — are all covered
without it.

### Read this first — corrected conclusions and their evidence

The sections below were written as things were discovered, so several state a
conclusion that a later section withdraws. This table is what survived.

| Claim | Status | Evidence |
|---|---|---|
| SFT is exhausted as a lever | **holds** | loss 0.002; epochs 1/2/3 all 14/40; capacity and method do not separate |
| bf16 = fp16 | **holds** | 3 seeds each, majority vote 37.5% vs 37.5%, p = 1.000 |
| Compiling ≈ 2× correctness | **holds** | Z3 vs gold: 15–25% correct against 35–45% compiling |
| `weaker` = a dropped frame condition | **holds** | 18 of 19, every config, every seed, 6 commands |
| Full FT degenerates (repetition) | **holds** | intervention: 3× the budget made repetition worse |
| Full FT is *worse* | **narrowed** | true on compile rate, **not** on correctness (18.8% vs 19.2%) |
| Restoring the preamble helps | **narrowed** | 9B only: +22.5pp compiling, +7.5pp correct, p = 0.004. 4B p = 0.549 |
| Every pass rate was understated | **withdrawn** | only `sft2-2`; `sft2-0` passed the identical 14 commands before and after |
| 9B has ~20pp of RFT headroom | **withdrawn** | of 16 sampling-recovered specs, 2 are correct |
| Sampling recovers nothing | **withdrawn** | true of `bok-0`'s first ten commands only |

**The single most useful number:** correctness, not compile rate. Every
intervention measured tonight raised compiling faster than correct —
sampling 12.5%, preamble 33%, against a 62.5% base rate — so a compile-rate
delta is an upper bound on the real gain, usually a loose one.

### Results (40 held-out commands)

| Run | | Verus pass | non-degenerate |
|---|---|---|---|
| **gold** | reference | **33/40 (82.5%)** | — |
| `sft2-1` | 4B fp16 LoRA | 18/40 (45.0%) | 39/40 |
| `sft2-0` | 4B bf16 LoRA | **14/40 (35.0%)** | 39/40 |
| `sft2-0` @ epoch 1 | 4B bf16 LoRA | 12/40 (30.0%) | 40/40 |
| `sft2-3` | 4B bf16 full FT | 11/40 (27.5%) | 39/40 |

### Epochs 2 and 3 buy termination, not correctness

Re-run at the corrected decode budget, `sft2-0`'s three checkpoints are
**identical on pass rate and differ only in how often the model fails to stop**:

| checkpoint | epoch | pass@1 | truncated | eval loss |
|---|---|---|---|---|
| `checkpoint-41` | 1 | 14/40 (35.0%) | **13** | 0.0129 |
| `checkpoint-82` | 2 | 14/40 (35.0%) | 4 | 0.0030 |
| `final` | 3 | 14/40 (35.0%) | 3 | 0.0025 |

The last two epochs cut eval loss five-fold and truncation from 13 to 3, without
moving a single command from fail to pass. Whatever that loss bought, it was
surface structure — output length and termination — not correctness.

(The earlier reading of this curve, 30.0% at epoch 1 against 35.0% at final, was
an artifact of the 2048-token cap: epoch 1 truncates far more, so the cap cost it
more.)

**Three independent lines now say SFT is exhausted as a lever**: training loss
reaches 0.002, capacity and method do not separate on pass rate, and training
longer changes nothing. Meanwhile the same checkpoint reaches 42.5% at pass@9 and
the 9B reaches 60.0% against a gold ceiling of 82.5% — the correct answers are in
the distribution, and next-token training is not what will surface them.

Read against 82.5%, not 100%. Non-degeneracy is high, so the model is producing
real specs and failing to compile them — it is not gaming the metric by
emitting `{ true }`. Failure reasons so far are spread across `missing_symbol`,
`parse_error`, `type_mismatch` and `verus_error` rather than concentrated,
matching what Iteration 7 found.

All: `dataset_clean` (1310/91/40), 3 epochs, 8×H100 single node, every epoch
checkpointed to `jisenli/spec-check-ckpt/<run>/checkpoint-*` plus `final`.
`sft2-0` took 486s wall clock.

**The 9B runs use a different dependency stack** (torch 2.9.1 + transformers
5.15, attention on sdpa rather than flash-attn) because `qwen3_5` is unknown to
transformers 4.x. The 4B/9B comparison carries that caveat.

### The training set is fully fitted — SFT scaling is spent

**Verified**, from the run logs:

| Run | train loss (final) | eval loss (epoch 3) | Verus pass |
|---|---|---|---|
| `sft2-0` 4B bf16 LoRA | 0.0022 | 0.0025 | 35.0% |
| `sft2-1` 4B fp16 LoRA | 0.0028 | 0.0015 | 45.0% |
| `sft2-2` 9B bf16 LoRA | 0.0066 | 0.0166 (ep 1) | see below |
| `sft2-3` 4B bf16 full | 0.0039 | 0.0009 | 27.5% |

A training loss of 0.002 after three epochs means the training set is
reproduced essentially perfectly. That single fact explains the whole sweep:
9B ≈ 4B, full fine-tune ≈ LoRA, and three epochs is already past saturation, so
none of capacity, method, or optimisation is the binding constraint. **More
parameters, more epochs, or more of the same objective will not move the pass
rate**, which is exactly what the (statistically indistinguishable) results
show.

**The validation set cannot be used to steer any of this.** It holds only type
definitions and helper stubs — near-mechanical transcription — and its loss is
0.001–0.003. There is no signal in it about command quality, so every training
decision (epoch count, learning rate, early stopping) is currently made blind.
The epoch-curve eval (`eval2-ep`) exists because it is the only feedback that
tracks the objective.

### The decode budget was too small (and it hit the runs unequally)

`--max-new-tokens` defaulted to 2048. The longest gold spec is 12837 characters,
roughly 4400 tokens, so the cap could not fit the hardest commands even in
principle. Truncated generations cluster at 5817–6338 characters while the
longest complete one is 5205 — a cliff at the cap, not a distribution.

| Run | truncated | reported pass |
|---|---|---|
| `sft2-0` 4B bf16 LoRA | 7/40 (17.5%) | 35.0% |
| `sft2-1` 4B fp16 LoRA | 8/40 (20.0%) | 45.0% |
| `sft2-3` 4B bf16 full | **14/40 (35.0%)** | 27.5% |
| `sft2-2` 9B bf16 LoRA | **26/40 (65.0%)** | 25.0% |

It hit the configurations **unequally**, so for the 4B LoRA runs and the 9B it
is a confound rather than a uniform penalty.

**`sft2-3` is the exception, and the reason matters: its truncation is a
symptom, not a penalty.** The full fine-tune degenerates into repetition — its
worst output repeats `!result.is_Ok()` 156 times, 160 of 185 conjuncts being
duplicates — so it truncates because it loops until the cap:

| | duplicate-conjunct fraction | repetitive outputs | truncated |
|---|---|---|---|
| `sft2-0` 4B LoRA | 4.9% | 2/40 | 7/40 |
| `sft2-1` 4B LoRA | — | 3/40 | 8/40 |
| `sft2-3` 4B full | **13.1%** | **7/40** | 14/40 |

Raising the budget does not help it, and re-running the whole eval at 6144
turns that from an inference into an intervention:

| `sft2-3` | 2048 | 6144 |
|---|---|---|
| Verus pass | 11/40 (27.5%) | 12/40 (30.0%) |
| repetitive outputs | 7/40 | **10/40** |
| mean duplicate-conjunct fraction | 13.0% | **19.3%** |
| mean / longest output | 3369 / 6676 chars | **6275 / 19973 chars** |

Tripling the budget recovered exactly one command (`RMI_RTT_FOLD`) while nearly
doubling the mean output and tripling the longest one, and repetition got
*worse* on every measure. **The extra room went almost entirely into looping.**
The model is not running out of budget, it is failing to stop.

With three seeds each, "full fine-tuning is worse" has to be stated more
precisely, because the two halves of it have very different evidence behind them:

| | LoRA (3 seeds) | full FT (3 seeds) | verdict |
|---|---|---|---|
| pass@1 | 35.0 / 37.5 / 40.0% | 30.0 / 27.5 / 22.5% | majority 37.5% vs 27.5%, McNemar **p = 0.289 — not significant** |
| truncated /40 | 3, 4, 4 | 9, 17, 18 | **ranges do not overlap**; exact permutation p = 0.100, which is the *minimum attainable* with 3-vs-3 |

So full fine-tuning is not demonstrably worse at *writing specs* — that
comparison is still unresolved. It is demonstrably **more prone to looping**:
all three LoRA runs rank below all three full runs on truncation with no
overlap, and the degeneration is separately confirmed by intervention (tripling
the budget made repetition worse, above). The low pass rate is a downstream
consequence of that defect rather than an independent capability gap.

Worth noting which measurement carried the result. On pass rate this comparison
ends in "not significant, no conclusion". The signal is entirely in the
degeneracy metric — which only exists because `non_degenerate` was found to be
blind to repetition while investigating something else.

This is also the clearest illustration of why a pass rate alone misleads here.
The same broken model reads as 27.5% under one decode cap and 30.0% under
another, and `non_degenerate` called it 39/40 in both — the defect was being
clipped at different points rather than measured.

This was invisible because `non_degenerate` only looked for specs that are too
SMALL (`{ true }`, no implication). It scored `sft2-3` at 39/40 while a sixth of
its outputs were loops. A duplicate-conjunct check is now part of it — `a && a`
is just `a`, so repeated conjuncts are semantically free and easy to count —
which moves `sft2-3` to 32/40 and the LoRA runs to 36–37/40.

Half the commands that never passed under any run (7 of the 14 where gold does
compile) were simply never allowed to finish: `RMI_REALM_DESTROY`,
`RMI_RTT_AUX_MAP_PROTECTED`, `RMI_RTT_FOLD`, `RMI_RTT_MAP_UNPROTECTED`,
`RMI_RTT_READ_ENTRY`, `RMI_VDEV_DESTROY`, `RMI_VDEV_P2P_BIND`. Genuinely-hard
commands number 7, not 14.

**Re-running at 6144 showed this claim was too broad.** `sft2-0` passes exactly
the same 14 commands before and after — `+0/-0`, despite a tripled budget and a
corrected prompt. Four of its seven truncated commands stopped being truncated
and still failed, for real reasons. So the 4B LoRA runs were **not** understated:
the truncated commands would have failed anyway, and 35.0% was right all along.
The confound was real but its size has to be measured per run, not assumed.

For `sft2-2` the effect is severe enough that its number measures nothing:
16 commands were lost to reasoning (below) and 10 more to spec truncation, and
of the 14 that ran to completion **10 compiled**. That subset is biased — the
commands that fit are the short ones, which are also the easy ones — so 10/14 is
an over-estimate, not the model's real rate. What it does establish is that
25.0% is not a capability measurement.

Fixed by raising the default to 6144 and counting truncation from both the token
cap and an unbalanced brace.

### The failure taxonomy was wrong, and failures were not diagnosable

`classify_failure()` tested a bare `"expected"` third in its cascade. That
substring appears in most rustc diagnostics — `expected 2 arguments, found 3`,
`expected struct \`Foo\`` — so arity and type errors were being reported as
`parse_error`. That bucket was 9/5/11 of the failures per run, i.e. most of what
the taxonomy claimed to show was mislabelled. Fixed: syntax patterns are tested
first (parsing precedes name resolution), `wrong_arity` and `bad_field_access`
are split out, and unrecognised output becomes `other_compile_error` instead of
being folded into a named bucket.

Compounding it, eval output stored only the reason string — not the compiler
output — so no failure could be examined without re-running the whole eval. Eval
JSONs now keep `output_head`, the raw decoder text when extraction fails, and a
truncation count.

Re-checking all 160 stored generations under the corrected classifier
(`scripts/analyze_failures.py`, which needs Verus but not the network):

| | `sft2-0` | `sft2-1` | `sft2-3` | `sft2-2` |
|---|---|---|---|---|
| `parse_error` before | 9 | 5 | 11 | 2 |
| `parse_error` after | **0** | **0** | **0** | **0** |

**Not one generation was ever syntactically invalid.** The 27 reported syntax
errors were type errors, arity errors, and — mostly — truncation, which Verus
reports as `mismatched closing delimiter` and therefore reads as a mistake the
model made rather than an answer it was cut off from finishing. Two layers of
mislabelling stacked, which is why the decode budget went unnoticed for a whole
sweep.

### `sft2-2` (9B): the number does not measure the model

The 9B eval returns `no_pub_open_spec_fn_found` on 16 of 40 commands. **Cause
confirmed by inspecting the outputs:** handed an open `<think>`, the model
reasons in prose — *"Let me analyze this RMI_DATA_CREATE_UNKNOWN command
specification: 1. **Inputs**: rd (Address), data (Address)…"* — and hits the
token cap before writing any spec. All 16 are cut off at 4.8k–9.7k characters,
i.e. exactly `--max-new-tokens`. Training taught it to close the block
immediately (loss 0.0066, token accuracy 0.998), but on longer sections the base
model's reasoning prior wins.

**Verified** while investigating: both Qwen templates render an empty
`<think>` block into the training conversation, but `add_generation_prompt=True`
cuts them in different places —

```
training render (both) ...assistant\n<think>\n\n</think>\n\n<spec><|im_end|>
4B generation prompt   ...assistant\n
9B generation prompt   ...assistant\n<think>\n
```

— so `train.py`'s length-based masking supervises the 4B on the think block
(it learns to emit it) and the 9B only from `</think>` onward. Both are
internally consistent, so **this is not yet shown to be the cause**. It does
mean the two models' eval paths differed by accident, so eval now derives the
prompt by cutting the training render at the answer, identically for every
template. Greedy numbers produced this way are comparable only to each other.

### Precision: bf16 and fp16 are identical, and the 10pp gap was seed noise

Three seeds each (42 / 1337 / 2024), scored on the same 40 commands:

| | seed 42 | seed 1337 | seed 2024 | range |
|---|---|---|---|---|
| bf16 | 35.0% | 37.5% | 40.0% | 35–40 |
| fp16 | 45.0% | 42.5% | 40.0% | 40–45 |

Majority vote across seeds: **bf16 15/40 (37.5%), fp16 15/40 (37.5%)** — equal,
with 2 discordant commands out of 40 and McNemar p = 1.000. The ranges overlap
at 40.0%, and the original 10pp gap came from comparing bf16's worst seed with
fp16's best.

**Seed noise is ~5pp, flipping 5–7 of the 40 commands.** Treat any
between-configuration difference below that as unmeasured until it has been run
on multiple seeds. Nothing about the eval set changed to establish this — the
same configuration was simply trained twice more.

### pass@k is the measurement that has statistical power here

The same three runs, the same 40 commands, the same paired test — only the
per-command measurement changes, from one greedy sample to nine:

| comparison | greedy discordant | p | pass@9 discordant | p |
|---|---|---|---|---|
| bf16 vs fp16 | 1 : 5 | 0.219 | **0 : 6** | **0.031** |
| bf16 vs 9B | 1 : 3 | 0.625 | **0 : 7** | **0.016** |
| fp16 vs 9B | 4 : 2 | 0.688 | 1 : 2 | 1.000 |

Greedy resolves nothing. pass@9 turns the discordant pairs one-sided, because it
estimates the same underlying quantity with a ninth of the sampling noise.

**This is the way out of the resolution problem**, and it is not the one this
document previously reached for. Enlarging the held-out set was costed and
rejected: 40 → 80 spends 56% of the training data to move the detectable
difference from 22pp to 15pp, and even surrendering all 98 alp14 commands only
reaches 14pp (see *Statistical power* below). Sampling more per command costs
inference time instead of training data, and inference is cheap. **Compare runs
on pass@k, not pass@1.**

Read with care on two counts. Three pairwise tests need a multiple-comparison
correction: at Bonferroni α = 0.0167, bf16-vs-9B survives and bf16-vs-fp16 does
not. And these are single seeds — `seed-a`/`seed-b`/`seed-c` are what show
whether bf16 trailing is stable across them.

### Capacity shows up in pass@k, not in pass@1

Best-of-9 at temperature 0.8, on the same 40 commands:

| | pass@1 | pass@9 | headroom | no sample passes |
|---|---|---|---|---|
| `sft2-0` 4B bf16 LoRA | 14/40 (35.0%) | 17/40 (42.5%) | +3 | 23/40 |
| `sft2-2` 9B bf16 LoRA | 16/40 (40.0%) | **24/40 (60.0%)** | **+8** | **16/40** |

Greedily the 9B leads by 2 commands, which McNemar puts at p = 0.625 — nothing.
Its pass@9 leads by 7, and it has seven fewer commands where all nine samples
fail. **The larger model does know more; it just does not rank it first.**

This reframes the sweep's original "9B ≈ 4B, so capacity is not the bottleneck".
That conclusion was an artifact of measuring with pass@1. Capacity does not help
*fitting* — training loss is 0.002 either way — but it does change how much
correct mass the distribution contains.

It also sets where rejection-sampling work should go, if the faithfulness gate
ever opens: **+20pp of reachable headroom on the 9B against +7.5pp on the 4B.**
And it retracts the earlier reading of `bok-0`'s first ten commands as evidence
that sampling recovers nothing — that was true of those ten and of no run in
full.

### Statistical power: what 40 commands can and cannot resolve

**Verified.** The 40-command eval set gives a binomial standard error of ~7.7pp
around a 35% rate, so two runs must differ by roughly **22pp** before the
difference is distinguishable from noise. `sft2-1` leads `sft2-0` by 10pp. That
is inside the noise floor, so the ranking above is not yet evidence that fp16
beats bf16.

Paired analysis is stronger and free, because the same 40 commands are scored by
every run — command difficulty, the dominant noise source, cancels. Comparing
`sft2-0` against `sft2-1` command by command:

| | count |
|---|---|
| both pass | 13 |
| both fail | 21 |
| only `sft2-1` (fp16) passes | 5 |
| only `sft2-0` (bf16) passes | 1 |

**34 of 40 commands behave identically under both runs**, so the entire signal
lives in 6 disagreements. McNemar's exact test on 5 vs 1 gives **p = 0.219** —
not significant. The 10pp gap is unsupported even under the more sensitive test.

**Assumed, not verified:** that the residual difference is seed noise rather than
a real but small precision effect. That is exactly what the `-s####` replicates
are being run to check.

Enlarging the eval set does not fix this and was **rejected**: the split is by
command name, so every command moved into the test set leaves the training set.
Going 40 → 80 costs 56% of the training data (293 → 129 command examples) and
only improves the detectable difference from 22pp to 15pp. Even surrendering all
98 alp14 commands to evaluation — an 85% cut to training data — only reaches
14pp. 121 command names is a structural ceiling that no split can escape
(`docs/dataset.md`). Power has to come from somewhere other than the split:

1. **Paired (McNemar) comparison** as the default analysis, never marginal rates.
2. **Seed replicates.** Averaging *k* seeds shrinks the standard error by √*k*;
   3 seeds is worth roughly a 3× larger eval set, at zero cost in training data.
3. **Other specifications** (PSCI / SDEI / DRTM) as additional held-out sets.
   These add samples the model has never seen *and* answer the more valuable
   question — whether anything generalises beyond RMM.

### Compiling is worth about half what it looks like

`scripts/semantic_equiv.py` asks Z3 whether a generated spec *means* the same
thing as gold, in both directions. Over the specs that compile:

| | compiles | equivalent | minus vacuous gold | **actually correct** |
|---|---|---|---|---|
| `sft2-0` 4B bf16 | 14/40 (35.0%) | 7 | 6 | **6/40 (15.0%)** |
| `sft2-1` 4B fp16 | 18/40 (45.0%) | 9 | 8 | **8/40 (20.0%)** |
| `sft2-2` 9B | 16/40 (40.0%) | 11 | 10 | **10/40 (25.0%)** |

**Roughly half of what compiles disagrees with gold**, so a compile rate is
about double the rate of specs that say the right thing. This is the constraint
"compiling is not a golden trajectory" turned into a number, and the number is
worse than the phrasing suggests.

The disagreements have shape, not noise:

- **weaker** (4) — permits behaviour gold forbids, the failure compile-success
  structurally cannot see. `RMI_DATA_DESTROY` for both 4B runs, `RMI_VSMMU_MAP`.
- **stronger** (5) — forbids behaviour gold permits. `RMI_VSMMU_CREATE` for all
  three models.
- **incomparable** (9) — disagrees both ways; 5 of them in `sft2-1` alone.

The same command failing the same way across independent models is a shared
misreading of the text, not sampling noise, and is where a faithfulness effort
should start.

**It also reorders the models.** By compile rate: fp16 45% > 9B 40% > bf16 35%.
By correctness: **9B 25% > fp16 20% > bf16 15%**. The 9B converts 11 of 16
compiling specs into correct ones (69%) where both 4B runs manage half. That
agrees with the pass@k result: the 9B's advantage shows up on two axes pass@1
cannot see — how much correct mass the distribution holds, and how faithful the
output is once it compiles.

**Caveats, both material.** Gold is a human reading of the PDF, so agreement
with gold is a weaker claim than faithfulness to the text. And 3 of the 98 gold
specs are literally `{ true }` — one of them, `PSCI_CPU_OFF`, is in the held-out
40, so every model scores a free pass there and "equivalent" means nothing on
that command. Subtract it from every pass rate in this document.

### The pass@k headroom is mostly not real — do not train on compile-success

Running the same Z3 comparison over the specs that *sampling* recovered — the
commands where greedy failed but some sample compiled — collapses the case for
rejection-sampling:

| | greedy compiles → correct | sampled extras → correct |
|---|---|---|
| `sft2-0` 4B bf16 | 6/14 (42.9%) | **0/3 (0%)** |
| `sft2-1` 4B fp16 | 8/18 (44.4%) | 1/5 (20.0%) |
| `sft2-2` 9B | 10/16 (62.5%) | **1/8 (12.5%)** |

For the 9B, pass@k takes compiling specs from 16 to 24 and correct ones from
**10 to 11**. Of the 16 extras across all three runs, 2 are right.

So the earlier reading — "+20pp of reachable headroom on the 9B, put
rejection-sampling work there" — **is withdrawn**. Sampling does not surface
answers the model knows; it surfaces more ways to compile something wrong, and
it does so at roughly a quarter of greedy's hit rate. **A reward of
compile-success would specifically reinforce specs that compile and say the
wrong thing**, which the model is already good at producing. The project's
standing rule that compiling is not a golden trajectory now has a measurement
behind it rather than an argument.

This does not retire pass@k as an *instrument*. Its statistical power (above) is
real, because it estimates a per-command quantity with less sampling noise. It
just cannot be an optimisation target: what it measures well is the propensity
to compile, not the propensity to be right.

It also softens the pass@9 model ranking. That comparison separated the 9B from
bf16 at p = 0.016, but on correctness the three runs are 10 / 8 / 6 — the same
order, a much smaller gap, and nothing tested for significance.

**Where this leaves the roadmap.** The faithfulness gate was the blocker on
RFT/DPO/RL, and it now exists and is validated in both directions. Its first
answer is not "you may proceed" but "the thing you were going to do would make
matters worse". The open direction is getting the model right on the first
attempt — which is what the `pre-*` (restore the training-time symbol table) and
`rep-*` (feed the compiler error back) experiments test, and they have to be
judged on semantic equivalence rather than on compile rate.

### Correctness reorders the configurations again, and flattens them

Applying the Z3 comparison to every seed replicate:

| config | compile rate | **correctness** | compiles that are correct |
|---|---|---|---|
| `sft2-2` 9B (1 seed) | 40.0% | **25.0%** | 62.5% |
| `sft2-1` 4B fp16 | 42.5% ±5.0 | 20.8% ±2.5 | 49% |
| `sft2-0` 4B bf16 | 37.5% ±5.0 | 19.2% ±7.5 | 51% |
| `sft2-3` 4B full FT | 28.8% ±2.5 | **18.8%** ±2.5 | **65%** |

**The full fine-tune is not the outlier it appeared to be.** It compiles 9–14
points less often than LoRA but is correct within 2 points of it, because the
specs it does produce are right 65% of the time against LoRA's ~50%. Its
degeneration cost it compilations that were largely going to be wrong anyway.

**On correctness, every 4B comparison is inside seed noise.** bf16 alone spreads
7.5pp across seeds, which swallows every gap between the three 4B configurations.
The 9B leads by ~5pp but has one seed, so it is not resolved either.

The two metrics also have *different* noise: bf16 is ±5.0pp on compile rate and
±7.5pp on correctness, fp16 is ±5.0 and ±2.5. They are not two readings of one
quantity, and reporting only the compile rate ranks fp16 first when it is not.

### Restoring the training-time preamble

Training put a 200-line symbol table in every prompt; inference removed it on
the assumption the model had memorised it. Over the first 14–16 commands,
restoring it moves the 9B from 7 to 12 and the 4B fp16 from 6 to 11, **with no
command lost in either**, and the 9B's `missing_symbol` failures go 3 → 0.

**Result, for the 9B (complete, 40 commands):**

| | compiles | equivalent | minus vacuous | **correct** |
|---|---|---|---|---|
| no preamble | 16/40 (40.0%) | 11 | 10 | **25.0%** |
| **with preamble** | **25/40 (62.5%)** | 14 | 13 | **32.5%** |

+9 compiles with **none lost**, McNemar p = 0.004. A single greedy generation
beats best-of-9 sampling (24/40), and 25/40 is 76% of the 33/40 gold ceiling.

**But the compile-rate delta is three times the correctness delta**: +22.5pp
against +7.5pp. Of the 9 newly-compiling commands only 3 are right — a 33%
conversion, better than sampling's 12.5% but below the model's own 62.5% base
rate. Anything that raises the compile rate appears to dilute correctness; the
question is only by how much.

**The two defects are independent.** The preamble fixes API misuse — the 59% of
failures that were `missing_symbol`, `type_mismatch` and `wrong_arity`, with
`missing_symbol` going to zero. It does nothing about frame conditions: of the 5
newly-compiling-but-wrong specs, 3 are `weaker`, the same dropped-frame-condition
failure as before. Specs now clear the compile bar still carrying the semantic
defect underneath it.

**It does not replicate on the 4B.** Running the same intervention on
`sft2-1`:

| | compiles | correct | McNemar |
|---|---|---|---|
| **9B** | 40.0% → **62.5%** (+9, **−0**) | 25.0% → **32.5%** | **p = 0.004** |
| 4B fp16 | 45.0% → 52.5% (+7, **−4**) | 20.0% → 22.5% | p = 0.549 |

The 4B gains 7 commands and loses 4, which is noise, and its correctness moves
+2.5pp — exactly the seed spread measured for that configuration. So the claim
is **"restoring the symbol table helps the 9B"**, not "helps". Whether that is
about capacity, about a 9000-character table crowding the 4B's attention, or
something else is unknown and untested.

So the roadmap has two separate targets, and only one of them has been addressed
on one model: supplying the symbol table is a free win on the 9B, and the
frame-condition omission is untouched by it anywhere.

### What the model actually drops: frame conditions

Two commands are gotten wrong the same way by every run that compiles them —
`RMI_DATA_DESTROY` is **weaker in 7 of 8**, `RMI_VSMMU_MAP` weaker in 5. A shared
direction across independent seeds and model sizes is a shared misreading, not
noise.

Textual clause diffing cannot say what was dropped: each run "misses one of 18
clauses" and no two miss the same one, which is what reworded parenthesisation
looks like. `scripts/ablate_clause.py` settles it with Z3 — remove gold's clause
*i* and re-test, and the clause whose removal stops the candidate being weaker is
the one it failed to say. For `RMI_VSMMU_MAP`, removing clause 20 makes gold
**exactly equivalent** to the model: that single clause is its only defect.

Both dropped clauses are the same kind of thing, about the same field:

```
RMI_VSMMU_MAP  #20  RttWalk(new_s,…).rtte.ripas == RttWalk(old_s,…).rtte.ripas
RMI_DATA_DESTROY #17  !(result.is_Ok() && …ripas == RAM) ==> (same equality)
```

**Frame conditions — "this state is unchanged" — and specifically over the RTT
walk's RIPAS field.** Dropping one is invisible to compilation (the spec still
builds, it just permits more) and is the dominant shape of the `weaker` verdict,
which is the failure class compile-success structurally cannot see.

The V3 prompt already ends with *"Keep unchanged-state constraints when implied
by the command behavior"*, so the model is failing at the one thing its prompt
explicitly asks for.

**This is not two examples — it is the rule.** Running the ablation over every
`weaker` verdict across all nine scored runs (`scripts/ablate_all_weaker.py`):

| what the dropped clause was | count |
|---|---|
| **frame condition** (`new_s … == old_s …`) | **18** |
| anything else | 1 |

**18 of 19.** It spans every configuration — 4B bf16, 4B fp16, 9B, full
fine-tune — and every seed, over six distinct commands (`RMI_DATA_DESTROY` 7×,
`RMI_VSMMU_MAP` 5×, `RMI_RTT_CREATE`, `RMI_DATA_CREATE_UNKNOWN`, `RMI_RTT_FOLD`,
`RMI_RTT_AUX_UNMAP_UNPROTECTED`). The single exception is a postcondition on an
output value, not a frame condition.

So the entire `weaker` failure mode — the one compile-success structurally cannot
see, and the one that matters, since a spec that permits too much is what makes a
verifier miss bugs — reduces to **one omission the model makes consistently:
it does not say what stays the same.** That is a far more tractable target than a
50% aggregate.

**A syntactic proxy exists but is weaker than it looks.** Counting clauses that
relate `new_s` back to `old_s`:

| verdict | n | model | gold | diff | short of gold |
|---|---|---|---|---|---|
| equivalent | 80 | 1.5 | 1.5 | **0.00** | **0%** |
| stronger | 12 | 5.5 | 4.3 | +1.17 | 0% |
| weaker | 19 | 6.1 | 6.8 | −0.68 | **68%** |
| incomparable | 16 | 3.8 | 4.1 | −0.38 | 25% |

Equivalent specs match gold's frame count exactly and not one is short, and 68%
of weaker ones are short — so the count carries real signal. But it **needs gold
as its reference**: nothing about "6 frame clauses" says whether 6 is enough. It
is therefore *not* a gold-free check, and it also misses the 32% of weaker specs
that carry the right number of frame clauses with the wrong content. Turning this
into a training-time signal would mean predicting the required frame conditions
from the section text, which is a separate problem.

### Four inference-time interventions, scored on correctness

All on `sft2-2` (9B), all zero-retraining, all judged with `semantic_equiv.py`:

| intervention | compiles | **correct** | Δcompile | Δcorrect | conversion |
|---|---|---|---|---|---|
| greedy baseline | 16 (40.0%) | **10 (25.0%)** | — | — | 62.5% base |
| **restore preamble** | 25 (62.5%) | **13 (32.5%)** | +9 | **+3** | **33.3%** |
| self-repair, 2 rounds | **31 (77.5%)** | 12 (30.0%) | **+15** | +2 | 13.3% |
| best-of-9 sampling | 24 (60.0%) | 11 (27.5%) | +8 | +1 | 12.5% |
| preamble + frame hint | 22 (55.0%) | 10 (25.0%) | −3 | −3 | — |

**Compile-rate gain barely predicts correctness gain.** Self-repair nearly
doubles compilation — 31/40 against gold's own 33/40 ceiling — and buys two
correct specs. Every intervention converts new compilations at well under the
model's own 62.5% base rate.

**And the conversion rate is a property of the intervention, not the model.**
Repeating all four on `sft2-1` (4B):

| intervention | 9B compile / correct | 4B compile / correct | **conversion 9B / 4B** |
|---|---|---|---|
| baseline | 40.0% / 25.0% | 45.0% / 20.0% | 62.5% / 44% (base) |
| restore preamble | 62.5% / 32.5% | 52.5% / 22.5% | **33% / 33%** |
| self-repair | 77.5% / 30.0% | 62.5% / 22.5% | **13% / 14%** |
| + frame hint | 55.0% / 25.0% | 50.0% / 20.0% | **0% / 0%** |

Two models differing in size, base rate and instruction-following convert new
compilations at nearly identical rates within each method. That gives a usable
rule for evaluating any future technique:

> **new-correct ≈ new-compiling × the method's conversion rate**, and a method
> converting below the model's own base rate produces output *worse* than what
> the model emits unprompted.

Three of the four do. The exception, restoring the preamble, is not really an
improvement to the model — it removes a self-inflicted handicap, since evaluation
had been withholding the symbol table that every training prompt contained.

Self-repair's ceiling is structural, not an implementation limit: its feedback is
the compiler, and a compiler cannot report a missing frame condition, because a
spec that omits one is perfectly legal. It fixes API misuse to near the ceiling
and is blind to the semantic defect by construction.

**Sampling is closed out.** pass@9 = 60.0%, pass@24 = 57.5%, pass@16 at
temperature 1.1 = 62.5%. Tripling the budget buys nothing; the ceiling is ~60%
compiling, of which the recovered portion is 87.5% wrong.

### The frame-condition hint works, and overshoots

Adding an explicit demand for frame conditions on top of the preamble:

| | compiles | correct | `weaker` | `stronger` |
|---|---|---|---|---|
| preamble only | 25 | **13** | **4** | 2 |
| + frame hint | 22 | 10 | **1** | **6** |

Net effect negative, **but the mechanism works**: the target failure mode drops
by 75%, and the specs land on the other side of correct — over-constrained rather
than under-constrained. The model reads the instruction and acts on it; it
applies it to state that is *supposed* to change.

That refutes the simpler reading, that a model at loss 0.002 merely replays
training patterns and cannot take instruction. It takes it. What it lacks is the
judgement of *which* state is meant to be invariant — which is the same reading
comprehension the task requires in the first place.

It is also the only intervention tested that moves the semantic axis at all; the
other three move compilation and leave `weaker` where it was. Worth calibrating
(demand frame conditions only where the document states the state is preserved)
rather than abandoning — untested, and a decision for a human.

**And it does not replicate on the 4B**, which responds to the same instruction
in form rather than in substance:

| `sft2-1` 4B | compiles | correct | `weaker` | `stronger` | repetitive |
|---|---|---|---|---|---|
| preamble only | 21 | **9 (22.5%)** | 2 | 3 | 3 |
| + frame hint | 20 | 8 (20.0%) | **3** | 3 | **5** |

`weaker` does not fall; repetition rises. Told to assert more invariants, the 4B
emits more clauses rather than more *correct* clauses.

### The 9B differs from the 4B in ways pass@1 cannot see

Three independent interventions, one conclusion:

| | 9B | 4B |
|---|---|---|
| restore preamble | +22.5pp compiling, p = 0.004 | +7.5pp, p = 0.549 |
| pass@k headroom | +8 commands | +3 commands |
| frame-condition instruction | `weaker` 4 → 1 | `weaker` 2 → 3, repetition ↑ |

All three move the 9B semantically and none move the 4B, while their greedy
compile rates sit at 40.0% and 45.0% — the 4B *ahead*. **The 9B adjusts what its
specs mean; the 4B adjusts how they look.** For choosing a base model that is a
far stronger argument than a compile rate, and it is invisible to the metric the
project has been using.

### Measured ceiling: gold itself is 33/40

Before reading any pass rate: **the gold specs compile on only 33 of the 40
held-out commands (82.5%)**, and 79/98 across all of alp14, under Verus
0.2026.04.12.f1166c4. The seven that fail are `RMI_PSMMU_MSI_CONFIG`,
`RMI_REALM_CREATE`, `RMI_VDEV_GET_STATE`, `RSI_MEASUREMENT_READ`,
`RSI_MEM_SET_PERM_INDEX`, `RSI_REALM_CONFIG`, `RSI_VDEV_GET_INFO`.

So 82.5% is the practical ceiling, not 100% — and the historical Claude figure
of 96.94% is *above what gold achieves*. A model whose output compiles more
reliably than the reference is a degeneracy signal, not a win, which is why
`eval_checkpoint.py` reports non-degeneracy alongside pass rate.

`sft2-0` is a **new baseline, not comparable to the historical v4 numbers** — the
framework, precision, dataset, and eval set all changed at once. Precision stays a
clean comparison because `sft2-0` vs `sft2-1` differ in nothing else.

---

---

## 4. Training TODO

### The constraint that shapes everything

Compile-pass is **not** a golden-trajectory signal. A spec that reads
`pub open spec fn f(...) -> bool { true }` compiles perfectly and is worthless — and
the model has already produced exactly this failure (`BUG_REPORT.md`, PSCI Bug 4).

This rules out, for now, **every method whose training signal is "it compiled"**:

- distilling Claude's passing outputs
- rejection-sampling / STaR / RFT on self-generated passing samples
- RL against a compile reward (worst of the three — it actively optimizes the proxy)

Methods therefore split into two classes:

| Class | Safe? | Why |
|---|---|---|
| **Capacity / numerics / hyperparameters**<br>(base model, bf16, full-FT vs LoRA, sequence handling) | ✅ | Compile-pass is never used as a training signal. A gain here is a real gain. |
| **Data generation / DPO / RL** | ❌ *for now* | Training signal is compile-pass; drifts toward compilable nonsense. |

**Only the first class is in scope until faithfulness is measurable.**

### Phase 0 — blockers

- [ ] Filter evaluation to the 40 held-out commands in `dataset_clean/splits.json`.
      `dataset_loader.load_dataset(split="test")` still returns all 98; running it
      as-is re-introduces the leak. See [`data-leakage.md`](data-leakage.md).
- [ ] Re-baseline **all three** models (Qwen v4, Claude, GPT) on the same 40. Every
      previously reported pass rate is scored on a different, contaminated set.
- [ ] Verify Verus runs inside the pod. The bundled binary is Linux x86 and the nodes
      are Linux x86, so it should work — confirm before, not after, a training run.

### Phase 1 — tonight: controlled sweep on the existing dataset

Data is fixed at 1310 examples (293 command). Only the model and numerics move.

- [ ] **`env1`** — interactive pod: torch, deps, `kubectl cp` the data, pull both base
      models, run Verus once. Do not launch training Jobs until this passes.
- [ ] **Port `train.py` off Unsloth.** It was written for a 48 GB Turing card and
      carries workarounds that are now liabilities:

      | current | change to | why |
      |---|---|---|
      | Unsloth | plain `peft` + `trl` | 8×H100 has 640 GB; the memory workarounds cost quality and caused the `padding_free` / `use_gradient_checkpointing="unsloth"` bugs |
      | `load_in_4bit=True` | unquantized bf16 LoRA | 4-bit was a memory compromise |
      | `fp16=True` | `bf16=True` | tonight's variable |
      | xformers | flash-attention-2 | available on H100 |
      | single GPU | 8-GPU DDP/FSDP | |

      Keep the one lesson that still applies: `SFTConfig` takes `max_length`, not
      `max_seq_length` — passing the wrong name fails silently.
- [ ] **bf16 vs fp16** (`sft2-0` vs `sft2-1`). All prior training was fp16 because
      the old GPU was Turing. Qwen3 is bf16-native, and fp16 fine-tuning of a
      bf16-native model is a known silent-degradation mode. Possibly a free win, and
      it establishes the new baseline for everything after.
- [ ] **Base-model ladder** (`sft2-2`, `sft2-3`). The strongest evidence in the whole
      project is that **Claude Haiku scores 96.94% with zero training while the
      fine-tuned 4B scores 48%** — base capability dominates. On 8×H100 the whole
      ladder is minutes-to-an-hour per rung. Watch for overfitting: 1310 examples is
      small, and val is type/helper-only, so it is biased for commands. Fix epochs at
      2 for this sweep to isolate the size variable.
- [ ] **Full fine-tune vs LoRA** (`sft2-4`), 4B only, one data point.

Metrics, identical for every run:

| | |
|---|---|
| primary | Verus pass@1, 40 held-out commands, fresh generation |
| secondary | pass@1 after best-of-8 |
| **guard** | **non-degeneracy rate** (satisfiable ∧ non-tautological ∧ signature coverage) — a drop here means the model is gaming the proxy |
| reference | Claude / GPT on the same 40 |
| monitor only | CodeBLEU — never an optimization target |

### Phase 2 — make faithfulness measurable — **DONE, and it changed Phase 3**

`scripts/semantic_equiv.py` compares a generated spec to gold with Z3 in both
directions, returning equivalent / stronger / weaker / incomparable, with
compile errors and timeouts kept separate from disagreements. Validated in both
directions before use: gold against itself is equivalent; `{ true }` is weaker;
gold plus a conjunct is stronger; a malformed control returns `compile_error`.

Its results are in section 3. The three that matter here:

1. **Compiling is worth about half what it looks like** — 15–25% correct against
   35–45% compiling.
2. **`weaker` — the direction that makes a verifier miss bugs — is one omission**,
   a dropped frame condition, in 18 of 19 cases across every configuration.
   `scripts/ablate_clause.py` localises it clause-by-clause.
3. **Rejection sampling would make things worse.** Of 16 specs recovered by
   sampling, 2 are correct. A compile-success reward reinforces the 87.5%.

The caveat that limits all of it: gold is a human reading of the PDF, so this
measures agreement with gold, not faithfulness to the text. Tier 1 below —
structural conformance against SCOPE's own parse of the source tables — is still
worth building, because it checks against the document rather than against
another person's reading of it.

**Consequence for Phase 3:** the gate is open and the answer is *no* to the plan
it was gating. Do not train on compile-success. What Phase 3 should become is an
open question for a human, but the measured facts pointing at it are: SFT is
saturated, the remaining defect is a specific and nameable one, and no
inference-time intervention has yet raised correctness by more than 7.5pp.

- [ ] **Tier 1 — structural conformance to the PDF, no LLM judgment required.**
      SCOPE's parser already extracts the source tables (`scope --mode raw`), and
      `training/scope_rule_check_ourcode.py` already consumes that dump. Score:
      - does the generated spec have one implication per failure-condition ID?
      - is every declared output value constrained? (the dangling-output check, which exists)
      - do the referenced helper functions match those in the source conditions?
      - does the footprint match? (needs the normalization work described in `OUR_CODE_RULE_CHECK.md` — currently 59/98 false positives)
- [ ] **Tier 2 — SMT equivalence against gold**, on the 293 commands that have one.
      Four-way verdict: equivalent / stronger / weaker / incomparable. This is the
      real "did it translate correctly" signal.
- [ ] **Tier 3 — a hand-checked calibration set** of 20–30 commands, judged against
      the PDF by a person. Without this, Tiers 1 and 2 have no known error rate.
- [ ] **Then answer the open question:** run Tiers 1+2 over Claude's 95 passing
      outputs. If Claude's faithfulness is high, distillation becomes viable again. If
      it is not, that is a finding worth publishing on its own.

### Phase 3 — gated on Phase 2 shipping

Once faithfulness is measurable the reward can be written correctly:

```
reward = compile_pass ∧ non-degenerate ∧ structural_conformance ≥ threshold
```

RFT, DPO, and GRPO all become safe at that point, and bf16 + flash-attn + vLLM on
H100 make GRPO practical. **The risk was never the hardware; it is the objective
function, and the objective function can be fixed first.**

---

## 5. The decontaminated runs (`sft3-*`) and the bug-finding benchmarks

Every `sft2-*` run was trained on the commands both bug-finding benchmarks score,
so neither benchmark could measure it: the check asks whether the model leaves an
output unconstrained, gold leaves it unconstrained because that IS the gap, and
the model was trained on gold. `dataset_bench` holds all 17 scored commands out at
every version — 249 command examples instead of 293, and a held-out set of 49
containing the old 40 as a strict subset. `sft3-0` and `sft3-2` are `sft2-0` and
`sft2-2` retrained on it, with the V3.1 prompt.

### The 15% data cut costs nothing

| | shared 40 | 49 held out | McNemar |
|---|---|---|---|
| `sft2-0` 4B | 14/40 (35.0%) | — | — |
| `sft3-0` 4B | 16/40 (40.0%) | 18/49 (36.7%) | p = 0.625 |
| `sft2-2` 9B | 16/40 (40.0%) | — | — |
| `sft3-2` 9B | 20/40 (50.0%) | 22/49 (44.9%) | p = 0.344 |

Neither is significant and neither needs to be — the claim is that removing the
benchmark commands did not hurt, and it did not. It also re-exposes how weak this
benchmark is: 36 of 40 commands land identically for `sft2-0`/`sft3-0`, so 4
carry all the information.

The worry that dropping `RMI_RTT_DESTROY` and `RMI_RTT_INIT_RIPAS` would gut the
RTT family was checked and is unfounded: `sft2-0`, which trained on all of them,
already scored **0/9 on RTT with 7 truncated**. RTT was broken before.

### Correctness moved more than compilation (4B)

| | compiles | correct | of compiles |
|---|---|---|---|
| `sft2-0` (40) | 14 (35.0%) | 6 (15.0%) | 43% |
| `sft3-0` (49) | 18 (36.7%) | 10 (20.4%) | **61%** |

The disagreements also changed shape, in the direction that matters:

| | weaker | incomparable | stronger |
|---|---|---|---|
| `sft2-0` | 2 | 3 | 1 |
| `sft3-0` | 1 | 1 | 4 |

`weaker` is the failure compile-success structurally cannot see. Trading it for
`stronger` trades missed bugs for false alarms. With the caveat that a `stronger`
clause which is plausible-but-unstated hides a gap exactly as confabulation does,
so which kind these four are still has to be read per command.

### rule_check_8bugs: the 9B matches gold

| generator | recall (16) | false alarms | missed |
|---|---|---|---|
| gold (control) | 16/16 | 0 | — |
| **`sft3-2` 9B** | **16/16** | 2 (`RMI_RTT_READ_ENTRY.desc`) | none |
| Claude Opus 5 | 14/16 | 2 (`RSI_FEATURES`) | `RMI_RTT_READ_ENTRY` ×2 |
| GPT `gpt-5.6-sol` | 14/16 | 2 (`RSI_FEATURES`) | `RMI_RTT_READ_ENTRY` ×2 |
| `sft3-0` 4B | 11/16 | 0 | VERSION ×4, `RSI_IPA_STATE_GET` |

The 9B's two false alarms are `desc`, which **SCOPE's own labelling patch marks
`FP`** — a checker limitation, not a model error. The SOTA models' `RSI_FEATURES`
alarm is a real miss: the spec defines `value` and they emitted `true`. `sft3-*`
does not, which is the V3.1 prompt correction landing.

**The 4B's two VERSION "misses" are a scoring artifact.** It emitted, for
`RMI_VERSION` on eac5:

```rust
(result.is_Ok() ==> lower == req)
&& (result.is_Ok() ==> higher == RmiInterfaceVersionHighestSupported(new_s))
```

while gold is `{ true }`. The Success-conditions **table** is empty — which is
what SCOPE's ground truth derives from — but the prose above it states the a/b/c
rule in full. The model read the prose and encoded it, so nothing dangles, so the
check does not fire, so it scores as a miss. The 9B and both SOTA models leave it
blank and are credited. **The benchmark rewards reading only the table.**

### Withdrawn: "capable models confabulate, weak ones leave the gap visible"

`BASELINE1_GENERAL_MODEL_COMPARISON.md` reads the `walk_level` result as a
property of capable models. Two experiments say it is not.

**The 9B falsifies the capability version.** It is stronger than the 4B on every
axis measured here, and it leaves `walk_level` unconstrained exactly as the 4B
does. Both fine-tuned models leave it; both general models fill it in. The line
is not capability, it is whether the model was fine-tuned on this task.

**A prompt line falsifies it outright.** `scripts/confab_probe.py` runs two arms
that differ only by one added paragraph, which names no command and no field:

| arm | eac5 | rel0 |
|---|---|---|
| base (`PROMPT_V3_SYSTEM` verbatim) | invents `walk_level as int == RttWalk(old_s, rd, ipa).level` | invents |
| + "do not define outputs the specification leaves undefined" | **leaves it unconstrained** | **leaves it unconstrained** |

The base arm reproduces the published failure on both versions, so the control
holds and the treatment is the only difference. **One paragraph recovers the
missed bug.**

All twelve cells, `walk_level` in the function body:

| generator | eac5 | rel0 |
|---|---|---|
| gold | blank | blank |
| `sft3-0` (4B, fine-tuned) | blank | blank |
| `sft3-2` (9B, fine-tuned) | blank | blank |
| Claude Opus 5, base prompt | **invents** | **invents** |
| Claude Opus 5, + no-invent | blank | blank |
| GPT `gpt-5.6-sol`, base prompt | **invents** | **invents** |

Two variables move independently and only one of them matters. Capability does
not: the 9B is stronger than the 4B on every axis measured here and behaves
identically to it. The prompt does: the same Claude flips from inventing to
blank, on both versions, from one added paragraph that names no command and no
field. Four independent base-arm generations invent it, each with different
syntax, which rules out sampling noise.

So the correct statement is not that capable models are worse detectors. It is
that this task has a convention — *when the document says nothing, say nothing* —
which the fine-tuned models absorbed from ~250 examples and the general models
were never told. That is a better result: it is actionable, and it means the
published SOTA rows understate what a general model does when asked properly.

### verus_rmm is not measuring bug-finding yet

| | eac5 | rel0 |
|---|---|---|
| gold (control) | 4/4 TP, 6/6 FP | 3/3 TP, 6/6 FP |
| `sft3-0` 4B | 0/4, 4 inconclusive | 1/3, 2 inconclusive |
| `sft3-2` 9B | 0/4, 4 inconclusive | — |

`inconclusive` means the generated function does not compile, so the obligation
never runs: `E0061` wrong arity, `E0308` mismatched types, `E0425` unknown
`RMI_SUCCESS`/`walk_top`. Unrepaired Claude sat at 1/4 for the same reason, and
`BENCHMARK_VERUS_RMM.md` showed one Verus-feedback repair round took it to 4/4,
gold parity. **Until `sft3-*` gets that repair pass this column measures Verus
syntax fluency.** `scripts/repair_eval.py` already exists; wiring it to the
benchmark's input format is the next step.
