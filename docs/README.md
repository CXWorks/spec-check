# Documentation

Working notes and findings for the `spec-check` project.

| Document | What it covers |
|---|---|
| [`data-leakage.md`](data-leakage.md) | Eval/train overlap in the alp14 benchmark — evidence, root cause, measured impact, and how the fix was chosen |
| [`dataset.md`](dataset.md) | The leak-free dataset that replaced it: why the split looks like this, what it contains, and how to evaluate against it |

## Current state

The benchmark is **40 held-out alp14 commands** (`dataset_clean/splits.json`), not
all 98. Numbers scored on all 98 for the fine-tuned Qwen models are inflated and not
comparable — see `data-leakage.md`.

## Conventions

- Every finding document states **what was verified** and **what was assumed**, separately.
- Claims backed by a script include the script, so anyone can re-run it.
- Numbers are quoted with the artifact they came from (file path or command), not from memory.
