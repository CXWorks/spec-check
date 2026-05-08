#!/usr/bin/env python3
"""
extract_sections_drtm.py

Extract per-command sections from the ARM DRTM (DEN0113 v1.4) spec PDF
pre-converted to text via:  pdf2txt -o ccaspec/drtm_1.txt -t text <pdf>

DRTM structure (DEN0113 v1.4):
    Chapter 3  Interface functions and data structures
      3.1   Introduction
      3.2   DRTM_VERSION        (subsections: 3.2.1, 3.2.2)
      3.3   DRTM_FEATURES       (subsections: 3.3.1)
      3.4   DRTM_DYNAMIC_LAUNCH (subsections: 3.4.1, 3.4.3)
      3.5   DRTM_UNPROTECT_MEMORY
      3.6   DRTM_CLOSE_LOCALITY
      3.7   DRTM_GET_ERROR
      3.8   DRTM_SET_ERROR
      3.9   DRTM_SET_TCB_HASH
      3.10  DRTM_LOCK_TCB_HASHES
      3.11  DRTM_ENABLE_SECURE_INTERRUPTS
      3.12  DRTM error encoding        ← skip (return codes)
      3.13+ data structures            ← skip

  NOTE: Top-level "3.N  COMMAND_NAME" headings exist only in the TOC, not
  in the body text.  The body uses subsection headings of the form
  "3.N.M  DRTM_COMMAND_NAME  <usage|caller|implementation>".
  We identify command boundaries by the first occurrence of "3.N.1".

Usage:
    python3 extract_sections_drtm.py drtm_1
    # expects ccaspec/drtm_1.txt → writes sections/drtm_1/
"""

import re
import os
import sys

# Subsections to SKIP:
#   3.1  = introduction
#   3.12+ = error encoding, data structures, return codes
SKIP_PARENT = {
    "3.1",
    "3.12", "3.13", "3.14", "3.15", "3.16", "3.17", "3.18",
}

# ---------------------------------------------------------------------------
# Preprocessing
# ---------------------------------------------------------------------------

def preprocess(txt_path: str) -> str:
    """Strip headers, footers, TOC, copyright lines from pdf2txt output."""
    out_lines = []

    with open(txt_path, "r", errors="replace") as fh:
        lines = fh.readlines()

    in_toc = False
    for raw in lines:
        line = raw.strip()

        if not line:
            continue

        # Copyright notices
        if re.search(r'Copyright.*(?:ARM|Arm|Limited)', line, re.IGNORECASE):
            continue
        # DEN0113 / document footers
        if re.match(r'^DEN\s*0113', line, re.IGNORECASE):
            continue
        # "Non-confidential"
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

        out_lines.append(raw)

    return "".join(out_lines)


# ---------------------------------------------------------------------------
# Section splitting
# ---------------------------------------------------------------------------

# Matches subsection headings: "3.N.M  DRTM_COMMAND ..."
_SUB_SEC_PAT = re.compile(
    r'(?m)^(3\.(\d+))\.(\d+)\s+(DRTM_\w+)',
)

# Pattern for data-structure / return-code sections that should terminate
# the command extraction (3.12 DRTM error encoding, 3.14+)
_STOP_PAT = re.compile(r'(?m)^3\.1[2-9]\b')


def extract_commands(cleaned_text: str) -> dict:
    """
    Return {CMD_NAME: raw_text} for each DRTM command section.
    Discovers commands from subsection headings; groups by parent section (3.N).
    Only uses matches that appear in the CONTENT body (not the TOC at the top),
    detected by requiring a minimum body size > 100 chars.
    """
    matches = list(_SUB_SEC_PAT.finditer(cleaned_text))

    if not matches:
        return {}

    # Collect (parent_num, cmd_name, match_start).
    # Multiple occurrences exist (TOC + body); we want the body occurrence.
    # Strategy: group all occurrences by parent, then pick the one whose
    # subsequent body is longest (the TOC occurrence has a near-empty body).
    parent_occurrences: dict[str, list[tuple[str, int]]] = {}
    for m in matches:
        parent = m.group(1)   # "3.N"
        cmd    = m.group(4)   # "DRTM_VERSION"
        parent_occurrences.setdefault(parent, []).append((cmd, m.start()))

    if not parent_occurrences:
        return {}

    # For each parent, pick the occurrence with the largest gap to the next
    # occurrence of ANY subsection (proxy for "has real content after it").
    # We build a sorted list of all match positions to compute gaps.
    all_positions = sorted(m.start() for m in matches)

    def body_gap(pos: int) -> int:
        """Chars until the next subsection heading after pos."""
        for p in all_positions:
            if p > pos:
                return p - pos
        return len(cleaned_text) - pos

    parent_info: dict[str, tuple[str, int]] = {}
    for parent, occurrences in parent_occurrences.items():
        best = max(occurrences, key=lambda t: body_gap(t[1]))
        parent_info[parent] = best

    # Sort by start position (body occurrences will be in chapter order)
    ordered = sorted(parent_info.items(), key=lambda kv: kv[1][1])

    # Hard stop: first 3.12+ heading that appears AFTER the last command start
    last_cmd_pos = max(v[1] for v in parent_info.values())
    stop_pos = len(cleaned_text)
    for m in _STOP_PAT.finditer(cleaned_text):
        if m.start() > last_cmd_pos:
            stop_pos = m.start()
            break

    results = {}
    for i, (parent, (cmd_name, start)) in enumerate(ordered):
        if parent in SKIP_PARENT:
            continue

        end = ordered[i + 1][1][1] if i + 1 < len(ordered) else stop_pos
        end = min(end, stop_pos)
        body = cleaned_text[start:end]

        # Require minimum body size to reject TOC entries
        if len(body) < 150:
            continue

        # Must contain substantive content
        has_content = any(
            kw in body
            for kw in ("usage", "Usage", "implementation", "caller",
                       "Return", "return", "SUCCESS", "NOT_SUPPORTED")
        )
        if not has_content:
            continue

        heading = f"{parent}  {cmd_name}\n"
        results[cmd_name] = heading + body

    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else ["drtm_1"]
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found — convert DEN0113 PDF first:")
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
        print(f"  types/ dir created (DRTM types are in layer1_drtm.rs)")

        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        print(f"  helpers/ dir created (empty)")

    print("Done.")


if __name__ == "__main__":
    main()
