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
"""
import ast
import re
from pathlib import Path

BASE = Path(__file__).resolve().parent.parent
RAW_FILE = BASE / "scope" / "alp14_raw.txt"
GEN_DIR = BASE / "results" / "ab_test_qwen_v3retrained" / "v3_qwen" / "alp14"

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


def extract_our_clauses(rs_text):
    """Return list of top-level implication RHS clauses guarded by
    `result.is_Ok() ==>` or `result.is_Err() ==>` in our generated spec fn."""
    body_lines = rs_text.split("\n")[1:]  # drop the `fn ...(...) -> bool {` signature line
    body = "\n".join(body_lines)
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


def main():
    cmds = parse_raw(RAW_FILE.read_text())
    print(f"Parsed {len(cmds)} commands from {RAW_FILE}")

    dangling_report = []
    footprint_report = []
    missing = []

    for name, info in sorted(cmds.items()):
        dirname = name.lower()
        rs_path = GEN_DIR / dirname / "generated.formatted.rs"
        if not rs_path.exists():
            missing.append(name)
            continue
        rs_text = rs_path.read_text()
        clauses, body = extract_our_clauses(rs_text)

        dangling = dangling_output_check(name, info["outputs"], body)
        if dangling:
            dangling_report.append((name, dangling))

        flagged = footprint_check(name, clauses, info["outputs"], info["footprints"])
        if flagged:
            footprint_report.append((name, flagged))

    print(f"\nCommands with no matching generated dir: {len(missing)}")
    for m in missing:
        print(f"  {m}")

    print(f"\n=== Dangling output check: {len(dangling_report)} commands flagged ===")
    for name, outs in dangling_report:
        print(f"{name}")
        for o in outs:
            print(f"  {o}")

    print(f"\n=== Footprint check: {len(footprint_report)} commands flagged ===")
    for name, clauses in footprint_report:
        print(f"{name}")
        for c in clauses:
            print(f"  {c}")


if __name__ == "__main__":
    main()
