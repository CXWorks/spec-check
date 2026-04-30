#!/usr/bin/env python3
"""
extract_sections_sdei.py

Extract per-command sections from the ARM SDEI (DEN0054C) spec PDF
pre-converted to text via:  pdf2txt -o ccaspec/sdei_1.txt -t text <pdf>

SDEI spec structure (DEN0054C):
    Chapter 5  Functions
      5.1   SDEI functions (subsections 5.1.1–5.1.19)
        5.1.1   SDEI_VERSION
        5.1.2   SDEI_EVENT_REGISTER
        5.1.3   SDEI_EVENT_ENABLE
        5.1.4   SDEI_EVENT_DISABLE
        5.1.5   SDEI_EVENT_CONTEXT
        5.1.6   SDEI_EVENT_COMPLETE
        5.1.7   SDEI_EVENT_COMPLETE_AND_RESUME
        5.1.8   SDEI_EVENT_UNREGISTER
        5.1.9   SDEI_EVENT_STATUS
        5.1.10  SDEI_EVENT_GET_INFO
        5.1.11  SDEI_EVENT_ROUTING_SET  (title missing in PDF text; inferred)
        5.1.12  SDEI_PE_MASK
        5.1.13  SDEI_PE_UNMASK
        5.1.14  SDEI_INTERRUPT_BIND
        5.1.15  SDEI_INTERRUPT_RELEASE
        5.1.16  SDEI_EVENT_SIGNAL
        5.1.17  SDEI_FEATURES
        5.1.18  SDEI_PRIVATE_RESET
        5.1.19  SDEI_SHARED_RESET
      5.2   SDEI event context (not a function spec — skip)
      5.3   Return codes (already in layer1_sdei.rs — skip)

Usage:
    python3 extract_sections_sdei.py sdei_1
    # expects ccaspec/sdei_1.txt → writes sections/sdei_1/
"""

import re
import os
import sys

# Top-level chapter-5 sections to skip (not command specs):
SKIP_SECTIONS = {"5.2", "5.3"}

# Section 5.1.11 title is missing from the PDF text — hardcode it.
MISSING_TITLES = {
    "5.1.11": "SDEI_EVENT_ROUTING_SET",
}


# ---------------------------------------------------------------------------
# Preprocessing
# ---------------------------------------------------------------------------

def preprocess(txt_path: str) -> str:
    """Strip headers, footers, TOC, copyright lines from pdf2txt output."""
    out_lines = []

    with open(txt_path, "r", errors="replace") as fh:
        lines = fh.readlines()

    for raw in lines:
        line = raw.strip()

        if not line:
            continue

        # Copyright notices
        if re.search(r'Copyright.*(?:ARM|Arm|Limited)', line, re.IGNORECASE):
            continue
        # DEN0054 document footers
        if re.match(r'^ARM\s+DEN\s*0054', line, re.IGNORECASE):
            continue
        # "Non-Confidential"
        if re.match(r'^Non-[Cc]onfidential', line):
            continue
        # "Page N of M" or lone page numbers
        if re.match(r'^Page\s+\d+\s+of\s+\d+', line):
            continue
        if re.match(r'^\d+\s*$', line):
            continue
        # TOC entries: "... N"
        if re.search(r'\.{4,}\s*\d+\s*$', line):
            continue
        # Running header
        if line in ("Software Delegated Exception Interface",
                    "Arm SDEI"):
            continue

        out_lines.append(raw)

    return "".join(out_lines)


# ---------------------------------------------------------------------------
# Section splitting
# ---------------------------------------------------------------------------

# Matches 5.1.N sections (not deeper: 5.1.N.M)
# "5.1.3  SDEI_EVENT_ENABLE" — must NOT be followed by ".<digit>"
_SEC_PAT = re.compile(
    r'(?m)^(5\.1\.(\d+))(?!\.\d)\s+(SDEI_\w+)',
)

# Also matches the chapter-level "5.2" / "5.3" sections to find their boundaries
_CHAPTER5_PAT = re.compile(
    r'(?m)^(5\.\d+)(?!\.\d)\s+',
)


def extract_commands(cleaned_text: str) -> dict:
    """
    Return {CMD_NAME: raw_text} for each SDEI function description section.
    """
    results = {}

    # Find all 5.1.N headings (with SDEI_ in title)
    sec_matches = list(_SEC_PAT.finditer(cleaned_text))

    # Also inject missing titles by scanning for "5.1.11" subsections
    # Build a synthetic match position for SDEI_EVENT_ROUTING_SET (5.1.11)
    injected: list[tuple[str, str, int]] = []  # (sec_num, cmd_name, pos)
    for sec_id, cmd_name in MISSING_TITLES.items():
        # Find first subsection (5.1.11.1) to get position
        sub_pat = re.compile(
            rf'(?m)^{re.escape(sec_id)}\.1\b',
        )
        m = sub_pat.search(cleaned_text)
        if m:
            injected.append((sec_id, cmd_name, m.start()))

    # Merge: list of (sec_num, cmd_name, start_pos)
    all_secs: list[tuple[str, str, int]] = []
    for m in sec_matches:
        all_secs.append((m.group(1), m.group(3), m.start()))
    for sec_id, cmd_name, pos in injected:
        # Only add if not already found
        found_nums = {s[0] for s in all_secs}
        if sec_id not in found_nums:
            all_secs.append((sec_id, cmd_name, pos))

    # Sort by position
    all_secs.sort(key=lambda t: t[2])

    # Find boundary of section 5.2 to stop extracting SDEI commands
    stop_pos = len(cleaned_text)
    for m in _CHAPTER5_PAT.finditer(cleaned_text):
        sec = m.group(1)
        if sec in SKIP_SECTIONS:
            stop_pos = min(stop_pos, m.start())

    for i, (sec_num, cmd_name, start) in enumerate(all_secs):
        # Skip sections past the stop boundary
        if start >= stop_pos:
            continue

        end = all_secs[i + 1][2] if i + 1 < len(all_secs) else stop_pos

        # Clamp to stop_pos
        end = min(end, stop_pos)
        body = cleaned_text[start:end]

        # Must contain substantive function description content
        has_content = any(
            kw in body
            for kw in ("Usage", "usage", "Client responsibilities",
                       "Dispatcher responsibilities", "Parameters",
                       "Return", "return code")
        )
        if not has_content:
            continue

        heading = f"{sec_num}  {cmd_name}\n"
        results[cmd_name] = heading + body

    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else ["sdei_1"]
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found — convert DEN0054C PDF first:")
            print(f"       pdf2txt -o ccaspec/{version}.txt -t text <pdf>")
            continue

        out_dir = os.path.join(base_dir, "sections", version)
        os.makedirs(out_dir, exist_ok=True)

        print(f"Processing {version} ...", flush=True)
        cleaned = preprocess(txt_path)

        cmds = extract_commands(cleaned)
        if not cmds:
            print("  [WARN] No commands found — check section numbering in PDF")
        else:
            for cmd_name, raw_text in sorted(cmds.items()):
                fname = f"{cmd_name}_command.txt"
                with open(os.path.join(out_dir, fname), "w") as fh:
                    fh.write(raw_text)
            print(f"  {len(cmds)} commands → sections/{version}/")
            print(f"  Commands: {', '.join(sorted(cmds.keys()))}")

        types_dir = os.path.join(out_dir, "types")
        os.makedirs(types_dir, exist_ok=True)
        print(f"  types/ dir created (SDEI types are in layer1_sdei.rs)")

        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        print(f"  helpers/ dir created (empty)")

    print("Done.")


if __name__ == "__main__":
    main()
