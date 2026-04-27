# Prompt Engineering: Recent Updates

## What was added

- Added `eval_results_codebleu.py` to evaluate existing artifacts under `results/ab_test/`.
- The script compares:
  - `generated.raw.rs` (unformatted)
  - `generated.formatted.rs` (verusfmt formatted)
- It reports aggregate and per-command CodeBLEU deltas.

## Current result structure expected

- `results/ab_test/{variant}/{version}/{command}/generated.raw.rs`
- `results/ab_test/{variant}/{version}/{command}/generated.formatted.rs`
- `results/ab_test/{variant}/{version}/{command}/oracle.raw.rs`
- `results/ab_test/{variant}/{version}/{command}/oracle.formatted.rs`

## How to run

```bash
python3 prompt_engineering/eval_results_codebleu.py
```

Optional detailed output:

```bash
python3 prompt_engineering/eval_results_codebleu.py --all-commands
```

## Notes

- `results/` is treated as experiment artifacts and should not be committed.
- Use `--save-results` in generation/evaluation workflows to refresh artifacts locally.
