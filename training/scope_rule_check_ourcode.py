#!/usr/bin/env python3
"""Apply SCOPE's rule-based checks (dangling-output, footprint) to OUR
Qwen-generated alp14 Verus code, instead of SCOPE's own generated code.

SCOPE's rule-mode checks (scope/scope: detect_dangling_output, footprint_checks)
operate on structured tables (outputs, failure/success conditions, footprint)
that SCOPE extracts directly from the spec PDF -- this extraction is generator-
independent. This script reuses that extraction (dumped via
`scope --target alp14 --mode raw`) and re-implements the same two checks
against our own generated `_spec fn` bodies.

Input:
  scope/alp14_raw.txt                                             (SCOPE's raw parse of alp14)
  results/ab_test_qwen_v3retrained/v3_qwen/alp14/<cmd>/generated.formatted.rs  (our generated code)

Both are overridable so the same check can be pointed at any spec version and any
generator's output:

  python3 scope_rule_check_ourcode.py \
      --raw-file work/scope_run/eac5_raw.txt \
      --gen-dir  results/baseline1_general/gpt56sol/eac5 \
      --json-out results/baseline1_general/gpt56sol/eac5_dangling.json

  # gold-spec control run (per-command files, not per-command directories)
  python3 scope_rule_check_ourcode.py \
      --raw-file work/scope_run/eac5_raw.txt \
      --gen-dir  training-dataset/specs/eac5 --gen-pattern '{cmd}_spec.rs'
"""
import argparse
import ast
import json
import re
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
RAW_FILE = BASE / "scope" / "alp14_raw.txt"
GEN_DIR = BASE / "results" / "ab_test_qwen_v3retrained" / "v3_qwen" / "alp14"
GEN_PATTERN = "{cmd}/generated.formatted.rs"

CMD_MARKER = re.compile(r"\n([A-Z][A-Z0-9_]+) command\n-{10,}\n")
LIST_LINE = re.compile(r"^\[.*\]$")


def parse_raw(text):
    text = "\n" + text  # so the first marker matches too
    matches = list(CMD_MARKER.finditer(text))
    cmds = {}
    for i, m in enumerate(matches):
        name = m.group(1)
        start = m.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        body = text[start:end]
        parts = body.split("-" * 44 + "\n")
        # parts[0] = context block, parts[1] = failure, parts[2] = success, parts[3] = footprint
        ctx_lines = [l for l in parts[0].split("\n") if l.strip()]
        list_lines = [l for l in ctx_lines if LIST_LINE.match(l.strip())]
        outputs = ast.literal_eval(list_lines[-1].strip()) if list_lines else []

        failure_posts = re.findall(r"^\s*post:\s*(.*)$", parts[1], re.M) if len(parts) > 1 else []
        success_posts = re.findall(r"^\s*post:\s*(.*)$", parts[2], re.M) if len(parts) > 2 else []

        footprints = []
        if len(parts) > 3:
            for l in parts[3].split("\n"):
                l = l.strip().rstrip("=").strip()
                if l and ": " in l:
                    fid, fval = l.split(": ", 1)
                    footprints.append((fid.strip(), fval.strip()))

        cmds[name] = {
            "outputs": outputs,
            "failure_posts": failure_posts,
            "success_posts": success_posts,
            "footprints": footprints,
        }
    return cmds


def split_top_level(s, sep):
    """Split s on `sep` only at paren-depth 0."""
    parts, depth, buf = [], 0, ""
    i = 0
    while i < len(s):
        c = s[i]
        if c == "(":
            depth += 1
        elif c == ")":
            depth -= 1
        if depth == 0 and s[i:i + len(sep)] == sep:
            parts.append(buf)
            buf = ""
            i += len(sep)
            continue
        buf += c
        i += 1
    parts.append(buf)
    return [p.strip() for p in parts if p.strip()]


def split_signature_body(rs_text):
    """Split a spec fn item into (signature, body) at the brace opening the body.

    Dropping the first line is not enough: verusfmt wraps a long signature over
    several lines, and any parameter name left in `body` makes the
    dangling-output check trivially pass -- a function whose entire body is
    `true` would look like it defines every declared output.
    """
    start = rs_text.find("pub open spec fn")
    if start < 0:
        start = 0
    depth = 0
    for i in range(start, len(rs_text)):
        ch = rs_text[i]
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif ch == "{" and depth == 0:
            return rs_text[start:i], rs_text[i + 1:]
    return rs_text[start:], ""      # stub or malformed: no body at all


def extract_our_clauses(rs_text):
    """Return list of top-level implication RHS clauses guarded by
    `result.is_Ok() ==>` or `result.is_Err() ==>` in our generated spec fn."""
    _, body = split_signature_body(rs_text)
    clauses = []
    for m in re.finditer(r"result\.is_(?:Ok|Err)\(\)\s*==>\s*(.*)", body):
        rhs = m.group(1)
        # cut off at the clause's own top-level end: matching close paren / `&&` at depth<0 is handled
        # by only taking up to the end of line's balanced parens; approximate by taking to end of line
        rhs = rhs.split("\n")[0].rstrip().rstrip(")").rstrip()
        clauses.extend(split_top_level(rhs, "&&"))
    return clauses, body


def normalize(s):
    s = re.sub(r"\b(old_s|new_s)\s*,\s*", "", s)
    s = re.sub(r"\s+", " ", s).strip()
    return s


def dangling_output_check(cmd_name, outputs, body):
    dangling = []
    for out_name, out_type in outputs:
        if "ReturnCode" in out_type:
            continue
        if not re.search(r"\b" + re.escape(out_name) + r"\b", body):
            dangling.append(out_name)
    return dangling


def footprint_check(cmd_name, clauses, outputs, footprints):
    output_names = {o[0] for o in outputs}
    footprint_vals = [normalize(f[1]) for f in footprints]
    flagged = []
    for clause in clauses:
        if "==" not in clause:
            continue
        lhs = normalize(clause.split("==")[0])
        if lhs in output_names:
            continue
        if any(fv and fv in lhs for fv in footprint_vals):
            continue
        # heuristic carve-outs mirrored from scope/scope's footprint_checks_inner
        if lhs.endswith(").state") and (".Granule(" in lhs or "GranuleAt(" in lhs or lhs.startswith("Granule") or lhs.startswith("GranuleAt")):
            continue
        flagged.append(clause)
    return flagged


def parse_args():
    ap = argparse.ArgumentParser(
        description="Apply SCOPE's dangling-output/footprint checks to generated Verus code",
    )
    ap.add_argument("--raw-file", type=Path, default=RAW_FILE,
                    help=f"SCOPE `--mode raw` dump (default: {RAW_FILE})")
    ap.add_argument("--gen-dir", type=Path, default=GEN_DIR,
                    help=f"Directory holding the generated specs (default: {GEN_DIR})")
    ap.add_argument("--gen-pattern", default=GEN_PATTERN,
                    help=f"Path of one command's code under --gen-dir, with {{cmd}} as the "
                         f"lowercased command name (default: {GEN_PATTERN})")
    ap.add_argument("--json-out", type=Path, default=None,
                    help="Also write the findings as JSON")
    ap.add_argument("--footprint", action="store_true",
                    help="Also run the footprint check (off by default: it is dominated by "
                         "noise from our explicit old_s/new_s state threading)")
    ap.add_argument("--label", default=None,
                    help="Free-text label recorded in the JSON output (e.g. model name)")
    return ap.parse_args()


def main():
    args = parse_args()
    cmds = parse_raw(args.raw_file.read_text())
    print(f"Parsed {len(cmds)} commands from {args.raw_file}")

    dangling_report = []
    footprint_report = []
    missing = []

    for name, info in sorted(cmds.items()):
        rs_path = args.gen_dir / args.gen_pattern.format(cmd=name.lower())
        if not rs_path.exists():
            missing.append(name)
            continue
        rs_text = rs_path.read_text()
        clauses, body = extract_our_clauses(rs_text)

        dangling = dangling_output_check(name, info["outputs"], body)
        if dangling:
            dangling_report.append((name, dangling))

        if args.footprint:
            flagged = footprint_check(name, clauses, info["outputs"], info["footprints"])
            if flagged:
                footprint_report.append((name, flagged))

    checked = len(cmds) - len(missing)
    print(f"\nCommands with no generated code: {len(missing)} (checked {checked})")
    for m in missing:
        print(f"  {m}")

    print(f"\n=== Dangling output check: {len(dangling_report)} commands flagged ===")
    for name, outs in dangling_report:
        print(f"{name}")
        for o in outs:
            print(f"  {o}")

    if args.footprint:
        print(f"\n=== Footprint check: {len(footprint_report)} commands flagged ===")
        for name, clauses in footprint_report:
            print(f"{name}")
            for c in clauses:
                print(f"  {c}")

    if args.json_out:
        payload = {
            "label": args.label,
            "raw_file": str(args.raw_file),
            "gen_dir": str(args.gen_dir),
            "gen_pattern": args.gen_pattern,
            "commands_in_raw": len(cmds),
            "commands_checked": checked,
            "commands_missing": missing,
            "dangling_output": [
                {"command": name, "outputs": outs} for name, outs in dangling_report
            ],
        }
        if args.footprint:
            payload["footprint"] = [
                {"command": name, "clauses": clauses} for name, clauses in footprint_report
            ]
        args.json_out.parent.mkdir(parents=True, exist_ok=True)
        args.json_out.write_text(json.dumps(payload, indent=2) + "\n")
        print(f"\nWrote {args.json_out}")


if __name__ == "__main__":
    main()
