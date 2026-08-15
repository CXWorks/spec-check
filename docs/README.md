# Documentation

Working notes and findings for the `spec-check` project.

| Document | What it covers |
|---|---|
| [`data-leakage.md`](data-leakage.md) | Eval/train overlap in the alp14 benchmark — evidence, root cause, measured impact, and the fix to apply before the next retrain |

## Conventions

- Every finding document states **what was verified** and **what was assumed**, separately.
- Claims backed by a script include the script, so anyone can re-run it.
- Numbers are quoted with the artifact they came from (file path or command), not from memory.
