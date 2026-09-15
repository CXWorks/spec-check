#!/usr/bin/env python3
"""Document-only spec-gap scan: declared outputs that no condition ever defines.

    python3 scripts/spec_gap_scan.py --pdf specs/DEN0137_1.1-alp14_rmm-arch_external.pdf

Needs no model and no gold. It asks one question per declared output: does the
name appear anywhere in this command's Failure/Success conditions? If not, the
document declares a value it never pins down, and two vendors can return
different things without either violating the spec.

Table parsing has to survive two shapes. pdftotext emits most tables row-wise
("out_top  X1  63:0  Address  ..."), but shreds some into stacked columns -- the
names on consecutive lines, then the registers, then the types. A row regex
silently drops outputs in the shredded form (`action` went missing from
RMI_PSMMU_IRQ_NOTIFY that way). So names are recovered by shape instead:
inside the Output values block, an output name is a lone snake_case token, while
column headers are capitalised, types are CamelCase, registers are X<n> and bit
ranges contain a colon.
"""
import argparse, json, re, subprocess, sys
from pathlib import Path

CMD_HEAD = re.compile(r'(?m)^B\d+\.\d+\.\d+ ([A-Z][A-Z0-9_]+) command$')
SUBSEC = re.compile(r'(?m)^\s*B[\d.]+\s+(Output values|Failure conditions|'
                    r'Success conditions|Footprint|Input values|Context)\b')
# A table row: name, then the register column, then a bit range. Anchoring on
# the register is what keeps Description-column prose out of the result -- an
# earlier shape-based heuristic that only required "lowercase token" pulled in
# every English word under Description and reported 365 undefined outputs on
# alp14, where the true answer is 6.
ROW = re.compile(r'(?m)^\s*([a-z][a-z0-9_]*)\s+(?:X|R)\d+\s+\d+:\d+\b')


def text_of(pdf: Path, cache: Path):
    if not cache.exists():
        cache.write_text(subprocess.run(["pdftotext", "-layout", str(pdf), "-"],
                                        capture_output=True, text=True,
                                        check=True).stdout)
    return cache.read_text()


def sections(text):
    heads = list(CMD_HEAD.finditer(text))
    for i, m in enumerate(heads):
        end = heads[i + 1].start() if i + 1 < len(heads) else len(text)
        body = text[m.start():end]
        parts, marks = {}, list(SUBSEC.finditer(body))
        for j, s in enumerate(marks):
            e = marks[j + 1].start() if j + 1 < len(marks) else len(body)
            parts.setdefault(s.group(1), "")
            parts[s.group(1)] += body[s.end():e]
        yield m.group(1), parts


def outputs_of(block):
    """Output names from the row-wise layout.

    Returns (names, parsed_ok). parsed_ok is False when the block has text but
    no row matched -- pdftotext shreds some tables into stacked columns, and
    those must be reported as unparsed rather than guessed at.
    """
    names, seen = [], set()
    for m in ROW.finditer(block):
        n = m.group(1)
        if n not in seen:
            seen.add(n); names.append(n)
    parsed_ok = bool(names) or not block.strip()
    return names, parsed_ok


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--pdf", required=True)
    ap.add_argument("--cache", default=None)
    ap.add_argument("--out", default=None)
    a = ap.parse_args()
    pdf = Path(a.pdf)
    cache = Path(a.cache) if a.cache else pdf.with_suffix(".txt")
    text = text_of(pdf, cache)

    rows, unparsed, ncmd = [], [], 0
    for cmd, parts in sections(text):
        ncmd += 1
        outs, ok = outputs_of(parts.get("Output values", ""))
        if not ok:
            unparsed.append(cmd)
        conds = (parts.get("Failure conditions", "")
                 + parts.get("Success conditions", ""))
        empty = "does not have any success conditions" in conds.lower()
        for o in outs:
            if o == "result":
                continue
            defined = bool(re.search(r'\b' + re.escape(o) + r'\b', conds))
            if not defined:
                rows.append({"command": cmd, "output": o,
                             "success_table_empty": empty})
    print(f"{pdf.name}: {ncmd} 条命令, {len(rows)} 个未定义输出"
          f", {len(unparsed)} 条表格解析失败")
    by = {}
    for r in rows:
        by.setdefault(r["command"], []).append(r["output"])
    for c in sorted(by):
        flag = " [成功条件表明确为空]" if any(
            r["success_table_empty"] for r in rows if r["command"] == c) else ""
        print(f"  {c:34s} {', '.join(by[c])}{flag}")
    if unparsed:
        print(f"\n  ⚠️ 表格被 pdftotext 打散、需人工看: {', '.join(unparsed)}")
    if a.out:
        Path(a.out).write_text(json.dumps(
            {"rows": rows, "unparsed": unparsed, "commands": ncmd}, indent=1))


if __name__ == "__main__":
    main()
