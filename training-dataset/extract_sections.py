#!/usr/bin/env python3
"""
extract_sections.py

Reuses SCOPE's PDF-preprocessing and section-splitting logic to extract, for each
RMM command, the raw spec text (between its heading and the next sibling heading).

Output: sections/{version}/{cmd_name}.txt
        e.g. sections/eac5/RMI_DATA_DESTROY_command.txt

Usage:
    python3 extract_sections.py eac5 rel0 alp11 alp12
    # or: python3 extract_sections.py   (defaults to all four versions)
"""

import re
import os
import sys

VERSIONS = ["eac5", "rel0", "alp11", "alp12", "alp13", "alp14"]


# ---------------------------------------------------------------------------
# Preprocessing — identical to lines 2273-2355 of scope
# ---------------------------------------------------------------------------
def preprocess(txt_path: str) -> str:
    """Return cleaned text (headers/footers/ToC stripped) as a single string."""
    is_header = False
    is_link = False
    is_contents = False
    is_release_info = False

    draft_pattern  = re.compile(r"\s+T\s+")
    draft_pattern2 = re.compile(r"\s+AF\s+")
    draft_pattern3 = re.compile(r"\s+R\s+")
    draft_pattern4 = re.compile(r"\s+D\s+")

    out_lines = []
    with open(txt_path, "r") as fh:
        file_iter = iter(fh)
        for raw_line in file_iter:
            line = raw_line.strip()

            # remove DRAF'T' in the middle of contents (alpha versions)
            if raw_line.endswith(" T\n"):
                raw_line = raw_line[:-2].rstrip() + "\n"
            # remove DRAFT lines (alpha)
            if (draft_pattern.match(raw_line) or draft_pattern2.match(raw_line) or
                    draft_pattern3.match(raw_line) or draft_pattern4.match(raw_line)):
                continue
            # remove empty lines
            if line == "" or raw_line == "\n":
                continue
            # remove headers
            if "Chapter" in line and "." in line:
                is_header = True
                continue
            if is_header:
                is_header = False
                continue
            # remove footers
            if "Copyright" in line:
                continue
            if ("1.0-eac5" in line or "1.0-rel0" in line or "1.1-alp11" in line or
                    "1.1-alp12" in line or "1.1-alp13" in line or "1.1-alp14" in line):
                continue
            # remove inner links
            if "See also:" in line:
                is_link = True
                continue
            if "architecture:" in line:
                is_link = True
                continue
            if is_link:
                if "•" in line:
                    continue
                else:
                    is_link = False
            # remove release information and license
            if "Release information" in line:
                next_line = next(file_iter, "")
                if "-20" in next_line:
                    is_release_info = True
                    continue
            if is_release_info:
                if "England CB1 9NJ" in line:
                    is_release_info = False
                continue
            # remove contents
            if "Contents" in line:
                next_line = next(file_iter, "")
                if "Realm Management Monitor specification" in next_line:
                    is_contents = True
                    continue
            if is_contents:
                if "Glossary" in line:
                    is_contents = False
                continue
            # remove exceptional figures
            if "Figure B1.1" in line:
                continue

            out_lines.append(raw_line)

    return "".join(out_lines)


# ---------------------------------------------------------------------------
# Section extraction — mirrors lines 2357-2414 of scope
# ---------------------------------------------------------------------------
def extract_types(cleaned_text: str) -> dict:
    """
    Return {type_name: raw_text} for every enum/struct/fieldset section
    found in the Types (Part C) and Architecture type sub-sections (Part A).

    raw_text is the full section body (heading reconstructed).
    """
    results = {}

    part_pat       = re.compile(r"(Part [A-Z])")
    chapter_pat    = re.compile(r"(Chapter [A-Z]\d)")
    section_pat    = re.compile(r"([A-Z]\d\.\d+\s+)")

    def split_with_headings(text, pattern):
        tokens = pattern.split(text)
        pairs = [("", tokens[0])]
        for i in range(1, len(tokens), 2):
            pairs.append((tokens[i], tokens[i + 1] if i + 1 < len(tokens) else ""))
        return pairs

    part_pairs = split_with_headings(cleaned_text, part_pat)

    for part_heading, part_body in part_pairs:
        part_text = part_heading + part_body

        # Types part (Part C) and Architecture part (Part A, for RMI/RSI/PSCI types)
        is_types_part = "Types" in part_text[:200] or "Constants and types" in part_text[:200]
        is_arch_part  = "Architecture" in part_text[:200]
        if not (is_types_part or is_arch_part):
            continue

        chapter_pairs = split_with_headings(part_body, chapter_pat)
        for ch_heading, ch_body in chapter_pairs:
            chapter_text = ch_heading + ch_body

            # In arch part, only collect from "RMI types", "RSI types", "PSCI types"
            if is_arch_part:
                if not any(t in chapter_text for t in ("RMI types", "RSI types", "PSCI types")):
                    continue

            section_pairs = split_with_headings(ch_body, section_pat)
            for sec_heading, sec_body in section_pairs:
                sec_text = sec_heading + sec_body
                # Each section is one type definition
                if not any(kw in sec_text[:400]
                           for kw in ("enumeration", "structure", "fieldset")):
                    continue

                sec_lines = sec_body.splitlines()
                if not sec_lines:
                    continue
                type_name = sec_lines[0].strip().split()[0] if sec_lines[0].strip() else None
                if not type_name:
                    continue
                # Avoid duplicates (arch part may repeat types already in Types part)
                if type_name not in results:
                    results[type_name] = sec_heading + sec_body

    return results


def extract_helper_fns(cleaned_text: str) -> dict:
    """
    Return {fn_name: raw_text} for every B3.x section in the
    'Command condition functions' chapter of the Interface part.
    """
    results = {}

    part_pat    = re.compile(r"(Part [A-Z])")
    chapter_pat = re.compile(r"(Chapter [A-Z]\d)")
    section_pat = re.compile(r"([A-Z]\d\.\d+\s+)")

    def split_with_headings(text, pattern):
        tokens = pattern.split(text)
        pairs = [("", tokens[0])]
        for i in range(1, len(tokens), 2):
            heading = tokens[i]
            body    = tokens[i + 1] if i + 1 < len(tokens) else ""
            pairs.append((heading, body))
        return pairs

    part_pairs = split_with_headings(cleaned_text, part_pat)

    for part_heading, part_body in part_pairs:
        part_text = part_heading + part_body
        if "Interface" not in part_text:
            continue

        chapter_pairs = split_with_headings(part_body, chapter_pat)

        for ch_heading, ch_body in chapter_pairs:
            chapter_text = ch_heading + ch_body
            if "Command condition functions" not in chapter_text:
                continue

            section_pairs = split_with_headings(ch_body, section_pat)

            for sec_heading, sec_body in section_pairs:
                sec_lines = sec_body.splitlines()
                if not sec_lines:
                    continue
                first_line = sec_lines[0].strip()
                if not first_line or "function" not in first_line:
                    continue
                fn_name = first_line.split()[0]
                if fn_name and fn_name not in results:
                    results[fn_name] = sec_heading + sec_body

    return results


def extract_commands(cleaned_text: str) -> dict:
    """
    Return {cmd_title: raw_text} for every sub-section whose title ends with
    ' command', extracted from the Interface part of the spec.

    raw_text includes the sub-section heading number that was consumed by
    re.split(), reconstructed via a capturing-group split so downstream
    readers have the full context.
    """
    results = {}

    # Capturing split helpers so we can reconstruct headings
    part_pat        = re.compile(r"(Part [A-Z])")
    chapter_pat     = re.compile(r"(Chapter [A-Z]\d)")
    section_pat     = re.compile(r"([A-Z]\d\.\d+\s+)")
    subsection_pat  = re.compile(r"([A-Z]\d\.\d+\.\d+\s+)")
    subsubsect_pat  = re.compile(r"([A-Z]\d\.\d+\.\d+\.\d+\s+)")

    def split_with_headings(text, pattern):
        """Split text on pattern, returning list of (heading, body) tuples."""
        tokens = pattern.split(text)
        # tokens = [pre, heading1, body1, heading2, body2, ...]
        pairs = [("", tokens[0])]
        for i in range(1, len(tokens), 2):
            heading = tokens[i]
            body    = tokens[i + 1] if i + 1 < len(tokens) else ""
            pairs.append((heading, body))
        return pairs

    part_pairs = split_with_headings(cleaned_text, part_pat)

    for part_heading, part_body in part_pairs:
        part_text = part_heading + part_body

        if "Interface" not in part_text:
            continue

        chapter_pairs = split_with_headings(part_body, chapter_pat)

        for ch_heading, ch_body in chapter_pairs:
            chapter_text = ch_heading + ch_body

            is_rmi   = "Realm Management Interface" in chapter_text
            is_rsi   = "Realm Services Interface" in chapter_text
            is_psci  = "Power State Control Interface" in chapter_text
            if not (is_rmi or is_rsi or is_psci):
                continue

            section_pairs = split_with_headings(ch_body, section_pat)

            for sec_heading, sec_body in section_pairs:
                section_text = sec_heading + sec_body

                is_cmd_section = (
                    (is_rmi  and "RMI commands"  in section_text) or
                    (is_rsi  and "RSI commands"  in section_text) or
                    (is_psci and "PSCI commands" in section_text)
                )
                if not is_cmd_section:
                    continue

                # Split into command sub-sections
                subsec_pairs = split_with_headings(sec_body, subsection_pat)

                for sub_heading, sub_body in subsec_pairs:
                    sub_lines = sub_body.splitlines()
                    if not sub_lines:
                        continue
                    sub_title = sub_lines[0].strip()

                    if not sub_title.endswith(" command"):
                        continue

                    # Reconstruct full raw text (heading + body with sub-sub headings intact)
                    raw_text = sub_heading + sub_body
                    results[sub_title] = raw_text

    return results


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else VERSIONS
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        txt_path = os.path.join(base_dir, "ccaspec", f"{version}.txt")
        if not os.path.exists(txt_path):
            print(f"[WARN] {txt_path} not found, skipping {version}", flush=True)
            continue

        out_dir = os.path.join(base_dir, "sections", version)
        os.makedirs(out_dir, exist_ok=True)

        print(f"Processing {version}...", flush=True)
        cleaned = preprocess(txt_path)
        cmds = extract_commands(cleaned)

        if not cmds:
            print(f"  [WARN] No commands found for {version}!")
            continue

        for cmd_title, raw_text in sorted(cmds.items()):
            # e.g. "RMI_DATA_DESTROY command" → "RMI_DATA_DESTROY_command"
            filename = cmd_title.replace(" ", "_") + ".txt"
            out_path = os.path.join(out_dir, filename)
            with open(out_path, "w") as fh:
                fh.write(raw_text)

        print(f"  Extracted {len(cmds)} commands → sections/{version}/")

        # Extract type sections
        types = extract_types(cleaned)
        if types:
            types_dir = os.path.join(out_dir, "types")
            os.makedirs(types_dir, exist_ok=True)
            for type_name, raw_text in sorted(types.items()):
                out_path = os.path.join(types_dir, f"{type_name}.txt")
                with open(out_path, "w") as fh:
                    fh.write(raw_text)
            print(f"  Extracted {len(types)} types → sections/{version}/types/")

        # Extract helper function sections
        helpers = extract_helper_fns(cleaned)
        if helpers:
            helpers_dir = os.path.join(out_dir, "helpers")
            os.makedirs(helpers_dir, exist_ok=True)
            for fn_name, raw_text in sorted(helpers.items()):
                out_path = os.path.join(helpers_dir, f"{fn_name}.txt")
                with open(out_path, "w") as fh:
                    fh.write(raw_text)
            print(f"  Extracted {len(helpers)} helper fns → sections/{version}/helpers/")

    print("Done.")


if __name__ == "__main__":
    main()
