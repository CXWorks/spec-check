#!/usr/bin/env python3
"""Score a generator against the SCOPE rule-mode dangling-output benchmark.

Self-contained: reads only files inside this directory, so the package can be
copied elsewhere and still reproduce every number in REPORT.md.

The check, in one sentence: for each output the spec declares for a command
(excluding the ReturnCode field), does the generated spec function's *body* ever
mention that output's name? If not, the output is dangling and the item counts
as detected.

Usage:
    python3 score.py                       # score every model in predictions/
    python3 score.py --model gpt56sol      # just one
    python3 score.py --json-out scores.json
"""
import argparse
import ast
import json
import re
from pathlib import Path

HERE = Path(__file__).resolve().parent
CMD_MARKER = re.compile(r"\n([A-Z][A-Z0-9_]+) command\n-{10,}\n")
LIST_LINE = re.compile(r"^\[.*\]$")


def parse_raw(text):
    """Parse `scope --mode raw` into {command: {"outputs": [(name, type), ...]}}.

    Same logic as training/scope_rule_check_ourcode.py::parse_raw, inlined so
    this package stands alone.
    """
    text = "\n" + text
    matches = list(CMD_MARKER.finditer(text))
    cmds = {}
    for i, m in enumerate(matches):
        start = m.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        ctx = text[start:end].split("-" * 44 + "\n")[0]
        list_lines = [l for l in ctx.split("\n") if LIST_LINE.match(l.strip())]
        cmds[m.group(1)] = {
            "outputs": ast.literal_eval(list_lines[-1].strip()) if list_lines else []
        }
    return cmds


def split_signature_body(rs_text):
    """Split a spec fn into (signature, body) at the brace opening the body.

    Dropping the first line is not enough: verusfmt wraps long signatures over
    several lines, and a parameter name left in `body` would make a function
    whose entire body is `true` look like it defines every declared output.
    """
    start = rs_text.find("pub open spec fn")
    if start < 0:
        start = 0
    depth = 0
    for i in range(start, len(rs_text)):
        c = rs_text[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        elif c == "{" and depth == 0:
            return rs_text[start:i], rs_text[i + 1:]
    return rs_text[start:], ""


def dangling(outputs, body):
    return [name for name, typ in outputs
            if "ReturnCode" not in typ
            and not re.search(r"\b" + re.escape(name) + r"\b", body)]


def score_model(model_dir, gt, tables):
    per_version = {}
    for version in ("eac5", "rel0"):
        vdir = model_dir / version
        if not vdir.exists():
            continue
        gt_cmds = {i["command"]: i["fields"] for i in gt["items"] if i["version"] == version}

        flagged, missing = {}, []
        for cmd, info in tables[version].items():
            f = vdir / f"{cmd.lower()}.rs"
            if not f.exists():
                missing.append(cmd)
                continue
            _, body = split_signature_body(f.read_text())
            d = dangling(info["outputs"], body)
            if d:
                flagged[cmd] = d

        detected = {c: [x for x in fs if x in flagged.get(c, [])] for c, fs in gt_cmds.items()}
        hit = {c: v for c, v in detected.items() if v}
        extra = [(c, o) for c, outs in flagged.items() for o in outs
                 if o not in set(gt_cmds.get(c, []))]

        per_version[version] = {
            "commands_checked": len(tables[version]) - len(missing),
            "commands_missing": missing,
            "positives": len(gt_cmds),
            "detected": sorted(hit),
            "missed": sorted(set(gt_cmds) - set(hit)),
            "recall": f"{len(hit)}/{len(gt_cmds)}",
            "field_recall": (f"{sum(len(v) for v in detected.values())}/"
                             f"{sum(len(v) for v in gt_cmds.values())}"),
            "false_alarms": [f"{c}.{o}" for c, o in sorted(extra)],
        }
    tot_hit = sum(len(v["detected"]) for v in per_version.values())
    tot_pos = sum(v["positives"] for v in per_version.values())
    return {"per_version": per_version,
            "total_recall": f"{tot_hit}/{tot_pos}",
            "total_false_alarms": sum(len(v["false_alarms"]) for v in per_version.values())}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", default=None, help="score only this model directory")
    ap.add_argument("--predictions", type=Path, default=HERE / "predictions")
    ap.add_argument("--json-out", type=Path, default=None)
    args = ap.parse_args()

    gt = json.loads((HERE / "ground_truth.json").read_text())
    tables = {v: parse_raw((HERE / "scope_tables" / f"{v}_raw.txt").read_text())
              for v in ("eac5", "rel0")}

    models = ([args.predictions / args.model] if args.model
              else sorted(p for p in args.predictions.iterdir() if p.is_dir()))

    all_scores = {}
    for md in models:
        s = score_model(md, gt, tables)
        all_scores[md.name] = s
        print(f"\n=== {md.name} ===")
        print(f"  TOTAL recall {s['total_recall']}   false alarms {s['total_false_alarms']}")
        for v, d in s["per_version"].items():
            print(f"  {v}: recall {d['recall']}  fields {d['field_recall']}  "
                  f"false alarms {d['false_alarms'] or '—'}  (checked {d['commands_checked']})")
            if d["missed"]:
                print(f"      missed: {', '.join(d['missed'])}")

    if args.json_out:
        args.json_out.write_text(json.dumps(all_scores, indent=2) + "\n")
        print(f"\nWrote {args.json_out}")


if __name__ == "__main__":
    main()
