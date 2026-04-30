#!/usr/bin/env python3
"""
extract_sections_scmi.py

Extract per-command sections from the ARM SCMI (DEN0056F v4.0) spec PDF
pre-converted to text via:  pdf2txt -o ccaspec/scmi_4.txt -t text <pdf>

Scope: Base Protocol (§3.2) and Power Domain Protocol (§3.3) only.

Section pattern (4-level): "3.2.2.1 PROTOCOL_VERSION"
  3.2.2.N  — Base Protocol commands
  3.2.3.N  — Base Protocol events
  3.3.2.N  — Power Domain Protocol commands
  3.3.3.N  — Power Domain Protocol events

Shared commands (PROTOCOL_VERSION, NEGOTIATE_PROTOCOL_VERSION,
PROTOCOL_ATTRIBUTES, PROTOCOL_MESSAGE_ATTRIBUTES) appear in both 3.2 and
3.3. Only the first (Base) occurrence is kept; duplicates are prepended
with the protocol prefix in cleanup_scmi.py's COMMANDS list.

Usage:
    python3 extract_sections_scmi.py scmi_4
    # expects ccaspec/scmi_4.txt → writes sections/scmi_4/
"""

import re
import os
import sys


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
        # DEN0056 document footers
        if re.match(r'^DEN0056', line, re.IGNORECASE):
            continue
        if re.match(r'^Arm®', line):
            continue
        # "Non-Confidential" / BET0 markers
        if re.match(r'^Non-[Cc]onfidential', line):
            continue
        if re.match(r'^B\s*E\s*T\s*0', line):
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

# Matches 4-level sections within Base and Power Domain protocols:
# "3.2.2.1 PROTOCOL_VERSION", "3.3.3.2 POWER_STATE_CHANGE_REQUESTED"
_CMD_PAT = re.compile(
    r'(?m)^(3\.[23]\.[23]\.(\d+))\s+([A-Z][A-Z0-9_]+)',
)

# Matches start of chapter 3.4+ to use as hard stop
_NEXT_CHAPTER_PAT = re.compile(r'(?m)^3\.[4-9](?!\.\d)\s+')


def extract_commands(cleaned_text: str) -> dict:
    """
    Return {CMD_NAME: raw_text} for each in-scope SCMI function.
    Keeps first occurrence by name (Base protocol takes precedence over
    Power Domain for shared generic commands).
    """
    matches = list(_CMD_PAT.finditer(cleaned_text))

    if not matches:
        return {}

    # Hard stop: first occurrence of chapter 3.4+ that comes AFTER
    # the last command match (to skip any earlier TOC occurrences).
    last_match_end = matches[-1].end()
    stop_pos = len(cleaned_text)
    for m in _NEXT_CHAPTER_PAT.finditer(cleaned_text):
        if m.start() > last_match_end:
            stop_pos = m.start()
            break

    results = {}

    for i, m in enumerate(matches):
        sec_num  = m.group(1)   # "3.2.2.1"
        cmd_name = m.group(3)   # "PROTOCOL_VERSION"

        start = m.start()
        if start >= stop_pos:
            continue

        end = matches[i + 1].start() if i + 1 < len(matches) else stop_pos
        end = min(end, stop_pos)

        body = cleaned_text[start:end]

        # Must contain return values / status table content
        has_content = any(
            kw in body
            for kw in ("Return values", "return value", "Status code",
                       "SUCCESS", "NOT_SUPPORTED", "INVALID_PARAMETERS",
                       "Parameters", "Description")
        )
        if not has_content:
            continue

        # Deduplicate: first occurrence of each name wins
        if cmd_name not in results:
            results[cmd_name] = body

    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else ["scmi_4"]
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found — convert DEN0056F PDF first:")
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
        print(f"  types/ dir created (SCMI types are in layer1_scmi.rs)")

        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        print(f"  helpers/ dir created (empty)")

    print("Done.")


if __name__ == "__main__":
    main()
