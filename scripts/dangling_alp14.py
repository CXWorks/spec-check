#!/usr/bin/env python3
"""Dangling-output check on alp14, adjudicated against the PDF itself.

    python3 scripts/dangling_alp14.py --gen-dir results/gen98/specs/sel-pr3/alp14

An output is *dangling* in a Verus spec when the signature declares it but the
body never constrains it. That alone is not a finding: it matters only together
with what the PDF says.

  PDF defines it  +  spec leaves it dangling  ->  generation defect
  PDF never defines it + spec leaves it dangling -> SPEC GAP (the finding)
  PDF never defines it + spec constrains it   ->  confabulation

The earlier pass (OUR_CODE_RULE_CHECK.md) took the output list from a SCOPE dump
and adjudicated by reading the PDF page by page. The SCOPE submodule is not
initialized here, so both halves come from training-dataset/sections/alp14/ --
the per-command `Output values` table gives the declared outputs, and the
`Failure conditions` / `Success conditions` subsections give the definitions.

Not from the PDF via pdftotext, which was tried first and is not safe here:
`-layout` shreds some tables into stacked columns (RMI_PSMMU_IRQ_NOTIFY comes out
as `result/action/rd` then `X0/X1/X2` on separate lines), so a row regex silently
drops outputs -- `action` among them -- and 22 of 98 commands parsed to zero
outputs. The section files are the project's own extraction, are cleanly aligned
in all 98 files, and are the exact text the model was shown. The PDF remains the
authority for adjudicating a finding by hand.

Why the output list cannot come from the Verus signature: a signature mixes
inputs and outputs with no syntactic marker, so `unused parameter` flags inputs
too -- `PSCI_FEATURES.psci_func_id` is an input, and reading it as a dangling
output is simply wrong.
"""
import argparse, json, re, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))

SECTIONS = ROOT / "training-dataset" / "sections" / "alp14"
SUBSEC = re.compile(r'(?m)^\s*B[\d.]+\s+(Output values|Failure conditions|'
                    r'Success conditions|Footprint|Input values|Context)\b')
# A table row starts with an identifier, then the register column.
ROW = re.compile(r'(?m)^\s{4,}([a-z_][a-z0-9_]*)\s{2,}(X\d+|R\d+)\b')


def parse_sections(section_dir: Path):
    """{cmd: {"outputs": [...], "conditions": "<failure+success text>"}}"""
    out = {}
    for f in sorted(section_dir.glob("*_command.txt")):
        cmd = f.name[:-len("_command.txt")]
        body = f.read_text(errors="replace")
        parts, marks = {}, list(SUBSEC.finditer(body))
        for j, s in enumerate(marks):
            e = marks[j + 1].start() if j + 1 < len(marks) else len(body)
            parts.setdefault(s.group(1), "")
            parts[s.group(1)] += body[s.end():e]
        outs, seen = [], set()
        for r in ROW.finditer(parts.get("Output values", "")):
            if r.group(1) not in seen:
                seen.add(r.group(1)); outs.append(r.group(1))
        conds = parts.get("Failure conditions", "") + parts.get("Success conditions", "")
        out[cmd] = {"outputs": outs, "conditions": conds}
    return out


def spec_dangling(src):
    """(declared params, names never mentioned in the body)."""
    from verify_generated_verus import extract_fn_block, split_params
    name, params_str, text = extract_fn_block(src)
    if not name:
        return None, None
    names = [p[0] for p in split_params(params_str)]
    body = text[text.find("{") + 1: text.rfind("}")]
    dang = {n for n in names
            if not re.search(r'\b' + re.escape(n) + r'\b', body)}
    return set(names), dang


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--gen-dir", required=True)
    ap.add_argument("--out", default=None)
    a = ap.parse_args()

    pdf = parse_sections(SECTIONS)
    gen = Path(a.gen_dir)

    rows = []
    for cmd, info in sorted(pdf.items()):
        f = gen / f"{cmd.lower()}.rs"
        if not f.exists():
            continue
        declared, dang = spec_dangling(f.read_text())
        if declared is None:
            rows.append({"command": cmd, "class": "unparseable"}); continue
        for o in info["outputs"]:
            if o == "result":          # the return code is not an output value
                continue
            in_pdf = bool(re.search(r'\b' + re.escape(o) + r'\b', info["conditions"]))
            if o not in declared:
                cls = "missing_param" if in_pdf else "spec_gap_absent_param"
            elif o in dang:
                cls = "generation_defect" if in_pdf else "SPEC_GAP"
            else:
                cls = "ok" if in_pdf else "confabulation"
            rows.append({"command": cmd, "output": o, "class": cls,
                         "defined_in_pdf_tables": in_pdf})

    from collections import Counter
    c = Counter(r["class"] for r in rows)
    print(f"=== {gen} ===")
    for k in ("SPEC_GAP", "spec_gap_absent_param", "confabulation",
              "generation_defect", "missing_param", "ok", "unparseable"):
        if c.get(k):
            print(f"  {k:24s} {c[k]}")
    for k in ("SPEC_GAP", "spec_gap_absent_param", "confabulation"):
        hits = [r for r in rows if r["class"] == k]
        if hits:
            print(f"\n  --- {k} ---")
            for r in hits:
                print(f"    {r['command']:34s} {r['output']}")
    if a.out:
        Path(a.out).write_text(json.dumps(rows, indent=1))
        print(f"\n[wrote] {a.out}")


if __name__ == "__main__":
    main()
