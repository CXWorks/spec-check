# Prompt Engineering

This folder contains the prompt-engineering pipeline for generating Verus command
specifications with Claude, plus evaluation scripts for comparing raw vs
verusfmt-normalized outputs.

## Methodology at a glance

- **Approach:** zero-shot / few-shot prompt engineering via Claude API.
- **Prompt variants:** 5 templates (`V0`..`V4`), where `V2` is few-shot and the
  others are zero-shot variants.
- **Target split for current experiments:** `alp14` test split.
- **Scale:** 98 command specs (one sample per command in `alp14`).

## Data preprocessing and inputs

For each command sample, the pipeline builds a triplet from
`training-dataset/`:

1. **Spec text input** from
  `sections/{version}/{COMMAND}_command.txt`
2. **Shared context (preamble)** from
  `specs/{version}/preamble.rs` (tail segment)
3. **Gold oracle** from
  `specs/{version}/{command_lower}_spec.rs`

Current reported prompt-engineering results in this folder use
`alp14` only (98 commands).

## Claude API workflow

```text
┌──────────────────────────────────────────────────────────────┐
│ Load alp14 dataset                                           │
│ (section text + preamble + oracle)                           │
└──────────────────────────────────────────────────────────────┘
               |
               v
┌──────────────────────────────────────────────────────────────┐
│ Prompt formatter (V0 / V1 / V2 / V3 / V4)                   │
└──────────────────────────────────────────────────────────────┘
               |
               v
┌──────────────────────────────────────────────────────────────┐
│ Claude Haiku API generates spec                              │
└──────────────────────────────────────────────────────────────┘
               |
               v
┌──────────────────────────────────────────────────────────────┐
│ Save raw output: generated.raw.rs                            │
└──────────────────────────────────────────────────────────────┘
        |                                  |
        |                                  v
        |               ┌──────────────────────────────────────────────┐
        |               │ verusfmt --verus-only normalization          │
        |               └──────────────────────────────────────────────┘
        |                                  |
        |                                  v
        |               ┌──────────────────────────────────────────────┐
        |               │ Save formatted output: generated.formatted.rs│
        |               └──────────────────────────────────────────────┘
        |                                  |
        └───────────────┬──────────────────┘
                v
┌──────────────────────────────────────────────────────────────┐
│ CodeBLEU evaluation (raw + formatted vs oracle)             │
└──────────────────────────────────────────────────────────────┘
               |
               v
┌──────────────────────────────────────────────────────────────┐
│ Aggregate + per-command report                              │
└──────────────────────────────────────────────────────────────┘
               |
               v
┌──────────────────────────────────────────────────────────────┐
│ results/ab_test/{variant}/{version}/{command}/              │
└──────────────────────────────────────────────────────────────┘
```

## Scripts overview

### `dataset_loader.py`

- Loads dataset samples from `training-dataset/`.
- Builds each sample with:
  - command section text (`sections/{version}/{cmd}_command.txt`)
  - shared preamble context (`specs/{version}/preamble.rs`)
  - oracle spec (`specs/{version}/{cmd}_spec.rs`)
- Supports split-based loading (`train`, `val`, `test`).

### `prompt_engineering.py`

- Main generation and A/B testing entrypoint.
- Defines 5 prompt variants (`V0` to `V4`).
- Calls Claude API to generate spec candidates.
- Optionally normalizes generated/oracle code via `verusfmt`.
- Computes Best@k CodeBLEU metrics and ranks prompt variants.
- Can save artifacts under `results/ab_test/` (and batch mode under
  `results/batch/`).

## Evaluation protocol (raw vs formatted)

For each generated candidate, we evaluate against the oracle with CodeBLEU for
Rust.

- **Sub-metrics:** `ngram_match`, `weighted_ngram_match`, `syntax_match`,
  `dataflow_match`
- **Weights:** `(0.25, 0.25, 0.25, 0.25)`

Two output variants are evaluated:

1. **Unformatted (raw):** direct Claude output
2. **Formatted:** output normalized with `verusfmt --verus-only`

In both cases, oracle code is normalized consistently before scoring when
formatted evaluation is used.

## How final scores are computed

### Single-sample setting (`n_samples=1`)

For each command, compute one CodeBLEU score, then average across commands.

### Multi-sample setting (`n_samples>1`, current default experiment uses 5)

We report **Best@k**. For each command $i$, let $s_{i,j}$ be the CodeBLEU score
of sample $j$. Then:

$$
\mathrm{Best@k} = \frac{1}{N}\sum_{i=1}^{N}\max_{1\le j\le k}(s_{i,j}),
$$

where $N=98$ for `alp14`.

So, this is **not** the average of all samples directly; it is the average of
the per-command best score among the first $k$ samples.

## Comparability note with `training/` results

When comparing prompt-engineering scores here with fine-tuning scores in
`training/STATUS.md`:

- Ensure both use the same oracle normalization policy.
- `0.637` in training docs is the Round-1 baseline re-evaluated against
  reformatted gold (for comparability), while `0.632` is an earlier report on
  unformatted gold.
- `0.416` is Round-2 (verusfmt-formatted training-data) model score on the same
  reformatted-gold comparison table.

This distinction matters when reasoning about whether changes come from training
data format, model generalization, or evaluation setup.

### `eval_codebleu.py`

- Evaluates two prebuilt files directly (baseline vs fmt), for example:
  - `alp14_generated.rs`
  - `alp14_generated_fmt.rs`
- Parses generated command specs and compares against gold specs from a target
  directory.
- Prints aggregate metrics and optional per-command best/worst lists.

### `eval_results_codebleu.py`

- Evaluates already-saved experiment artifacts under `results/ab_test/`.
- For each prompt variant, compares:
  - `generated.raw.rs` (unformatted)
  - `generated.formatted.rs` (verusfmt formatted)
- Reports:
  - average raw/formatted CodeBLEU
  - improvement delta (`formatted - raw`)
  - per-command best/worst improvements

## Current result structure expected

- `results/ab_test/{variant}/{version}/{command}/generated.raw.rs`
- `results/ab_test/{variant}/{version}/{command}/generated.formatted.rs`
- `results/ab_test/{variant}/{version}/{command}/oracle.raw.rs`
- `results/ab_test/{variant}/{version}/{command}/oracle.formatted.rs`

## How to run

Run aggregate raw-vs-formatted evaluation on existing artifacts:

```bash
python3 prompt_engineering/eval_results_codebleu.py
```

Optional detailed per-command output:

```bash
python3 prompt_engineering/eval_results_codebleu.py --all-commands
```

Typical generation + save-results flow:

```bash
python3 prompt_engineering/prompt_engineering.py --limit 98 --n-samples 5 --save-results

# resume from partial runs (recommended for long API batches)
python3 prompt_engineering/prompt_engineering.py --limit 98 --n-samples 5 --save-results --resume
```

## Notes

- `results/` is treated as experiment artifacts and should not be committed.
- Use `--save-results` to refresh local artifacts for evaluation.
- `prompt_engineering.py` uses `.env`/environment variable `ANTHROPIC_API_KEY`
  for Claude API authentication.
