#!/usr/bin/env python3
"""
extract_sections_sbi3.py

Extract per-function sections from the RISC-V SBI **v3.0** spec
(`specs/riscv-sbi.pdf`, ratified 2025-07-16).

Why this is separate from extract_sections_sbi.py: that one targets SBI v2.0,
whose chapter numbering it hard-codes (ch3 = Base, ch7 = RFENCE, ...). v3.0
renumbered everything (ch4 = Base, ch5 = Legacy) and grew to ~20 chapters, and
its headings carry a trailing dot after the subsection number ("4.1. Function:"
rather than "4.1  Function:"), which the v2.0 pattern does not match. Rather
than make one extractor guess at the revision, v2.0 keeps its own file.

Section shape in v3.0:

    4.1. Function: Get SBI specification version (FID #0)
    struct sbiret sbi_get_spec_version(void);
    Returns the current SBI specification version. ...

The C prototype on the line after the heading carries the canonical
`sbi_*` name, so command names come from there rather than from the prose title.

Usage:
    python3 extract_sections_sbi3.py <text-file> [out-dir]

The text file is the PDF converted with either `pdftotext -layout` or pypdf.
"""

import os
import re
import sys

# Chapters that are not live function specifications.
#   1 Introduction / 2 Binary Encoding / 3 Shared parameters
#   5 Legacy Extensions -- deprecated in v3.0, and specified as bare "Extension:"
#     entries with no sbiret contract to translate
SKIP_CHAPTERS = {"1", "2", "3", "5"}

# "4.1. Function: Get SBI specification version (FID #0)"
_SEC_PAT = re.compile(r'(?m)^(\d+)\.(\d+)\.\s+Function:\s*(.+?)\s*$')

# "struct sbiret sbi_get_spec_version(void);"
_PROTO_PAT = re.compile(r'\b(sbi_[a-z0-9_]+)\s*\(')


def preprocess(txt_path: str) -> str:
    """Strip TOC, copyright, running page headers from the extracted text."""
    out = []
    with open(txt_path, "r", errors="replace") as fh:
        lines = fh.readlines()

    for raw in lines:
        line = raw.strip()
        if not line:
            continue
        # Running page header that repeats the section title:
        #   "4.1. Function: Get SBI specification version (FID #0) | Page 14"
        # Left in place these duplicate every heading and split each body in two.
        if re.search(r'\|\s*Page\s+\d+\s*$', line):
            continue
        # TOC entries ("Some Title ......... 15")
        if re.search(r'\.{4,}\s*\d+\s*$', line):
            continue
        if re.search(r'Copyright|RISC-V International|Creative Commons', line, re.IGNORECASE):
            continue
        if re.match(r'^RISC-V SBI Spec', line, re.IGNORECASE):
            continue
        if re.match(r'^Page\s+\d+\s+of\s+\d+', line):
            continue
        if re.match(r'^\d+\s*$', line):
            continue
        out.append(raw)

    return "".join(out)


def _cmd_name(body: str, title: str) -> str:
    """Prefer the sbi_* identifier from the C prototype; fall back to the title."""
    m = _PROTO_PAT.search(body)
    if m:
        return m.group(1).upper()
    cleaned = re.sub(r'\s*\(FID\s*#\d+\)\s*$', '', title).strip()
    slug = re.sub(r'[^A-Z0-9]+', '_', cleaned.upper()).strip('_')
    return f"SBI_{slug[:40]}"


def extract_commands(cleaned_text: str) -> dict:
    """Return {CMD_NAME: section_text} for each v3.0 function section."""
    results = {}
    matches = list(_SEC_PAT.finditer(cleaned_text))

    for idx, m in enumerate(matches):
        chapter, subsec, title = m.group(1), m.group(2), m.group(3).strip()
        if chapter in SKIP_CHAPTERS:
            continue
        # "Function Listing" is a summary table, not a function specification
        if title.lower().startswith("listing"):
            continue

        start = m.start()
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(cleaned_text)
        body = cleaned_text[start:end].strip()

        name = _cmd_name(body, title)
        # Same function split across chapters would otherwise overwrite silently
        if name in results:
            name = f"{name}__{chapter}_{subsec}"
        results[name] = body

    return results


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    txt_path = sys.argv[1]
    out_dir = sys.argv[2] if len(sys.argv) > 2 else os.path.join(
        os.path.dirname(os.path.abspath(__file__)), "sections", "sbi3")

    cleaned = preprocess(txt_path)
    cmds = extract_commands(cleaned)
    if not cmds:
        print("[WARN] No functions found — check that this really is the v3.0 PDF")
        sys.exit(1)

    os.makedirs(out_dir, exist_ok=True)
    for name, text in sorted(cmds.items()):
        with open(os.path.join(out_dir, f"{name}.txt"), "w") as fh:
            fh.write(text)

    print(f"{len(cmds)} functions → {out_dir}")
    for name in sorted(cmds):
        print(f"  {name}")


if __name__ == "__main__":
    main()
