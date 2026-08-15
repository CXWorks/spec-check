# Data Leakage: the alp14 eval set overlaps the training set

**Status:** confirmed, not yet fixed
**Found:** 2026-08-15
**Affects:** every CodeBLEU number reported for the fine-tuned Qwen models (`item_split_e2_best`, `item_split_v3_e2_best`, `item_split_v4_best`)
**Does not affect:** Verus pass rates, the Claude/GPT prompt-engineering tracks, or the spec bugs found so far

---

## TL;DR

**79 of the 98 alp14 commands used as the evaluation set have their gold answer present, byte-for-byte, inside `dataset/train.jsonl`.** The validation split has the same problem (81 of 99 alp13 commands).

The cause is that the repo contains **two incompatible dataset-split schemes** and the training script and the evaluation loader each use a different one.

The measured effect on CodeBLEU turned out to be **small** (+0.0196 between the leaked and clean subsets), so this does *not* overturn the project's conclusions — but it is a real methodological flaw that should be fixed before the next retrain, and every CodeBLEU number should carry a caveat until then.

---

## 1. What the evaluation actually uses

`prompt_engineering/dataset_loader.py` splits **by spec version**:

```python
TRAIN_VERSIONS = ["eac5", "rel0", "alp11", "alp12"]
VAL_VERSIONS   = ["alp13"]
TEST_VERSIONS  = ["alp14"]
```

`run_qwen_v3.py` and `prompt_engineering_v3.py` both call `load_dataset(split="test")`, which returns **all 98 alp14 commands**. The gold answer each generated spec is scored against is `training-dataset/specs/alp14/<cmd>_spec.rs`.

## 2. What the training set actually contains

`training/build_dataset.py` splits **by item name** — it collects every command name seen across all six versions, shuffles with a fixed seed, and cuts 80/10/10. Then:

```python
# training/build_dataset.py:346
if cmd in cmd_train:
    train_exs.append(ex)                            # no version guard
elif cmd in cmd_val  and version == EVAL_VERSION:   # version guard
    val_exs.append(ex)
elif cmd in cmd_test and version == EVAL_VERSION:   # version guard
    test_exs.append(ex)
```

Note the asymmetry: the `val` and `test` branches are restricted to `EVAL_VERSION` (`alp14`), but the `train` branch is not. So when a command name falls in `cmd_train`, **all six of its per-version instances go into the training set — including the alp14 one**.

The item-based split is internally sound: train/val/test command names do not overlap (verified: intersection is empty). The problem is only that the evaluation loader uses a *different* partition of the same data.

## 3. Evidence

```
eval set (dataset_loader, split="test")     : 98 alp14 commands
alp14 command examples inside train.jsonl   : 79
overlap                                     : 79 / 98  = 80.6%
gold answers byte-identical after strip()   : 79 / 79  = 100%
```

Concrete case — `RMI_DATA_CREATE`:

- what the model was trained to emit: the `assistant` message of the alp14 `RMI_DATA_CREATE` example in `train.jsonl`
- what the evaluation scores against: `training-dataset/specs/alp14/rmi_data_create_spec.rs`
- **identical, 4984 characters, no difference**

Same check on the validation split:

```
val (alp13) : eval=99  in train.jsonl=81  overlap=81 (81.8%)  byte-identical=81
test (alp14): eval=98  in train.jsonl=79  overlap=79 (80.6%)  byte-identical=79
```

Breakdown of `train.jsonl` (1268 examples):

| version | examples | | kind | examples |
|---|---|---|---|---|
| eac5 | 123 | | command | 381 |
| rel0 | 125 | | type_definition | 257 |
| alp11 | 248 | | helper_stub | 630 |
| alp12 | 254 | | | |
| alp13 | 257 | | | |
| **alp14** | **261** | | | |

Of the 261 alp14 examples in the training set: 79 command, 56 type, 126 helper.

## 4. Measured impact

Splitting the 98 evaluated commands by whether the model saw their gold answer during training
(scores from `results/ab_test_qwen_v3retrained/v3_qwen/alp14/*/meta.json`, the run that reported
CodeBLEU 0.7627):

| subset | n | mean CodeBLEU |
|---|---|---|
| leaked (gold answer seen in training) | 79 | **0.7665** |
| clean (never seen) | 19 | **0.7469** |
| all | 98 | 0.7627 |
| **gap** | | **+0.0196** |

**The model is not memorizing.** If it were, the leaked subset would score near 1.0; it scores barely
two points above the clean subset. Plausible reasons: only 2 training epochs, LoRA rank 16 on a 4B
model, and gold outputs several thousand characters long. The same command appearing across six
versions with slightly different text probably acts more like augmentation than repetition.

The 19 clean commands, individually:

```
0.3212  RSI_VERSION                      0.7659  RMI_RTT_DESTROY
0.4657  RMI_VERSION                      0.7969  RSI_VDEV_VALIDATE_MAPPING
0.5855  RMI_VDEV_GET_STATE               0.7971  RMI_VDEV_AUX_COUNT
0.6161  RMI_PDEV_GET_STATE               0.8286  RMI_VDEV_P2P_UNBIND
0.6164  PSCI_CPU_SUSPEND                 0.8782  RMI_PDEV_STOP
0.6205  RMI_FEATURES                     0.8844  RSI_MEM_GET_PERM_VALUE
0.6530  RSI_ATTESTATION_TOKEN_CONTINUE   0.9602  RMI_PDEV_DESTROY
0.6803  RMI_PDEV_IDE_KEY_REFRESH         1.0000  RMI_VDEV_GET_INTERFACE_REPORT
0.7209  PSCI_FEATURES                    1.0000  RMI_MEC_SET_SHARED
                                         1.0000  RMI_GRANULE_DELEGATE
```

Note that the clean subset happens to contain both degenerate short-target commands
(`RSI_VERSION`, `RMI_VERSION`), whose gold answer is essentially `true` and on which CodeBLEU is
known to be very noisy. Excluding them narrows the gap further, possibly reversing its sign — so
0.0196 should be read as an upper bound on the leakage effect, not a point estimate.

## 5. What is and is not affected

**Affected:**

- All CodeBLEU numbers for the fine-tuned Qwen models on alp14 (0.639 / 0.7627 / 0.8389) — inflated by
  roughly 0.02, and methodologically invalid as reported regardless of magnitude.
- Baseline 1's eac5/rel0 CodeBLEU (0.8139 / 0.8056) — worse, since those two versions are in
  `TRAIN_VERSIONS` entirely. `BASELINE1_SCOPE_REPRODUCTION.md` already flags this.

**Not affected:**

- **Verus pass rates.** A compiler either accepts the code or it doesn't; having seen the gold answer
  during training doesn't change that verdict. The 48.0% / 66.33% / 96.94% three-way comparison stands.
- **Checkpoint selection during training.** `train.py --val dataset/val.jsonl` uses the *item-split*
  val file (31 examples, no name overlap with train), not `dataset_loader`'s alp13. So
  `load_best_model_at_end` was not contaminated.
- **The Claude and GPT tracks.** Neither model ever saw any gold answer; leakage is a fine-tuning-only
  issue.
- **The spec bugs found so far.** Those come from `ensures false` scans, not from similarity scoring.

**Explicitly not the explanation for the CodeBLEU-vs-Verus rank reversal.** An earlier hypothesis was
that the fine-tuned model's high CodeBLEU was mostly memorization, which would explain why it loses on
Verus pass rate. Section 4 rules that out. The explanation in
[`BASELINE2_GENERAL_MODEL_COMPARISON.md`](../BASELINE2_GENERAL_MODEL_COMPARISON.md) stands: fine-tuning
teaches the oracle's *style* (naming, structure, type aliases), which is what a similarity metric
rewards, and that requires no exposure to the specific answers. GPT landing in the middle of both
rankings — with zero exposure to gold answers — independently supports this.

## 6. Fix — applied

**Resolved in `training/build_dataset.py`. The new dataset is specified in
[`dataset.md`](dataset.md).** Summary of how the fix was arrived at, including two
approaches that were considered and rejected:

**Rejected — exclude alp13/alp14 from training by version.** This was the first proposal and it is
wrong. Measuring each version's gold specs against alp14's shows the six versions are successive
drafts of one document, not independent corpora:

| version | shared commands | byte-identical to alp14 | mean similarity |
|---|---|---|---|
| eac5 | 41 | 7 | 0.727 |
| rel0 | 41 | 8 | 0.753 |
| alp11 | 79 | 38 | 0.912 |
| alp12 | 79 | 48 | 0.932 |
| alp13 | 93 | **67** | **0.957** |

Excluding alp13 and alp14 still leaves **36 of the 98** alp14 commands with a byte-identical copy in
alp11/alp12, at a cost of 518 training examples. **Version is not a valid split dimension here.**

(The stated motive for excluding alp13 — that it is `dataset_loader`'s val split — was also weak:
nothing in the repo calls `load_dataset(split="val")`. alp13-as-val is dead code left from the
older cascaded scheme.)

**Rejected — evaluate on the existing `dataset/test.jsonl`.** Worth recording that
`build_dataset.py`'s item-name split was *already* leak-free: train/val/test item names have zero
overlap, and none of `test.jsonl`'s 11 commands appears in training at any version. **The bug was
entirely on the evaluation side** — `dataset_loader.py` ignored `test.jsonl` and re-partitioned by
version. So switching the evaluator to `test.jsonl` would have cost zero training data. It was
rejected only because 11 commands is too small a benchmark to compare three models on.

**Applied — same name-based principle, larger held-out set.** Commands are split train/test by name
(40 held out, drawn from alp14, removed from training at every version); validation is drawn from
types and helpers, which the benchmark does not score, so no command names are spent on it.

```
command training examples : 381 → 293  (−23%)
held-out benchmark        : 40 commands, verified zero leakage
```

`assert_no_leakage()` now runs on every build and exits non-zero if any item name lands in more than
one split — the guard this script previously lacked.

**Still open:** `prompt_engineering/dataset_loader.py` continues to split by version, and
`load_dataset(split="test")` still returns all 98 alp14 commands. Every evaluation script must filter
to `splits.json["command_test"]`; see [`dataset.md`](dataset.md#how-to-evaluate). Fixing the loader
itself would be better than relying on each caller to remember.

## 7. Reproducing this analysis

From the repo root, with `training-dataset/{sections,specs,dataset}` present:

```bash
python3 - <<'PY'
import json, sys
sys.path.insert(0, 'prompt_engineering')
from dataset_loader import load_dataset

train = [json.loads(l) for l in open('training-dataset/dataset/train.jsonl') if l.strip()]

for split, ver in [('val', 'alp13'), ('test', 'alp14')]:
    ev = {s.command: s.oracle for s in load_dataset(versions=[ver])}
    tr = {e['metadata']['command']: e['messages'][-1]['content']
          for e in train
          if e['metadata'].get('version') == ver
          and e['metadata'].get('kind') == 'command'}
    overlap = set(ev) & set(tr)
    identical = sum(1 for c in overlap if tr[c].strip() == ev[c].strip())
    print(f'{split} ({ver}): eval={len(ev)} in-train={len(tr)} '
          f'overlap={len(overlap)} ({100*len(overlap)/len(ev):.1f}%) identical={identical}')
PY
```

Expected output:

```
val (alp13): eval=99 in-train=81 overlap=81 (81.8%) identical=81
test (alp14): eval=98 in-train=79 overlap=79 (80.6%) identical=79
```

## 8. Caveats on this analysis

1. **The impact numbers in section 4 are from the v3 run** (`item_split_v3_e2_best`, CodeBLEU 0.7627),
   because that is the only per-command result set committed to the repo. The v4 results
   (`results/ab_test_qwen_v4/`, CodeBLEU 0.8389) are referenced by
   `BASELINE2_GENERAL_MODEL_COMPARISON.md` but are not in the repo, so the leaked/clean split has not
   been recomputed for v4. The direction should hold; the exact gap should be re-measured on the
   server.

2. **The `train.jsonl` inspected here was last committed 2026-04-29**, before Iteration 7 regenerated
   the dataset and retrained. It is therefore not literally the file v4 was trained on. However
   `training/build_dataset.py`'s split logic has not changed since 2026-07-17, `SPLIT_SEED = 42` is
   fixed, and `split_names()` sorts before shuffling — so regenerating produces an identical partition.
   The leakage is present in v4's training data as well. Confirming this directly on the server is a
   one-command check and worth doing.

3. **The overlap check covers commands only.** Types and helpers were not checked against an
   evaluation set because the current benchmark is command-only, but 56 alp14 type and 126 alp14
   helper examples are also in the training set and would leak into any future type/helper benchmark.
