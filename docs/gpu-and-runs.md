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
| `sft2-4` | `Qwen/Qwen3.5-9B` | bf16 | full fine-tune | new | capacity × method | ⏳ running |

All: `dataset_clean` (1310/91/40), 3 epochs, 8×H100 single node, every epoch
checkpointed to `jisenli/spec-check-ckpt/<run>/checkpoint-*` plus `final`.
`sft2-0` took 486s wall clock.

**The 9B runs use a different dependency stack** (torch 2.9.1 + transformers
5.15, attention on sdpa rather than flash-attn) because `qwen3_5` is unknown to
transformers 4.x. The 4B/9B comparison carries that caveat.

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
