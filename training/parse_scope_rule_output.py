#!/usr/bin/env python3
"""Parse SCOPE's `--mode rule` output (after its own labelling patch is applied)
into a machine-readable ground-truth file.

SCOPE's rule mode prints two sections, `[Dangling output result]` and
`[Footprint check result]`. `patch/{target}_rule.patch` then annotates each
finding with `// XXX: TP` or `// XXX: FP` -- the paper's pre-determined
labelling. This script extracts the labelled dangling-output findings so the
same 8 true positives can be joined against any generator's output.

Usage:
    python3 parse_scope_rule_output.py \
        --rule-file work/scope_run/eac5_rule.txt --version eac5 \
        --rule-file work/scope_run/rel0_rule.txt --version rel0 \
        --out training/scope_ground_truth_eac5_rel0.json
"""
import argparse
import json
import re
from pathlib import Path

SECTION = re.compile(r"^\[(.+?) result\]\s*$")
SEPARATOR = "-" * 44
CMD_LINE = re.compile(r"^(?P<cmd>[A-Z][A-Z0-9_]+) command\s*(?://\s*XXX:\s*(?P<label>\w+))?\s*$")
FIELD_LINE = re.compile(r"^(?P<field>[A-Za-z_][\w.]*)\s*(?://\s*XXX:\s*(?P<label>\w+))?\s*$")


def parse_rule_file(text):
    """Return {section_name: [ {command, label, fields:[{name,label}]} ]}."""
    sections = {}
    current_section = None
    current_cmd = None

    for line in text.splitlines():
        line = line.rstrip()
        if not line:
            continue

        m = SECTION.match(line)
        if m:
            current_section = m.group(1)
            sections[current_section] = []
            current_cmd = None
            continue

        if line.startswith(SEPARATOR):
            current_cmd = None
            continue

        if current_section is None:
            continue

        m = CMD_LINE.match(line)
        if m:
            current_cmd = {
                "command": m.group("cmd"),
                "label": m.group("label"),
                "fields": [],
            }
            sections[current_section].append(current_cmd)
            continue

        m = FIELD_LINE.match(line)
        if m and current_cmd is not None:
            current_cmd["fields"].append(
                {"name": m.group("field"), "label": m.group("label")}
            )

    return sections


def true_positive_fields(cmd_entry):
    """Fields counted as SCOPE true positives for a command.

    Labels sit at two levels: on the command line (applies to every field of
    that command) or on individual field lines (RMI_RTT_READ_ENTRY labels
    walk_level TP and desc FP separately).
    """
    if cmd_entry["label"] == "TP":
        return [f["name"] for f in cmd_entry["fields"]]
    if cmd_entry["label"] == "FP":
        return []
    return [f["name"] for f in cmd_entry["fields"] if f["label"] == "TP"]


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--rule-file", action="append", required=True, type=Path,
                    help="Patched {target}_rule.txt (repeatable, paired with --version)")
    ap.add_argument("--version", action="append", required=True,
                    help="Spec version label for the preceding --rule-file")
    ap.add_argument("--out", type=Path, required=True)
    args = ap.parse_args()

    if len(args.rule_file) != len(args.version):
        ap.error("--rule-file and --version must be given the same number of times")

    ground_truth = {}
    for path, version in zip(args.rule_file, args.version):
        sections = parse_rule_file(path.read_text())
        dangling = sections.get("Dangling output", [])
        if not any(e["label"] or any(f["label"] for f in e["fields"]) for e in dangling):
            raise SystemExit(
                f"{path}: no XXX labels found -- apply patch/{version}_rule.patch first"
            )

        entries = []
        for e in dangling:
            tp_fields = true_positive_fields(e)
            if tp_fields:
                entries.append({"command": e["command"], "fields": tp_fields})

        ground_truth[version] = {
            "source": str(path),
            "dangling_output_tp": entries,
            "tp_command_count": len(entries),
            "footprint": [
                {"command": e["command"], "label": e["label"],
                 "fields": [f["name"] for f in e["fields"]]}
                for e in sections.get("Footprint check", [])
            ],
        }
        print(f"{version}: {len(entries)} dangling-output TP commands "
              f"({sum(len(e['fields']) for e in entries)} fields)")
        for e in entries:
            print(f"  {e['command']}: {', '.join(e['fields'])}")

    args.out.write_text(json.dumps(ground_truth, indent=2) + "\n")
    print(f"\nWrote {args.out}")


if __name__ == "__main__":
    main()
