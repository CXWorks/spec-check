# SCOPE rule-mode dangling-output benchmark (eac5 + rel0)

Self-contained package: ground truth, model predictions, and a scorer that
reproduces every number in [`REPORT.md`](REPORT.md) using only files in this
directory. Copy it anywhere and `python3 score.py` still works.

```
ground_truth.json          16 positive items (8 commands x 2 spec versions), from
                           SCOPE's own `XXX: TP` labels
predictions/
  _gold_oracle/            hand-written reference annotations (control; scores 16/16)
  gpt56sol/                GPT gpt-5.6-sol (high), via `codex exec`
  claude_opus5/            Claude Opus 5 (high), via `claude -p`
    {eac5,rel0}/{command}.rs      one generated `pub open spec fn` per command
scope_tables/
  {version}_raw.txt        SCOPE's parsed output/footprint tables (`--mode raw`)
  {version}_rule_labelled.txt   SCOPE's own rule output with its TP/FP labels applied
score.py                   the scorer
scores.json                scorer output for the models shipped here
REPORT.md                  results, analysis, controls, limitations
```

## The task

For each command, a generator produces one Verus `pub open spec fn`. An item counts
as **detected** when the generated function's *body* never mentions a declared
output's name — SCOPE's dangling-output check. The 16 positives are the commands
SCOPE's own labelling patch marks `XXX: TP`.

Scoring reports command-level recall, field-level recall, and false alarms
(flagged outputs that are not in the ground truth).

## Usage

```bash
python3 score.py                        # every model in predictions/
python3 score.py --model _gold_oracle   # the control: expect 16/16, 0 false alarms
python3 score.py --json-out scores.json
```

To score a new generator, drop its output at
`predictions/<name>/{eac5,rel0}/<command_lowercase>.rs` — one complete
`pub open spec fn` per file — and re-run. Commands with no file are reported as
`commands_missing` rather than silently counted as misses.

## Headline

| Generator | Recall | False alarms |
|---|---|---|
| Gold oracle (control) | 16/16 | 0 |
| GPT `gpt-5.6-sol` (high) | 14/16 | 2 |
| Claude Opus 5 (high) | 14/16 | 2 |

Both models miss the same item (`RMI_RTT_READ_ENTRY`/`walk_level`) for the same
reason — they invent a definition for an output the spec never defines, which
masks the gap. See [`REPORT.md`](REPORT.md).

## Provenance

Ground truth is re-derived from SCOPE rather than transcribed: `scope --mode rule`
+ `patch/{version}_rule.patch`, parsed by `training/parse_scope_rule_output.py`
into `training/scope_ground_truth_eac5_rel0.json`. The `.txt` inputs given to SCOPE
are byte-identical to fresh `pdftotext -layout` output from the ARM PDFs.
Generation used `run_baseline1_general.py` (V3 prompt + preamble + RAG top-3, n=1).
