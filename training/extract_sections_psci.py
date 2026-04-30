#!/usr/bin/env python3
"""
extract_sections_psci.py

Extract per-command sections from the ARM PSCI (DEN0022F.b) spec PDF
pre-converted to text via:  pdf2txt -o ccaspec/psci_13.txt -t text <pdf>

PSCI v1.3 spec structure (DEN0022F.b):
    Chapter 5  Functions
      5.1  Function prototypes  (subsections 5.1.1-5.1.22 — C prototypes only, skipped)
      5.2  Arguments and return values (error codes table — already in layer1_psci.rs)
      5.3  PSCI_VERSION       ← command descriptions start here
      5.4  CPU_SUSPEND
      5.5  CPU_OFF
      5.6  CPU_ON
      5.7  AFFINITY_INFO
      5.8  MIGRATE
      5.9  MIGRATE_INFO_TYPE and MIGRATE_INFO_UP_CPU
      5.10 SYSTEM_OFF
      5.11 SYSTEM_OFF2
      5.12 SYSTEM_RESET
      5.13 SYSTEM_RESET2
      5.14 MEM_PROTECT
      5.15 MEM_PROTECT_CHECK_RANGE
      5.16 PSCI_FEATURES
      5.17 CPU_FREEZE
      5.18 CPU_DEFAULT_SUSPEND
      5.19 NODE_HW_STATE
      5.20 SYSTEM_SUSPEND
      5.21 PSCI_SET_SUSPEND_MODE
      5.22 PSCI_STAT_RESIDENCY/COUNT

No dedicated types chapter — types/return-codes are in layer1_psci.rs.
No helper functions chapter — helpers/ dir is created empty for pipeline compatibility.

Usage:
    python3 extract_sections_psci.py psci_13
    # expects ccaspec/psci_13.txt → writes sections/psci_13/
"""

import re
import os
import sys

# Sections to SKIP (not real command specs):
#   5.1 = function prototypes (C signatures, no semantics)
#   5.2 = register/error-code reference
SKIP_SECTIONS = {"5.1", "5.2"}


# ---------------------------------------------------------------------------
# Preprocessing
# ---------------------------------------------------------------------------

def preprocess(txt_path: str) -> str:
    """Strip headers, footers, TOC, copyright lines from pdf2txt output."""
    out_lines = []

    with open(txt_path, "r", errors="replace") as fh:
        lines = fh.readlines()

    i = 0
    while i < len(lines):
        raw = lines[i]
        line = raw.strip()
        i += 1

        if not line:
            continue

        # Copyright notices
        if re.search(r'Copyright.*(?:ARM|Arm|Limited)', line, re.IGNORECASE):
            continue

        # DEN0022 / document version footers
        if re.match(r'^DEN0022', line):
            continue

        # "Non-Confidential" / version number standing alone
        if re.match(r'^Non-Confidential', line):
            continue
        if re.match(r'^1\.\d+\s*$', line):   # "1.3" alone on a line
            continue

        # TOC entries: "Some Title ........ N"
        if re.search(r'\.{4,}\s*\d+\s*$', line):
            continue

        # "Page N of 96" footers
        if re.match(r'^Page\s+\d+\s+of\s+\d+', line):
            continue

        # Lone page numbers
        if re.match(r'^\d+\s*$', line):
            continue

        # "Power State Coordination Interface" running header
        if line == "Power State Coordination Interface":
            continue

        out_lines.append(raw)

    return "".join(out_lines)


# ---------------------------------------------------------------------------
# Section splitting
# ---------------------------------------------------------------------------

# Matches top-level sections of chapter 5: "5.3  PSCI_VERSION"
# but NOT subsections like "5.3.1  Intended use"
# Pattern: line starts with "5.<digits>" NOT followed by another ".<digits>"
_TOP_SEC_PAT = re.compile(r'(?m)^(5\.\d+)(?!\.\d)\s+(.*?)$')


def extract_commands(cleaned_text: str) -> dict:
    """
    Return {CMD_NAME: raw_text} for each PSCI function description section.
    Targets sections 5.3–5.22; skips 5.1 (prototypes) and 5.2 (arg/return ref).
    """
    results = {}

    matches = list(_TOP_SEC_PAT.finditer(cleaned_text))

    for idx, m in enumerate(matches):
        sec_num   = m.group(1)          # e.g. "5.3"
        sec_title = m.group(2).strip()  # e.g. "PSCI_VERSION"

        if sec_num in SKIP_SECTIONS:
            continue

        # Body: from end of this heading line to start of next same-level section
        start = m.end()
        end   = matches[idx + 1].start() if idx + 1 < len(matches) else len(cleaned_text)
        body  = cleaned_text[start:end]

        # Filter: must look like a real function description
        has_content = any(
            kw in body
            for kw in ("Intended use", "Caller responsibilities",
                       "Implementation responsibilities", "Return codes",
                       "return code", "Preconditions", "Function ID")
        )
        if not has_content:
            continue

        # Derive command name from title
        # "PSCI_VERSION" → "PSCI_VERSION"
        # "CPU_SUSPEND" → "CPU_SUSPEND"
        # "MIGRATE_INFO_TYPE and MIGRATE_INFO_UP_CPU" → split into two
        # "PSCI_STAT_RESIDENCY/COUNT" → split into two
        cmd_names = _parse_cmd_names(sec_title)

        # Full section text (heading + body)
        raw = m.group(0).split("\n", 1)[0] + "\n" + body

        for cmd_name in cmd_names:
            if cmd_name and cmd_name not in results:
                results[cmd_name] = raw

    return results


def _parse_cmd_names(title: str) -> list:
    """
    Extract one or two command names from a section title.
    Handles: "CPU_ON", "MIGRATE_INFO_TYPE and MIGRATE_INFO_UP_CPU",
             "PSCI_STAT_RESIDENCY/COUNT", "SYSTEM_OFF2"
    """
    # "A and B" → [A, B]
    if " and " in title:
        parts = [p.strip() for p in title.split(" and ")]
        return [_clean_name(p) for p in parts]

    # "A/COUNT" → ["A", "A_COUNT"] (e.g. PSCI_STAT_RESIDENCY/COUNT)
    if "/" in title:
        base, suffix = title.rsplit("/", 1)
        base   = _clean_name(base.strip())
        suffix = suffix.strip().upper()
        return [base, base.rsplit("_", 1)[0] + "_" + suffix]

    return [_clean_name(title)]


def _clean_name(s: str) -> str:
    """Uppercase and strip non-identifier characters from a command name token."""
    name = s.split()[0] if s.split() else s
    return re.sub(r'[^A-Z0-9_]', '', name.upper())


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else ["psci_13"]
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found — obtain DEN0022F.b and run:")
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

        # No dedicated types chapter in PSCI — types are in layer1_psci.rs
        types_dir = os.path.join(out_dir, "types")
        os.makedirs(types_dir, exist_ok=True)
        print(f"  types/ dir created (PSCI types are in layer1_psci.rs — no L2 inference needed)")

        # No helper functions chapter either
        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        print(f"  helpers/ dir created (empty — PSCI has no helper functions chapter)")

    print("Done.")


if __name__ == "__main__":
    main()
