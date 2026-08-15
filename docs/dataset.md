# The leak-free dataset (`dataset_clean/`)

Built by `training/build_dataset.py`. Replaces the split described in
[`data-leakage.md`](data-leakage.md), which put 79 of the 98 evaluated commands'
gold answers into the training set.

**Not committed.** The `.jsonl` files are gitignored — they are 9.4 MB and fully
determined by `splits.json` plus the source data. Regenerate them anywhere with:

```bash
python3 training/build_dataset.py    # needs Python 3.10+; the repo default may be 3.9
```

`splits.json` **is** tracked, and is the authoritative record of which commands
are held out.

---

## Why the split looks like this

The six spec versions are successive drafts of **one document**, not six
independent corpora. Measured against alp14's gold specs:

| version | shared commands | byte-identical | >95% similar | mean similarity |
|---|---|---|---|---|
| eac5 | 41 | 7 | 3 | 0.727 |
| rel0 | 41 | 8 | 3 | 0.753 |
| alp11 | 79 | 38 | 18 | 0.912 |
| alp12 | 79 | 48 | 13 | 0.932 |
| alp13 | 93 | **67** | 14 | **0.957** |

So **version is not a valid split dimension.** Holding out alp14 while training on
alp13 leaks almost everything; holding out alp13 *and* alp14 still leaves 36 of the
98 alp14 commands with a byte-identical copy in alp11/alp12.

The only sound dimension is the **item name**: a command name held out for testing
is removed from training at *every* version.

## Why only 40 test commands

The command-name space is small — **121 unique names across all six versions**, of
which alp14 has 98. Only 23 names exist solely in older versions (deprecated
`RSI_RDEV_*`, `RMI_P2P_*`, `RMI_DEV_MEM_*` — the superseded device-passthrough API).

That makes "hold out all 98 alp14 commands" infeasible: training would be left with
23 names → 45 command examples (−88%), all of them deprecated commands, used to
predict current ones.

Each name held out costs ~3.5 training examples (a name appears in 3.9 versions on
average):

| test size | train command names | train command examples | vs. 381 |
|---|---|---|---|
| 98 | 23 | 45 | −88% ❌ |
| 70 | 41 | 129 | −66% |
| 50 | 71 | 245 | −36% |
| **40** | **81** | **293** | **−23%** |
| 30 | 91 | 345 | −9% |

40 was chosen as the point where the benchmark is still large enough to separate
models (the live gap on Verus pass rate is 48% vs 96.94%, far wider than the ±8%
standard error of a 40-sample binomial) while keeping three quarters of the command
training data.

## Why validation comes from types and helpers

The benchmark scores **commands only**. Types and helpers are therefore free to use
for the training-time `eval_loss` signal, which avoids spending scarce command names
on validation. A 90/10 name split gives 91 validation examples.

Caveat: `eval_loss` then reflects type/helper fit, not command fit. It is a
"did the run blow up" monitor, not a basis for checkpoint selection. Since
`RESULTS_V3.md` Iteration 3 already established **2 epochs** as optimal, train with
a fixed epoch count rather than relying on `load_best_model_at_end`.

## Contents

```
training-dataset/dataset_clean/
├── train.jsonl    1310 examples
├── val.jsonl        91 examples   (types + helpers only)
├── test.jsonl       40 examples   (commands, alp14)
└── splits.json                    (tracked; the name lists + seed)
```

| kind | old (leaky) | new (clean) | change |
|---|---|---|---|
| command | 381 | **293** | **−23%** |
| type_definition | 257 | 291 | +13% |
| helper_stub | 630 | 726 | +15% |
| **train total** | **1268** | **1310** | **+3%** |

Total training data went slightly *up*, because the old item split also held out
10%+10% of type and helper names for splits that were never evaluated; those are now
reclaimed. **Do not read the +3% as "no data was lost."** The task is command
generation, and command examples dropped 23%. If the retrained model scores worse,
check this first before touching the prompt.

## Verification

Built-in: `build_dataset.py` calls `assert_no_leakage()` and exits non-zero if any
item name lands in more than one split. This guard is what the previous version
lacked.

Independently verified on the generated files:

```
test command names appearing anywhere in train (any version) : 0 / 40
test gold answers appearing verbatim in train                : 0 / 40
val names overlapping train                                  : 0
```

58 alp14 command examples remain in `train.jsonl`. That is correct and not leakage:
those are train-split names, and their alp14 instances are legitimate training data —
none of them is scored.

## How to evaluate

**Do not evaluate on all 98 alp14 commands.** `prompt_engineering/dataset_loader.py`
still splits by version and `load_dataset(split="test")` still returns all 98; using
it reintroduces the leak for the fine-tuned models.

Score only the 40 commands in `splits.json["command_test"]`:

```python
import json
test_cmds = set(json.load(open("training-dataset/dataset_clean/splits.json"))["command_test"])
dataset = [s for s in load_dataset(split="test") if s.command in test_cmds]
```

For the Claude and GPT tracks this restriction is not required for correctness —
neither model ever saw a gold answer — but **use the same 40 anyway**, otherwise the
three-way comparison in `BASELINE2_GENERAL_MODEL_COMPARISON.md` is not measuring the
same thing across models.

## Held-out commands (40)

```
PSCI_CPU_OFF                     RMI_RTT_AUX_MAP_UNPROTECTED
RMI_DATA_CREATE                  RMI_RTT_AUX_UNMAP_UNPROTECTED
RMI_DATA_CREATE_UNKNOWN          RMI_RTT_CREATE
RMI_DATA_DESTROY                 RMI_RTT_FOLD
RMI_GRANULE_DELEGATE             RMI_RTT_MAP_UNPROTECTED
RMI_MEC_SET_SHARED               RMI_RTT_READ_ENTRY
RMI_PDEV_IDE_RESET               RMI_RTT_SET_RIPAS
RMI_PDEV_SET_PUBKEY              RMI_RTT_SET_S2AP
RMI_PSMMU_MSI_CONFIG             RMI_VDEV_AUX_COUNT
RMI_REALM_CREATE                 RMI_VDEV_DESTROY
RMI_REALM_DESTROY                RMI_VDEV_GET_INTERFACE_REPORT
RMI_REC_DESTROY                  RMI_VDEV_GET_STATE
RMI_REC_ENTER                    RMI_VDEV_LOCK
RMI_RTT_AUX_MAP_PROTECTED        RMI_VDEV_P2P_BIND
RMI_VDEV_START                   RSI_MEASUREMENT_READ
RMI_VDEV_UNLOCK                  RSI_MEM_SET_PERM_INDEX
RMI_VSMMU_CREATE                 RSI_REALM_CONFIG
RMI_VSMMU_MAP                    RSI_VDEV_GET_INFO
RSI_ATTESTATION_TOKEN_CONTINUE   RSI_VSMMU_ACTIVATE
RSI_IPA_STATE_GET                RSI_VSMMU_GET_INFO
```

Two of these — `RMI_RTT_READ_ENTRY` and `RSI_ATTESTATION_TOKEN_CONTINUE` — are
commands with known spec bugs (SCOPE's dangling-output finding and one of the five
machine-verified inconsistencies respectively). Having them in the held-out set is
useful: it makes bug-recall on them a genuine generalization result.

## Changing the split

Edit `CMD_TEST_SIZE` in `training/build_dataset.py` and rerun. `SPLIT_SEED = 42` is
fixed and `split_two()` sorts before shuffling, so the same size always yields the
same split. Note that changing the size **invalidates comparability** with any run
scored on the current 40 — record which split each result used.
