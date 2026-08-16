#!/usr/bin/env python3
"""Join a generator's dangling-output findings against SCOPE's own labelled
rule-mode true positives, and produce a first-pass triage for anything flagged
that SCOPE did not list.

The dangling-output check answers exactly one question: "does the generated spec
function ever mention this declared output?" It does not say *why* not. Three
very different causes produce the same flag (see BASELINE1_SCOPE_REPRODUCTION.md):

  1. spec gap      -- the spec's own structured conditions table never defines
                      the output either, so nobody could translate it. This is
                      the same root cause as SCOPE's finding.
  2. checker FP    -- the model did establish the output, under an accessor name
                      the literal-name check doesn't recognise.
  3. generation    -- the table clearly defines it and the model dropped it.

This script decides none of those. It computes the recall against SCOPE's
ground truth, lists the extra flags, and for each one greps the command's own
section text for the output name so a human knows which cases need reading.

Usage:
    python3 baseline1_compare.py \
        --ground-truth training/scope_ground_truth_eac5_rel0.json \
        --findings eac5=results/baseline1_general/gpt56sol/eac5_dangling.json \
        --findings rel0=results/baseline1_general/gpt56sol/rel0_dangling.json \
        --label "GPT gpt-5.6-sol (high)" \
        --md-out results/baseline1_general/gpt56sol/rule_check.md
"""
import argparse
import json
import re
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
SECTIONS = BASE / "training-dataset" / "sections"

# Heading lines for the RMM spec's structured condition tables. pdftotext keeps
# the section number and the leading indentation, e.g.
#     "               B4.3.20.2 Failure conditions"
COND_HEADING = re.compile(r"^\s*(?:[A-Z]?[\d.]+\s+)?(?:Failure|Success) conditions\s*$", re.M)


def load_findings(path):
    data = json.loads(Path(path).read_text())
    return {e["command"]: e["outputs"] for e in data["dangling_output"]}, data


def condition_block(version, command):
    """Return the text from the first Failure/Success conditions heading onward."""
    p = SECTIONS / version / f"{command}_command.txt"
    if not p.exists():
        return None
    text = p.read_text(encoding="utf-8", errors="ignore")
    m = COND_HEADING.search(text)
    return text[m.start():] if m else ""


def mentions(block, name):
    return bool(block) and re.search(r"\b" + re.escape(name) + r"\b", block) is not None


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--ground-truth", type=Path, required=True)
    ap.add_argument("--findings", action="append", required=True,
                    metavar="VERSION=PATH",
                    help="Per-version dangling-output JSON from scope_rule_check_ourcode.py")
    ap.add_argument("--label", default="generator")
    ap.add_argument("--md-out", type=Path, default=None)
    ap.add_argument("--json-out", type=Path, default=None)
    args = ap.parse_args()

    gt = json.loads(args.ground_truth.read_text())
    report = {"label": args.label, "versions": {}}
    md = [f"# Dangling-output rule check: {args.label}", ""]

    for spec in args.findings:
        version, _, path = spec.partition("=")
        flagged, meta = load_findings(path)
        gt_entries = {e["command"]: e["fields"] for e in gt[version]["dangling_output_tp"]}

        reproduced, missed = [], []
        for cmd, fields in gt_entries.items():
            hit_fields = [f for f in fields if f in flagged.get(cmd, [])]
            (reproduced if hit_fields else missed).append(
                {"command": cmd, "expected": fields, "flagged": hit_fields}
            )

        extra = []
        for cmd, outs in flagged.items():
            gt_fields = set(gt_entries.get(cmd, []))
            for out in outs:
                if out in gt_fields:
                    continue
                block = condition_block(version, cmd)
                extra.append({
                    "command": cmd,
                    "output": out,
                    "in_gt_command": cmd in gt_entries,
                    "section_found": block is not None,
                    "conditions_table_mentions_output": mentions(block, out),
                    "verdict": None,     # filled in by hand after reading the source
                })

        report["versions"][version] = {
            "commands_checked": meta["commands_checked"],
            "ground_truth_tp_commands": len(gt_entries),
            "reproduced": reproduced,
            "missed": missed,
            "extra_flags": extra,
            "recall": f"{len(reproduced)}/{len(gt_entries)}",
        }

        md += [
            f"## {version}",
            "",
            f"- Commands checked: **{meta['commands_checked']}**",
            f"- SCOPE rule-mode TPs rediscovered: **{len(reproduced)}/{len(gt_entries)}**",
            f"- Extra flags (not in SCOPE's TP list): **{len(extra)}**",
            "",
            "| SCOPE TP command | Expected fields | Flagged in our run | Rediscovered |",
            "|---|---|---|---|",
        ]
        for e in sorted(reproduced + missed, key=lambda x: x["command"]):
            mark = "yes" if e["flagged"] else "**no**"
            md.append(f"| `{e['command']}` | {', '.join(e['expected'])} | "
                      f"{', '.join(e['flagged']) or '—'} | {mark} |")

        if extra:
            md += [
                "",
                "### Extra flags — need source-verified verdicts",
                "",
                "| Command | Output | Conditions table mentions it? | Verdict |",
                "|---|---|---|---|",
            ]
            for e in sorted(extra, key=lambda x: (x["command"], x["output"])):
                seen = "yes" if e["conditions_table_mentions_output"] else "no"
                md.append(f"| `{e['command']}` | `{e['output']}` | {seen} | TODO |")
        md.append("")

    total_gt = sum(v["ground_truth_tp_commands"] for v in report["versions"].values())
    total_rep = sum(len(v["reproduced"]) for v in report["versions"].values())
    report["total_recall"] = f"{total_rep}/{total_gt}"

    print(json.dumps({k: (v if k != "versions" else
                          {vk: vv["recall"] for vk, vv in v.items()})
                      for k, v in report.items() if k != "versions"} |
                     {"per_version_recall": {k: v["recall"] for k, v in report["versions"].items()},
                      "extra_flags": {k: len(v["extra_flags"]) for k, v in report["versions"].items()}},
                     indent=2))

    if args.md_out:
        args.md_out.parent.mkdir(parents=True, exist_ok=True)
        args.md_out.write_text("\n".join(md), encoding="utf-8")
        print(f"Wrote {args.md_out}")
    if args.json_out:
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        print(f"Wrote {args.json_out}")


if __name__ == "__main__":
    main()
