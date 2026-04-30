#!/usr/bin/env python3
"""
extract_sections_tdx.py

Extract per-function sections from the Intel TDX Module ABI (v1.5) spec PDF
pre-converted to text via:
    pdftotext -layout ccaspec/intel-tdx-abi.pdf ccaspec/tdx_15.txt

TDX ABI v1.5 structure:
    Chapters 1–13  Background, architecture, data structures (skip)
    Chapter 14     TDH.* (host-side TDCALL leaf functions)
      14.1  TDH.MNG.CREATE
      14.2  TDH.MNG.ADDCX
      ...
    Chapter 15     TDG.* (guest-side TDCALL leaf functions)
      15.1  TDG.VP.VMCALL
      15.2  TDG.VP.INFO
      ...

NOTE: Section headings in the body use the form "14.N  TDH.MNG.CREATE" etc.
We identify command boundaries from these N.M headings in chapters 14 and 15.

Usage:
    python3 extract_sections_tdx.py tdx_15
    # expects ccaspec/tdx_15.txt → writes sections/tdx_15/
"""

import re
import os
import sys

# Chapters to SKIP (non-function chapters)
SKIP_CHAPTERS = set(str(i) for i in range(1, 14))  # skip 1–13

# ---------------------------------------------------------------------------
# Preprocessing
# ---------------------------------------------------------------------------

def preprocess(txt_path: str) -> str:
    """Strip headers, footers, TOC, copyright lines from pdftotext output."""
    out_lines = []

    with open(txt_path, "r", errors="replace") as fh:
        lines = fh.readlines()

    for raw in lines:
        line = raw.strip()

        if not line:
            continue

        # Intel copyright / legal notices
        if re.search(r'Intel Corporation|All rights reserved|No license.*patent',
                     line, re.IGNORECASE):
            continue

        # Document version/number footers
        if re.match(r'^348551', line):   # Intel doc number
            continue
        if re.match(r'^Revision\s+\d', line, re.IGNORECASE):
            continue

        # "Page N of M" or lone page numbers
        if re.match(r'^Page\s+\d+\s+of\s+\d+', line):
            continue
        if re.match(r'^\d+\s*$', line):
            continue

        # TOC entries: "Some Title ........ N"
        if re.search(r'\.{4,}\s*\d+\s*$', line):
            continue

        out_lines.append(raw)

    return "".join(out_lines)


# ---------------------------------------------------------------------------
# Section splitting
# ---------------------------------------------------------------------------

# Matches section headings in chapters 14 and 15:
#   "14.1  TDH.MNG.CREATE"
#   "15.3  TDG.MR.REPORT"
# Pattern: (14|15).N  TDH.*/TDG.*
_TOP_SEC_PAT = re.compile(
    r'(?m)^(1[45])\.(\d+)(?!\.\d)\s+(TD[HG]\.[\w.]+.*?)$'
)


def section_to_cmd_name(chapter: str, title: str) -> str:
    """
    Convert a TDX section title to a canonical command name.
    "TDH.MNG.CREATE" → "TDH_MNG_CREATE"
    "TDG.VP.VMCALL"  → "TDG_VP_VMCALL"
    """
    # Extract the TDH/TDG identifier
    m = re.search(r'(TD[HG]\.[\w.]+)', title)
    if m:
        return m.group(1).replace('.', '_').upper()

    # Fallback: sanitize the title
    name = re.sub(r'[^A-Z0-9_]', '_', title.upper().replace('.', '_'))
    return name[:50].rstrip('_')


def extract_commands(cleaned_text: str) -> dict:
    """
    Return {CMD_NAME: raw_text} for each TDX leaf function section.
    """
    results = {}

    matches = list(_TOP_SEC_PAT.finditer(cleaned_text))

    for idx, m in enumerate(matches):
        chapter   = m.group(1)   # "14" or "15"
        title     = m.group(3).strip()  # "TDH.MNG.CREATE"

        if chapter in SKIP_CHAPTERS:
            continue

        # Body: from end of this heading to start of next same-level section
        start = m.end()
        end   = matches[idx + 1].start() if idx + 1 < len(matches) else len(cleaned_text)
        body  = cleaned_text[start:end]

        # Filter: must look like a real function spec
        has_content = any(
            kw in body
            for kw in ("TDX_SUCCESS", "TDX_OPERAND", "Error Code",
                       "Preconditions", "Leaf", "RCX", "RBX",
                       "Output Operands", "Input Operands",
                       "TDCALL", "TD management", "Description")
        )
        if not has_content:
            continue

        cmd_name = section_to_cmd_name(chapter, title)

        if cmd_name and cmd_name not in results:
            raw = m.group(0).split("\n", 1)[0] + "\n" + body
            results[cmd_name] = raw

    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else ["tdx_15"]
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found — obtain Intel TDX ABI v1.5 PDF and run:")
            print(f"       pdftotext -layout ccaspec/intel-tdx-abi.pdf ccaspec/{version}.txt")
            continue

        out_dir = os.path.join(base_dir, "sections", version)
        os.makedirs(out_dir, exist_ok=True)

        print(f"Processing {version} ...", flush=True)
        cleaned = preprocess(txt_path)

        cmds = extract_commands(cleaned)
        if not cmds:
            print("  [WARN] No commands found — check chapter/section numbering in PDF")
        else:
            for cmd_name, raw_text in sorted(cmds.items()):
                fname = f"{cmd_name}_command.txt"
                with open(os.path.join(out_dir, fname), "w") as fh:
                    fh.write(raw_text)
            print(f"  {len(cmds)} commands → sections/{version}/")
            print(f"  Commands: {', '.join(sorted(cmds.keys()))}")

        # TDX types are in layer1_tdx.rs; no separate types chapter to process
        types_dir = os.path.join(out_dir, "types")
        os.makedirs(types_dir, exist_ok=True)
        print(f"  types/ dir created (TDX types are in layer1_tdx.rs — no L2 inference needed)")

        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        print(f"  helpers/ dir created (empty)")

    print("Done.")


if __name__ == "__main__":
    main()
