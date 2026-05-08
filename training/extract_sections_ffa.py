#!/usr/bin/env python3
"""
extract_sections_ffa.py

Extract per-function sections from the ARM FF-A (DEN0077A v1.3) spec PDF
pre-converted to text via:  pdf2txt -o ccaspec/ffa_1.txt -t text <pdf>

FF-A function chapters:
  12  Status functions:        FFA_ERROR, FFA_SUCCESS
  13  Setup/Discovery:         FFA_VERSION, FFA_FEATURES, FFA_RX_ACQUIRE,
                               FFA_RX_RELEASE, FFA_RXTX_MAP, FFA_RXTX_UNMAP,
                               FFA_PARTITION_INFO_GET, FFA_PARTITION_INFO_GET_REGS,
                               FFA_ID_GET, FFA_SPM_ID_GET, FFA_CONSOLE_LOG,
                               FFA_NS_RES_INFO_GET, FFA_ABORT
  14  CPU cycle management:    FFA_MSG_WAIT, FFA_YIELD, FFA_RUN,
                               FFA_INTERRUPT, FFA_NORMAL_WORLD_RESUME
  15  Messaging:               FFA_MSG_SEND2, FFA_MSG_SEND_DIRECT_REQ,
                               FFA_MSG_SEND_DIRECT_RESP, FFA_MSG_SEND_DIRECT_REQ2,
                               FFA_MSG_SEND_DIRECT_RESP2
  16  Notifications:           FFA_NOTIFICATION_BITMAP_CREATE,
                               FFA_NOTIFICATION_BITMAP_DESTROY,
                               FFA_NOTIFICATION_BIND, FFA_NOTIFICATION_UNBIND,
                               FFA_NOTIFICATION_SET, FFA_NOTIFICATION_GET,
                               FFA_NOTIFICATION_BIND2, FFA_NOTIFICATION_UNBIND2,
                               FFA_NOTIFICATION_SET2, FFA_NOTIFICATION_GET2,
                               FFA_NOTIFICATION_INFO_GET
  17  Interrupt management:    FFA_EL3_INTR_HANDLE

Section heading format: "NN.M FFA_NAME" — no trailing period, end of line.

Usage:
    python3 extract_sections_ffa.py ffa_1
    # expects ccaspec/ffa_1.txt → writes sections/ffa_1/
"""

import re
import os
import sys

# Only extract sections from chapters 12–17
FFA_CHAPTERS = set(range(12, 18))


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
        # DEN0077 / "Arm Firmware Framework" document footers
        if re.match(r'^DEN0077', line, re.IGNORECASE):
            continue
        if re.match(r'^Arm Firmware Framework', line):
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

        out_lines.append(raw)

    return "".join(out_lines)


# ---------------------------------------------------------------------------
# Section splitting
# ---------------------------------------------------------------------------

# Section headings: "12.2 FFA_ERROR", "16.11 FFA_NOTIFICATION_INFO_GET"
# Chapter headings are like "12 Status functions" (no FFA_ prefix — ignored)
# We match: digits.digits  FFA_WORD
# The section number must start with a chapter in FFA_CHAPTERS (12–17)
_SEC_PAT = re.compile(
    r'(?m)^(\d+)\.(\d+)\s+(FFA_[A-Z_0-9]+)\s*$',
)

# Chapter 18+ as hard stop
_STOP_PAT = re.compile(r'(?m)^18\b')


def extract_functions(cleaned_text: str) -> dict:
    """
    Return {FN_NAME: raw_text} for each FF-A function section.
    """
    matches = list(_SEC_PAT.finditer(cleaned_text))

    if not matches:
        return {}

    # Hard stop at first chapter 18+ that follows the last FFA_ section match
    # (earlier occurrences are in the TOC).
    last_match_end = matches[-1].end()
    stop_pos = len(cleaned_text)
    for m in _STOP_PAT.finditer(cleaned_text):
        if m.start() > last_match_end:
            stop_pos = m.start()
            break

    results = {}

    for i, m in enumerate(matches):
        chapter  = int(m.group(1))
        sec_sub  = m.group(2)
        fn_name  = m.group(3)
        sec_num  = f"{chapter}.{sec_sub}"

        if chapter not in FFA_CHAPTERS:
            continue

        start = m.start()
        if start >= stop_pos:
            continue

        # End: start of next matched section or stop
        end = stop_pos
        for j in range(i + 1, len(matches)):
            nxt_chapter = int(matches[j].group(1))
            if nxt_chapter in FFA_CHAPTERS or nxt_chapter > 17:
                end = matches[j].start()
                break
        end = min(end, stop_pos)

        body = cleaned_text[start:end]

        # Must contain substantive function description content
        has_content = any(
            kw in body
            for kw in ("Description", "Syntax", "Return", "FFA_ERROR",
                       "Parameters", "Register usage", "SUCCESS",
                       "NOT_SUPPORTED", "INVALID_PARAMETERS")
        )
        if not has_content:
            continue

        if fn_name not in results:
            results[fn_name] = body

    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else ["ffa_1"]
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found — convert DEN0077A PDF first:")
            print(f"       pdf2txt -o ccaspec/{version}.txt -t text <pdf>")
            continue

        out_dir = os.path.join(base_dir, "sections", version)
        os.makedirs(out_dir, exist_ok=True)

        print(f"Processing {version} ...", flush=True)
        cleaned = preprocess(txt_path)

        fns = extract_functions(cleaned)
        if not fns:
            print("  [WARN] No functions found — check section numbering in PDF")
        else:
            for fn_name, raw_text in sorted(fns.items()):
                fname = f"{fn_name}_command.txt"
                with open(os.path.join(out_dir, fname), "w") as fh:
                    fh.write(raw_text)
            print(f"  {len(fns)} functions → sections/{version}/")
            print(f"  Functions: {', '.join(sorted(fns.keys()))}")

        types_dir = os.path.join(out_dir, "types")
        os.makedirs(types_dir, exist_ok=True)
        print(f"  types/ dir created (FF-A types are in layer1_ffa.rs)")

        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        print(f"  helpers/ dir created (empty)")

    print("Done.")


if __name__ == "__main__":
    main()
