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

# The LAST section of chapter 5 has no following "5.N" heading to stop at, so it
# ran to end-of-document and swallowed chapters 6-7 and the appendices: 5.22 came
# out at 1460 lines instead of 71, and both commands that section defines
# (PSCI_STAT_RESIDENCY, PSCI_STAT_COUNT) carried all of it. Stop instead at the
# first numbered section of a later chapter. Requiring a non-space after the
# number is what keeps the surviving table-of-contents fragment "6.4 " -- a bare
# number on its own line, title wrapped to the next -- from ending the section at
# the top of the document.
_NEXT_CHAPTER_PAT = re.compile(r'(?m)^\s*[6-9]\.\d+(?!\.\d)\s+\S')


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
        if idx + 1 < len(matches):
            end = matches[idx + 1].start()
        else:
            nxt = _NEXT_CHAPTER_PAT.search(cleaned_text, start)
            end = nxt.start() if nxt else len(cleaned_text)
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


# Subsections of 5.1: "5.1.4  CPU_ON". These carry the parameter list, the return
# declarations and the Function ID -- the things an RMM command section states
# inline. PSCI puts them in a chapter of their own, which is why every one of the
# 22 commands cross-references material outside its own section and no RMM
# command does.
_PROTO_PAT = re.compile(r'(?m)^\s*(5\.1\.\d+)\s+([A-Z][A-Za-z0-9_ /]*?)\s*$')


def _proto_name(title: str) -> str:
    """Command name from a 5.1.x title.

    Not _clean_name: that keeps only the first whitespace-separated token, and
    the PDF breaks one title as "PSCI_SET_ SUSPEND_MODE", which would truncate to
    "PSCI_SET_". Here the spaces are noise inside a single identifier, so they
    are removed rather than used as a delimiter.
    """
    return re.sub(r'[^A-Z0-9_]', '', title.upper().replace(' ', ''))


def extract_prototypes(cleaned_text: str) -> dict:
    """{CMD_NAME: prototype text} for the 5.1.x function-prototype subsections.

    Keyed on the LAST occurrence of each number: the table of contents lists them
    all before the body does, and a ToC line would otherwise win.
    """
    seen = {}
    for m in _PROTO_PAT.finditer(cleaned_text):
        seen[m.group(1)] = m
    if not seen:
        return {}
    ordered = sorted(seen.values(), key=lambda m: m.start())
    out = {}
    for i, m in enumerate(ordered):
        if i + 1 < len(ordered):
            end = ordered[i + 1].start()
        else:
            # 5.1.22 is the last prototype, so it has no successor to stop at and
            # ran to end-of-document exactly as 5.22 did: PSCI_STAT_COUNT came out
            # at 2599 lines. Stop at the next top-level section, which is 5.2.
            nxt = _TOP_SEC_PAT.search(cleaned_text, m.end())
            end = nxt.start() if nxt else len(cleaned_text)
        name = _proto_name(m.group(2))
        if name:
            out.setdefault(name, cleaned_text[m.start():end].rstrip())
    return out


# ---------------------------------------------------------------------------
# Cross-reference closure
# ---------------------------------------------------------------------------

# A heading line: a section number, then a title that starts with a capital and
# is short. Deliberately strict -- a loose pattern picks up numeric table rows
# and the index then reports a 4-line CPU_ON. Validated against the sections the
# splitter above produces: 5.3/5.4/5.6/5.12 agree to within the heading line.
_HEADING = re.compile(r'^\s*(\d+(?:\.\d+){0,2})\s+([A-Z][A-Za-z0-9_ ,/\-]{2,60})\s*$')
_XREF = re.compile(r'section\s+(\d+(?:\.\d+){0,2})', re.I)


def lines_of(cleaned_text: str) -> list:
    return cleaned_text.split("\n")


def section_index(cleaned_text: str) -> dict:
    """{number: (start, end)} over every numbered section, by line.

    The table of contents lists every number before the body does, so the LAST
    match for a number wins. A section runs until the next heading that is not
    one of its descendants, which is what makes 5.3 include 5.3.1 rather than
    stopping at it.
    """
    lines = cleaned_text.split("\n")
    pos = {}
    for i, l in enumerate(lines):
        m = _HEADING.match(l)
        if m:
            pos[m.group(1)] = i
    order = sorted(pos.items(), key=lambda kv: kv[1])
    out = {}
    for i, (num, start) in enumerate(order):
        end = len(lines)
        for other, s2 in order[i + 1:]:
            if not other.startswith(num + "."):
                end = s2
                break
        out[num] = (start, end)
    return out


def reference_closure(cleaned_text: str, index: dict, seeds: list, max_hops: int = 9) -> list:
    """Sections reachable from `seeds` by following "see section N", to a fixed point.

    Transitive on purpose. Pasting a command's prototype in front of its
    description does not resolve the problem, it moves it: the prototypes carry
    their own references, and the count of unresolved ones went UP, from 52 to
    72. Only the closure terminates.

    It stays small. Across the 17 commands with a clean prototype mapping the
    closure averages 597 lines, 14% of the document, against 53 for the
    description alone -- so this is a bounded amount of context, not "give the
    model the whole PDF". Three commands are the exception at ~40%, all of them
    in the CPU_SUSPEND family, where the power-state encoding drags in chapter 6.
    """
    lines = cleaned_text.split("\n")
    seen = {s for s in seeds if s in index}
    frontier = set(seen)
    for _ in range(max_hops):
        nxt = set()
        for n in frontier:
            st, en = index[n]
            for r in _XREF.findall("\n".join(lines[st:en])):
                if r in index and r not in seen:
                    nxt.add(r)
        if not nxt:
            break
        seen |= nxt
        frontier = nxt
    return sorted(seen, key=lambda n: index[n][0])


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    flags = {"--with-prototypes", "--with-closure"}
    argv = [a for a in sys.argv[1:] if a not in flags]
    with_prototypes = "--with-prototypes" in sys.argv[1:]
    with_closure = "--with-closure" in sys.argv[1:]
    versions = argv if argv else ["psci_13"]
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

        # `<version>_full` = the same commands with their 5.1.x prototype
        # prepended. Emitted alongside rather than instead: the plain version is
        # what the pipeline has always produced, and the pair is what measures
        # whether the missing parameter tables actually cost anything.
        if with_prototypes:
            protos = extract_prototypes(cleaned)
            hit = sorted(set(cmds) & set(protos))
            missing = sorted(set(cmds) - set(protos))
            full_dir = os.path.join(base_dir, "sections", f"{version}_full")
            os.makedirs(full_dir, exist_ok=True)
            for cmd_name, raw_text in sorted(cmds.items()):
                pre = protos.get(cmd_name)
                text = (f"{pre}\n\n{raw_text}" if pre else raw_text)
                with open(os.path.join(full_dir, f"{cmd_name}_command.txt"), "w") as fh:
                    fh.write(text)
            print(f"  {len(hit)}/{len(cmds)} commands matched a 5.1.x prototype "
                  f"→ sections/{version}_full/")
            if missing:
                print(f"  [WARN] no prototype for: {', '.join(missing)}")

        # `<version>_closure` = description + prototype + everything either of
        # them references, transitively. The point of the three variants is that
        # they are a ladder: whether more resolved context keeps helping is a
        # measurement, not something to assume.
        if with_closure:
            index = section_index(cleaned)
            protos = extract_prototypes(cleaned)
            # command -> the section numbers it starts from
            seeds = {}
            for num, (st, en) in index.items():
                if not re.match(r'^5\.\d+$', num):
                    continue
                title = lines_of(cleaned)[st].strip()
                for nm in _parse_cmd_names(title.split(None, 1)[1] if " " in title else ""):
                    if nm in cmds:
                        seeds.setdefault(nm, []).append(num)
            for num, (st, en) in index.items():
                if not re.match(r'^5\.1\.\d+$', num):
                    continue
                title = lines_of(cleaned)[st].strip()
                nm = _proto_name(title.split(None, 1)[1] if " " in title else "")
                if nm in cmds:
                    seeds.setdefault(nm, []).append(num)
            cl_dir = os.path.join(base_dir, "sections", f"{version}_closure")
            os.makedirs(cl_dir, exist_ok=True)
            all_lines = lines_of(cleaned)
            sizes = []
            for cmd_name, raw_text in sorted(cmds.items()):
                sec = reference_closure(cleaned, index, seeds.get(cmd_name, []))
                if sec:
                    text = "\n\n".join("\n".join(all_lines[index[n][0]:index[n][1]])
                                       for n in sec)
                else:
                    text = raw_text
                with open(os.path.join(cl_dir, f"{cmd_name}_command.txt"), "w") as fh:
                    fh.write(text)
                sizes.append(len(text.splitlines()))
            got = sum(1 for c in cmds if seeds.get(c))
            print(f"  {got}/{len(cmds)} commands resolved a closure "
                  f"(mean {sum(sizes)//max(len(sizes),1)} lines, max {max(sizes)}) "
                  f"→ sections/{version}_closure/")

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
