#!/usr/bin/env python3
"""
split_specs.py

Splits each {version}_gold.rs into:
  - specs/{version}/preamble.rs          — everything before the first _spec function
  - specs/{version}/{cmd_name}_spec.rs   — the pub open spec fn {cmd}_spec block
  - specs/{version}/{cmd_name}_rule.rs   — the pub proof fn {cmd}_rule block (if present)

Usage:
    python3 split_specs.py eac5 rel0 alp11 alp12
    # or: python3 split_specs.py   (defaults to all four versions)
"""

import re
import os
import sys

VERSIONS = ["eac5", "rel0", "alp11", "alp12", "alp13", "alp14"]


def extract_function_blocks(lines: list[str]) -> list[tuple[str, int, int]]:
    """
    Find all top-level function definitions and return a list of
    (function_name, start_line_index, end_line_index) triples.

    Works by tracking brace depth: a function starts when we see a
    `pub open spec fn` or `pub proof fn` declaration and ends when
    brace depth returns to 0.

    Returns the end_line_index as *inclusive* (the line with the closing `}`).
    """
    spec_fn_pat  = re.compile(r'^pub open spec fn (\w+)\s*\(')
    proof_fn_pat = re.compile(r'^pub proof fn (\w+)\s*\(')

    blocks = []
    i = 0
    while i < len(lines):
        line = lines[i]
        m = spec_fn_pat.match(line) or proof_fn_pat.match(line)
        if m:
            fn_name   = m.group(1)
            fn_start  = i
            depth     = 0
            seen_open = False  # have we entered the body ({)?
            j         = i
            while j < len(lines):
                opens  = lines[j].count('{')
                closes = lines[j].count('}')
                depth += opens - closes
                if opens > 0:
                    seen_open = True
                # For stubs (no body, ends with ';'): terminate on the stub line itself
                stripped = lines[j].rstrip()
                if not seen_open and stripped.endswith(';'):
                    blocks.append((fn_name, fn_start, j))
                    i = j + 1
                    break
                # For functions with bodies: terminate when depth returns to 0
                if seen_open and depth == 0:
                    blocks.append((fn_name, fn_start, j))
                    i = j + 1
                    break
                j += 1
            else:
                i += 1
        else:
            i += 1
    return blocks


def extract_type_definitions(lines: list[str]) -> dict:
    """
    Parse the preamble to extract per-type Verus definitions.

    Returns {type_name: text} for:
      - pub enum Foo { ... }
      - struct Foo { ... }

    Helper function signatures (pub open spec fn ...) are NOT included here —
    they are layer-3 stubs, not directly derived from PDF type sections.
    """
    enum_pat   = re.compile(r"^pub enum (\w+)\s*\{")
    struct_pat = re.compile(r"^struct (\w+)\s*\{")

    result = {}
    i = 0
    while i < len(lines):
        line = lines[i]
        m = enum_pat.match(line) or struct_pat.match(line)
        if m:
            type_name = m.group(1)
            start     = i
            depth     = 0
            j         = i
            while j < len(lines):
                depth += lines[j].count("{") - lines[j].count("}")
                if depth == 0 and j > start:
                    result[type_name] = "".join(lines[start:j + 1])
                    i = j + 1
                    break
                j += 1
            else:
                i += 1
        else:
            i += 1
    return result


def extract_layer3_stubs(lines: list[str]) -> dict:
    """
    Extract pub open spec fn stubs (ending with ';') from preamble.
    Returns {fn_name: stub_text}.
    These are uninterpreted spec function declarations (no body).
    """
    stub_pat = re.compile(r'^pub open spec fn (\w+)\s*\(')
    result = {}
    i = 0
    while i < len(lines):
        line = lines[i]
        m = stub_pat.match(line)
        if m:
            fn_name = m.group(1)
            # Collect lines until ';' (stub, no body)
            # If we hit '{', this is a full function — skip
            stub_lines = []
            j = i
            found_body = False
            while j < len(lines):
                stub_lines.append(lines[j])
                stripped = lines[j].rstrip()
                if '{' in lines[j] and '}' not in lines[j]:
                    found_body = True
                    break
                if stripped.endswith(';'):
                    break
                j += 1
            if not found_body and stub_lines:
                result[fn_name] = "".join(stub_lines)
                i = j + 1
            else:
                i += 1
        else:
            i += 1
    return result


def cmd_name_from_fn(fn_name: str) -> str | None:
    """
    Convert function name to canonical command name.
    'rmi_data_create_spec' → 'RMI_DATA_CREATE'
    'rmi_data_create_rule' → 'RMI_DATA_CREATE'
    Returns None if neither _spec nor _rule suffix.
    """
    if fn_name.endswith("_spec"):
        return fn_name[:-5].upper()
    if fn_name.endswith("_rule"):
        return fn_name[:-5].upper()
    return None


def split_gold(version: str, base_dir: str):
    rs_path = os.path.join(base_dir, f"{version}_gold.rs")
    if not os.path.exists(rs_path):
        print(f"[WARN] {rs_path} not found, skipping.", flush=True)
        return

    out_dir = os.path.join(base_dir, "specs", version)
    os.makedirs(out_dir, exist_ok=True)

    with open(rs_path) as fh:
        lines = fh.readlines()

    blocks = extract_function_blocks(lines)

    # Identify the first _spec function to determine preamble end
    first_spec_start = None
    for fn_name, start, end in blocks:
        if fn_name.endswith("_spec"):
            first_spec_start = start
            break

    if first_spec_start is None:
        print(f"  [WARN] No _spec functions found in {version}_gold.rs")
        return

    # Write preamble (everything up to, not including, first _spec)
    preamble_lines = lines[:first_spec_start]
    preamble_path  = os.path.join(out_dir, "preamble.rs")
    with open(preamble_path, "w") as fh:
        fh.writelines(preamble_lines)

    # Build per-command dictionaries
    spec_blocks = {}  # cmd_name → (start, end)
    rule_blocks = {}

    for fn_name, start, end in blocks:
        cmd = cmd_name_from_fn(fn_name)
        if cmd is None:
            continue
        if fn_name.endswith("_spec"):
            spec_blocks[cmd] = (start, end)
        elif fn_name.endswith("_rule"):
            rule_blocks[cmd] = (start, end)

    # Write per-type definitions extracted from preamble
    type_defs = extract_type_definitions(preamble_lines)
    if type_defs:
        types_dir = os.path.join(out_dir, "types")
        os.makedirs(types_dir, exist_ok=True)
        for type_name, text in sorted(type_defs.items()):
            out_path = os.path.join(types_dir, f"{type_name}.rs")
            with open(out_path, "w") as fh:
                fh.write(text)

    # Write per-helper stub files extracted from preamble
    helper_stubs = extract_layer3_stubs(preamble_lines)
    if helper_stubs:
        helpers_dir = os.path.join(out_dir, "helpers")
        os.makedirs(helpers_dir, exist_ok=True)
        for fn_name, text in sorted(helper_stubs.items()):
            out_path = os.path.join(helpers_dir, f"{fn_name}.rs")
            with open(out_path, "w") as fh:
                fh.write(text)

    spec_count = 0
    rule_count = 0
    for cmd, (start, end) in sorted(spec_blocks.items()):
        block_lines = lines[start:end + 1]
        # Check for [EXCLUDED] marker (coverage patches add this as a comment)
        text = "".join(block_lines)
        if "[EXCLUDED]" in text:
            print(f"  [SKIP] {cmd} has [EXCLUDED] marker, skipping.")
            continue

        filename = cmd.lower() + "_spec.rs"
        out_path  = os.path.join(out_dir, filename)
        with open(out_path, "w") as fh:
            fh.writelines(block_lines)
        spec_count += 1

    for cmd, (start, end) in sorted(rule_blocks.items()):
        block_lines = lines[start:end + 1]
        filename = cmd.lower() + "_rule.rs"
        out_path  = os.path.join(out_dir, filename)
        with open(out_path, "w") as fh:
            fh.writelines(block_lines)
        rule_count += 1

    # Write epilogue: content after the last function block, before closing '}'
    # (e.g. 'fn main() {}' needed for Verus standalone compilation)
    # We identify the last '} // verus!' or standalone '}' at depth 0 as the
    # outer closing brace and exclude it; everything before it is the epilogue.
    if blocks:
        last_end = max(end for _, _, end in blocks)
        remaining = lines[last_end + 1:]
        # Find the last non-empty line; it should be the outer closing '}'
        last_nonempty_idx = None
        for k in range(len(remaining) - 1, -1, -1):
            if remaining[k].strip():
                last_nonempty_idx = k
                break
        # Epilogue is everything before the last non-empty line
        if last_nonempty_idx is not None and last_nonempty_idx > 0:
            epilogue_lines = remaining[:last_nonempty_idx]
        else:
            epilogue_lines = []
        epilogue_text = "".join(epilogue_lines).strip()
        if epilogue_text:
            epilogue_path = os.path.join(out_dir, "epilogue.rs")
            with open(epilogue_path, "w") as fh:
                fh.write(epilogue_text + "\n")

    print(f"  {version}: preamble ({first_spec_start} lines), "
          f"{len(type_defs)} types, {len(helper_stubs)} helpers, "
          f"{spec_count} _spec, {rule_count} _rule functions")


def main():
    versions = sys.argv[1:] if len(sys.argv) > 1 else VERSIONS
    base_dir = os.path.dirname(os.path.abspath(__file__))

    for version in versions:
        print(f"Processing {version}...", flush=True)
        split_gold(version, base_dir)

    print("Done.")


if __name__ == "__main__":
    main()
