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

### Results (40 held-out commands)

| Run | | Verus pass | non-degenerate |
|---|---|---|---|
| **gold** | reference | **33/40 (82.5%)** | — |
| `sft2-1` | 4B fp16 LoRA | 18/40 (45.0%) | 39/40 |
| `sft2-0` | 4B bf16 LoRA | **14/40 (35.0%)** | 39/40 |
| `sft2-0` @ epoch 1 | 4B bf16 LoRA | 12/40 (30.0%) | 40/40 |
| `sft2-3` | 4B bf16 full FT | 11/40 (27.5%) | 39/40 |

The epoch curve is 30.0% (epoch 1) → 35.0% (epoch 3); epoch 2 was not measured
because the storage quota stopped `eval2-ep` after its first checkpoint. A 5pp
move across two epochs is well inside the ±22pp this eval set can resolve, so it
is not yet evidence that the extra epochs help.

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

### Every pass rate below is understated: the decode budget was too small

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

It hit the configurations **unequally**, so it is a confound rather than a
uniform penalty. `sft2-3` was truncated twice as often as `sft2-0`, which is
enough on its own to withdraw "full fine-tuning is worse" — that comparison has
no support left.

Half the commands that never passed under any run (7 of the 14 where gold does
compile) were simply never allowed to finish: `RMI_REALM_DESTROY`,
`RMI_RTT_AUX_MAP_PROTECTED`, `RMI_RTT_FOLD`, `RMI_RTT_MAP_UNPROTECTED`,
`RMI_RTT_READ_ENTRY`, `RMI_VDEV_DESTROY`, `RMI_VDEV_P2P_BIND`. Genuinely-hard
commands number 7, not 14.

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

### Statistical power: why none of the above is yet a result

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

### Phase 2 — make faithfulness measurable (critical path)

The project currently has **no metric for its actual goal**. It can measure "compiles";
it cannot measure "faithful to the PDF". Until that exists, Phase 3 is unsafe.

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
