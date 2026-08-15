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

**Principle: names on the cluster reveal nothing about the project. The mapping from
name to experiment lives only in this repo.**

Names must not contain: `spec`, `verus`, `rmm`, `arm`, `cca`, or any personal
username. They must be lowercase (RFC 1123 — `DE2-RL-test-x` is not a legal object
name).

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

Do **not** reuse another user's secrets (`yjian-hf-token` etc.). Qwen3 checkpoints are
public, so no HF token is needed to pull them; checkpoints stay on the PVC for now.

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

| Run | Base model | Precision | Method | Dataset | Result |
|---|---|---|---|---|---|
| `env1` | — | — | interactive env validation | — | pending |
| `sft2-0` | Qwen3-4B | **bf16** | LoRA r=16, 2 ep | `dataset_clean` | pending |
| `sft2-1` | Qwen3-4B | fp16 | LoRA r=16, 2 ep | `dataset_clean` | pending — precision control |
| `sft2-2` | ~9B | bf16 | LoRA r=16, 2 ep | `dataset_clean` | pending |
| `sft2-3` | ~14B | bf16 | LoRA r=16, 2 ep | `dataset_clean` | pending |
| `sft2-4` | Qwen3-4B | bf16 | full fine-tune, 2 ep | `dataset_clean` | pending |

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

- [ ] **`env1`** — interactive pod: torch, deps, `kubectl cp` the data, pull the base
      model, run Verus once. Do not launch training Jobs until this passes.
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
