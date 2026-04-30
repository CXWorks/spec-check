#!/usr/bin/env python3
"""
extract_sections_sbi.py

Extract per-function sections from the RISC-V SBI (v2.0) spec PDF
pre-converted to text via:  pdftotext -layout ccaspec/riscv-sbi.pdf ccaspec/sbi_2.txt

SBI v2.0 spec structure (riscv-sbi.pdf):
    Chapter 1  Introduction
    Chapter 2  Binary Encoding
    Chapter 3  Base Extension (EID #0x10)
      3.1  Get SBI specification version
      3.2  Get SBI implementation ID
      3.3  Get SBI implementation version
      3.4  Probe SBI extension
      3.5  Get machine vendor ID
      3.6  Get machine architecture ID
      3.7  Get machine implementation ID
    Chapter 4  Legacy Extensions (EID 0x00-0x0F)  — skip
    Chapter 5  Timer Extension (EID #0x54494D45)
      5.1  Function: sbi_set_timer
    Chapter 6  IPI Extension (EID #0x735049)
      6.1  Function: sbi_send_ipi
    Chapter 7  RFENCE Extension (EID #0x52464E43)
      7.1-7.5  Remote fence functions
    Chapter 8  HSM Extension (EID #0x48534D)
      8.1-8.4  Hart state management functions
    Chapter 9  System Reset Extension (EID #0x53525354)
      9.1  Function: sbi_system_reset
    Chapter 10  Performance Monitoring Unit (EID #0x504D55)
      10.1-10.6  PMU counter functions
    Chapter 11  Debug Console Extension (EID #0x4442434E)
      11.1-11.2  Debug console read/write
    Chapter 12  NACL Extension
      12.1-12.2  NACL functions

Usage:
    python3 extract_sections_sbi.py sbi_2
    # expects ccaspec/sbi_2.txt → writes sections/sbi_2/
"""

import re
import os
import sys

# Chapters to SKIP (not function specifications):
#   1  = Introduction
#   2  = Binary Encoding (EID/FID format)
#   4  = Legacy Extensions (deprecated)
SKIP_CHAPTERS = {"1", "2", "4"}

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

        # Copyright / license lines
        if re.search(r'Copyright|RISC-V International|Creative Commons', line, re.IGNORECASE):
            continue

        # Document version footers  ("RISC-V SBI Specification", "Ratified", etc.)
        if re.match(r'^RISC-V SBI Spec', line, re.IGNORECASE):
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

# Matches top-level chapter sections like:
#   "3.1  Get SBI specification version"
#   "5.1  Function: sbi_set_timer"
#   "8.1  Function: sbi_hart_start"
# Pattern: N.M  <title>  (where N is chapter number, M is subsection)
# We capture both "N.M" sections and "N.M.P" subsections but group by "N.M"
_TOP_SEC_PAT = re.compile(
    r'(?m)^(\d+)\.(\d+)(?!\.\d)\s+(.*?)$'
)


def section_to_cmd_name(chapter: str, subsec: str, title: str) -> str:
    """
    Derive a canonical SBI command name from chapter, subsection, and title.

    SBI function names follow the pattern "sbi_<verb>_<object>" in the spec.
    We try to extract from the title; fall back to chapter-based naming.
    """
    # Strip "Function: " prefix
    title_clean = re.sub(r'^Function:\s*', '', title, flags=re.IGNORECASE).strip()

    # If title contains an sbi_ identifier, use it
    m = re.search(r'\b(sbi_\w+)', title_clean, re.IGNORECASE)
    if m:
        return m.group(1).upper()

    # Chapter-based fallback using known function lists
    chapter_map = {
        ("3", "1"): "SBI_GET_SPEC_VERSION",
        ("3", "2"): "SBI_GET_IMPL_ID",
        ("3", "3"): "SBI_GET_IMPL_VERSION",
        ("3", "4"): "SBI_PROBE_EXTENSION",
        ("3", "5"): "SBI_GET_MVENDORID",
        ("3", "6"): "SBI_GET_MARCHID",
        ("3", "7"): "SBI_GET_MIMPID",
        ("5", "1"): "SBI_SET_TIMER",
        ("6", "1"): "SBI_SEND_IPI",
        ("7", "1"): "SBI_REMOTE_FENCE_I",
        ("7", "2"): "SBI_REMOTE_SFENCE_VMA",
        ("7", "3"): "SBI_REMOTE_SFENCE_VMA_ASID",
        ("7", "4"): "SBI_REMOTE_HFENCE_GVMA_VMID",
        ("7", "5"): "SBI_REMOTE_HFENCE_VVMA",
        ("8", "1"): "SBI_HART_START",
        ("8", "2"): "SBI_HART_STOP",
        ("8", "3"): "SBI_HART_GET_STATUS",
        ("8", "4"): "SBI_HART_SUSPEND",
        ("9", "1"): "SBI_SYSTEM_RESET",
        ("10", "1"): "SBI_PMU_NUM_COUNTERS",
        ("10", "2"): "SBI_PMU_COUNTER_GET_INFO",
        ("10", "3"): "SBI_PMU_COUNTER_CONFIG_MATCHING",
        ("10", "4"): "SBI_PMU_COUNTER_START",
        ("10", "5"): "SBI_PMU_COUNTER_STOP",
        ("10", "6"): "SBI_PMU_COUNTER_FW_READ",
        ("11", "1"): "SBI_DEBUG_CONSOLE_WRITE",
        ("11", "2"): "SBI_DEBUG_CONSOLE_READ",
        ("12", "1"): "SBI_NACL_PROBE_FEATURE",
        ("12", "2"): "SBI_NACL_SETUP_SHMEM",
    }
    key = (chapter, subsec)
    if key in chapter_map:
        return chapter_map[key]

    # Generic fallback
    name = re.sub(r'[^A-Z0-9_]', '_',
                  title_clean.upper().replace(' ', '_').replace('-', '_'))
    return f"SBI_{name[:40]}"


def extract_commands(cleaned_text: str) -> dict:
    """
    Return {CMD_NAME: raw_text} for each SBI function description section.
    """
    results = {}

    matches = list(_TOP_SEC_PAT.finditer(cleaned_text))

    for idx, m in enumerate(matches):
        chapter   = m.group(1)   # e.g. "8"
        subsec    = m.group(2)   # e.g. "1"
        title     = m.group(3).strip()  # e.g. "Function: sbi_hart_start"

        if chapter in SKIP_CHAPTERS:
            continue

        # Body: from end of this heading to start of next same-level section
        start = m.end()
        end   = matches[idx + 1].start() if idx + 1 < len(matches) else len(cleaned_text)
        body  = cleaned_text[start:end]

        # Filter: must look like a real function description
        has_content = any(
            kw in body
            for kw in ("Return value", "return value", "SBI_SUCCESS",
                       "sbi_ecall", "Parameters", "parameter",
                       "Description", "Errors", "error code",
                       "hart_id", "Function ID", "FID")
        )
        if not has_content:
            continue

        cmd_name = section_to_cmd_name(chapter, subsec, title)

        if cmd_name and cmd_name not in results:
            raw = m.group(0).split("\n", 1)[0] + "\n" + body
            results[cmd_name] = raw

    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else ["sbi_2"]
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found — obtain RISC-V SBI PDF and run:")
            print(f"       pdftotext -layout ccaspec/riscv-sbi.pdf ccaspec/{version}.txt")
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

        # No dedicated types chapter in SBI — types are in layer1_sbi.rs
        types_dir = os.path.join(out_dir, "types")
        os.makedirs(types_dir, exist_ok=True)
        print(f"  types/ dir created (SBI types are in layer1_sbi.rs — no L2 inference needed)")

        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        print(f"  helpers/ dir created (empty — SBI has no helper functions chapter)")

    print("Done.")


if __name__ == "__main__":
    main()
