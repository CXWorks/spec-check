#!/usr/bin/env python3
"""Extractor 2: which return codes does the document declare for each command?

    python3 scripts/extract_returns.py --doc sdei
    python3 scripts/extract_returns.py --doc drtm --out results/returns-drtm.json

This is the input the rule check needs. Extractor 1 gives the text a command is
described by; this gives the list the generated spec is then checked against --
a code the document declares but the spec never constrains is the same shape of
finding as RMM's dangling output.

IT IS NOT THE RMM CHECK. RMM commands declare named outputs in a table
(`out_top`, `rtt_tree`), so "is this output ever constrained" is a direct
question. PSCI, SDEI, DRTM and FF-A are SMC-convention APIs: the result is a
value in X0 described by error codes and bit fields, and there is no table of
named outputs to leave dangling. The transferable question is the one about
return codes.

Two filters, because the naive regex for "ALL_CAPS_WITH_UNDERSCORES" pulls in
command names -- SDEI_EVENT_REGISTER and FFA_VERSION were both being counted as
return codes:

  1. A token that is a command name in this document is not a return code.
     Extractor 1's output is exactly that list, so it is free.
  2. A real return code recurs across commands; a name that appears in only one
     section is usually that section's own subject or a cross-reference. The
     threshold is configurable and reported, never silent.

COVERAGE VARIES ENORMOUSLY and is reported per document rather than averaged
away: SDEI declares a Return block for 19 of 19 commands and DRTM for 9 of 11,
but PSCI has one for 3 of 22 and FF-A for 7 of 39. Where the block is missing
the whole section is scanned, which is weaker -- those commands are flagged
`weak_source` in the output so a downstream count can exclude them.
"""
import argparse, json, re, sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SECTIONS = ROOT / "training-dataset" / "sections"

# Where the return declaration starts, per document. Everything else is shared.
# Each document defines its return codes once, in a table. That table is the
# authoritative vocabulary and beats any heuristic: recurrence-across-commands
# also promotes domain acronyms, and ELC, IRQ, CRTM, DLME, PCR and TPM were all
# being counted as return codes before this.
CODE_TABLE = {
    "drtm":    (r'(?m)^\s*3\.18 +Return codes\b', r'(?m)^\s*(?:3\.19|4\.)'),
    "sdei":    (r'(?m)^\s*5\.3 +Return codes\b',  r'(?m)^\s*(?:5\.4|6\.)'),
    # PSCI has no numbered "Return codes" section; the list lives under a
    # table caption instead, so anchor on the caption.
    "psci_13": (r'(?mi)^\s*Table \d+ +Return error codes\s*$', r'(?mi)^\s*Table \d+\b'),
}
PDF_OF = {
    "drtm":    "DEN0113_DRTM_1.4.pdf",
    "sdei":    "ARM_DEN0054C_Software_Delegated_Exception_Interface.pdf",
    "psci_13": "DEN0022F.b_Power_State_Coordination_Interface.pdf",
    "ffa":     "DEN0077A_Firmware_Framework_Arm_A-profile_1.3_ALP4.pdf",
}

RETURN_HEAD = {
    "sdei":    r'(?mi)^\s*Return\b',
    "drtm":    r'(?mi)^\s*Return\b',
    "psci_13": r'(?mi)^\s*Return\b',
    "ffa":     r'(?mi)^\s*(?:Return|Output) ?(?:value|parameter)s?\b|^\s*Return\b',
}
# Underscores are NOT required. Insisting on them dropped every single-word
# code -- SUCCESS, DENIED, ALREADY_ON's siblings -- and left a two-code
# vocabulary on documents that plainly have more.
CODE = re.compile(r'\b([A-Z][A-Z0-9_]{2,})\b')
# All-caps words that are prose or structure rather than return values. The
# recurrence filter removes most of the rest.
STOP = {"MUST", "SHOULD", "SHALL", "NOTE", "TABLE", "SEE", "THE", "AND", "NOT",
        "ALL", "ANY", "FOR", "THIS", "THAT", "WITH", "FROM", "WHEN", "IF",
        "SMC", "HVC", "SMCCC", "PSCI", "SDEI", "DRTM", "FFA", "API", "ABI",
        "CPU", "PE", "EL", "ELR", "SPSR", "VBAR", "MPIDR", "UUID", "ID",
        "TRUE", "FALSE", "MBZ", "SBZ", "RES", "IMPLEMENTATION", "DEFINED"}


def load(doc):
    d = SECTIONS / doc
    if not d.is_dir():
        sys.exit(f"no sections for {doc} -- run extractor 1 first")
    return {p.name[:-len("_command.txt")]: p.read_text(errors="replace")
            for p in sorted(d.glob("*_command.txt"))}


def doc_vocab(doc):
    """Return codes as the document itself enumerates them, or None."""
    spec = CODE_TABLE.get(doc)
    if not spec or doc not in PDF_OF:
        return None
    pdf = ROOT / "specs" / PDF_OF[doc]
    if not pdf.exists():
        return None
    import subprocess
    txt = subprocess.run(["pdftotext", "-layout", str(pdf), "-"],
                         capture_output=True, text=True, check=True).stdout
    head, tail = spec
    # The LAST match, not the first: the same heading appears in the table of
    # contents, where the "section" that follows is more contents and contains
    # no codes at all. That is how this silently returned nothing.
    ms = list(re.finditer(head, txt))
    if not ms:
        return None
    m = ms[-1]
    e = re.search(tail, txt[m.end():])
    block = txt[m.end(): m.end() + (e.start() if e else 4000)]
    # Only the first column. A table row is "NAME  Description...  Value", so
    # the code is the leading token; taking every match also picked up acronyms
    # out of the Description column (TPM) and the page footer (DEN, ARM).
    # Indentation is not constrained at all: DRTM's rows start at the margin,
    # SDEI's about ten columns in, PSCI's about thirty-eight. Any fixed bound
    # silently drops one of them. What keeps prose out is that the token must
    # be the first thing on its line and be followed by a column gap, inside a
    # block that is already known to be the code table.
    rows = re.findall(r'(?m)^\s*([A-Z][A-Z0-9_]{2,})\s{2,}\S', block)
    return {c for c in rows if c not in STOP} or None


def candidates(text, head_pat, window):
    """Codes from the Return block, or from the whole section if there is none."""
    m = re.search(head_pat, text) if head_pat else None
    if m:
        return CODE.findall(text[m.start(): m.start() + window]), True
    return CODE.findall(text), False


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--doc", required=True)
    ap.add_argument("--window", type=int, default=3000,
                    help="chars of the Return block to read")
    ap.add_argument("--min-commands", type=int, default=2,
                    help="a code must appear in at least this many commands")
    ap.add_argument("--out", default=None)
    a = ap.parse_args()

    cmds = load(a.doc)
    names = set(cmds)                      # filter 1: command names
    head = RETURN_HEAD.get(a.doc)

    raw, strong = {}, {}
    for c, t in cmds.items():
        toks, ok = candidates(t, head, a.window)
        raw[c] = [x for x in toks
                  if x not in names and x != c and x not in STOP]
        strong[c] = ok

    vocab = doc_vocab(a.doc)
    source = "document's own Return codes table"
    if not vocab:
        # Fallback only: a real code recurs across commands. Weaker, and it
        # promotes domain acronyms, so say so rather than reporting a number
        # that looks equally solid.
        spread = Counter()
        for c, toks in raw.items():
            for x in set(toks):
                spread[x] += 1
        vocab = {x for x, n in spread.items() if n >= a.min_commands}
        source = f"HEURISTIC: seen in >= {a.min_commands} commands (no code table found)"

    rows = {c: sorted({x for x in toks if x in vocab}) for c, toks in raw.items()}
    n_strong = sum(strong.values())
    print(f"{a.doc}: {len(cmds)} commands, {n_strong} with a Return block "
          f"({n_strong*100//max(len(cmds),1)}%)")
    print(f"  code vocabulary ({len(vocab)}) from {source}:")
    print(f"    {', '.join(sorted(vocab)[:14])}{' ...' if len(vocab) > 14 else ''}")
    empty = [c for c, v in rows.items() if not v]
    print(f"  commands with no code extracted: {len(empty)}"
          + (f"  {empty[:6]}" if empty else ""))
    for c in sorted(rows)[:5]:
        print(f"    {c:32s} {rows[c]}{'' if strong[c] else '   [weak_source]'}")
    if a.out:
        Path(a.out).parent.mkdir(parents=True, exist_ok=True)
        Path(a.out).write_text(json.dumps(
            {"doc": a.doc, "vocab": sorted(vocab),
             "commands": {c: {"codes": rows[c], "weak_source": not strong[c]}
                          for c in rows}}, indent=1))
        print(f"  [wrote] {a.out}")


if __name__ == "__main__":
    main()
