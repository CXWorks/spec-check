# alp14, all 98 commands, five arms (2026-09-12)

`sft3-2` (Qwen3.5-9B + LoRA), weights unchanged. Generated on research-secure-05,
five arms in parallel on one H100 each, greedy decode (`do_sample=False`).

    specs/<arm>/alp14/<command>.rs    the generated specs, 98 per arm
    equiv-<arm>.json                  Z3 verdicts vs training-dataset/specs/alp14

Arms: `nopre` (no preamble), `tail` (200-line tail), `sel` (selected), and each
preamble mode again with `--repair-rounds 3`.

Both repair arms are run because the preamble mode behind the historical
"dict + 3 repair rounds" row is not recorded anywhere in the repo -- running both
was cheaper than guessing which one it had been.

**The 98 must never be pooled into one number.** 49 are held out and 49 are
training commands whose alp14 text and gold the model saw verbatim; pooling
inflates correctness 1.5-1.9x and collapses four distinct configurations onto an
identical 52/98. See docs/9B结果-pass率与找bug.md for the split tables.

Checked in rather than left on HF: the artifacts behind this project's headline
numbers were unreachable twice in one day behind an expired token.
