# 9B on DEN0137 2.0 BET3 (2026-09-15)

`sft3-2` (Qwen3.5-9B + LoRA), 121 commands, no gold exists for this document.
Two arms: `sel` (selected preamble) and `tail` (200-line tail). Greedy decode.

Run to answer one question: does the model, reading the same document, leave the
same outputs unconstrained as the document-only scan said were undefined? It
does, on all but two (see docs/2.0bet3-进展与计划.md).

Compile rate here is 10/121 and 11/121 and is NOT a model-quality number. The
preamble is alp14's stand-in and 2.0 renamed RmiCommandReturnCode to RmiResult
(104 occurrences) among 71 missing types, so missing_symbol dominates the
failures. The Z3 sweep is omitted for the same reason. The dangling-output check
never invokes Verus and is unaffected.
