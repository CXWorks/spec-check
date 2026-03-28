#!/usr/bin/env python3
"""
test_e2e_oracle.py

Oracle end-to-end test: validates pipeline assembly logic without GPU models.
Uses golden files (specs/{version}/) as "perfect model outputs" at each stage.

Assembles a complete .rs file and diffs against the gold file to verify
that the pipeline reconstruction is correct.

Usage:
    python3 test_e2e_oracle.py --version eac5
    python3 test_e2e_oracle.py --version eac5 --out eac5_assembled.rs
"""

import argparse
import importlib.util
import os
import sys
import difflib

BASE_DIR = os.path.dirname(os.path.abspath(__file__))


def _load_split_specs():
    spec = importlib.util.spec_from_file_location(
        "split_specs", os.path.join(BASE_DIR, "split_specs.py")
    )
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

def load_file(path: str) -> str:
    with open(path) as fh:
        return fh.read()


def load_preamble(version: str) -> str:
    path = os.path.join(BASE_DIR, "specs", version, "preamble.rs")
    return load_file(path)


def load_type_defs(version: str) -> dict[str, str]:
    types_dir = os.path.join(BASE_DIR, "specs", version, "types")
    result = {}
    if os.path.exists(types_dir):
        for fname in sorted(os.listdir(types_dir)):
            if fname.endswith(".rs"):
                result[fname[:-3]] = load_file(os.path.join(types_dir, fname)).strip()
    return result


def load_helper_stubs(version: str) -> dict[str, str]:
    helpers_dir = os.path.join(BASE_DIR, "specs", version, "helpers")
    result = {}
    if os.path.exists(helpers_dir):
        for fname in sorted(os.listdir(helpers_dir)):
            if fname.endswith(".rs"):
                result[fname[:-3]] = load_file(os.path.join(helpers_dir, fname)).strip()
    return result


def load_cmd_specs(version: str) -> dict[str, str]:
    specs_dir = os.path.join(BASE_DIR, "specs", version)
    result = {}
    if os.path.exists(specs_dir):
        for fname in sorted(os.listdir(specs_dir)):
            if fname.endswith("_spec.rs"):
                cmd_key = fname[:-8]  # rmi_data_create_spec → rmi_data_create
                text = load_file(os.path.join(specs_dir, fname)).strip()
                if "[EXCLUDED]" not in text:
                    result[cmd_key] = text
    return result


def load_cmd_rules(version: str) -> dict[str, str]:
    specs_dir = os.path.join(BASE_DIR, "specs", version)
    result = {}
    if os.path.exists(specs_dir):
        for fname in sorted(os.listdir(specs_dir)):
            if fname.endswith("_rule.rs"):
                cmd_key = fname[:-8]
                result[cmd_key] = load_file(os.path.join(specs_dir, fname)).strip()
    return result


def get_gold_fn_order(version: str) -> list[str]:
    """
    Return function names in the order they appear in the gold .rs file.
    Used to preserve the original spec ordering (rmi_*, rsi_*, psci_*, rules).
    """
    gold_path = os.path.join(BASE_DIR, f"{version}_gold.rs")
    if not os.path.exists(gold_path):
        return []
    split_specs = _load_split_specs()
    with open(gold_path) as fh:
        lines = fh.readlines()
    blocks = split_specs.extract_function_blocks(lines)
    return [fn_name for fn_name, _start, _end in blocks]


def assemble_from_preamble(version: str) -> str:
    """
    Assemble complete .rs by concatenating preamble + command specs + rules
    in the same order as the gold file (preserving rmi/rsi/psci ordering).
    """
    preamble  = load_preamble(version)
    cmd_specs = load_cmd_specs(version)
    cmd_rules = load_cmd_rules(version)

    # Use gold file ordering to reconstruct the correct sequence
    fn_order = get_gold_fn_order(version)

    parts = [preamble.rstrip()]

    seen_specs = set()
    seen_rules = set()
    for fn_name in fn_order:
        if fn_name.endswith("_spec"):
            cmd_key = fn_name[:-5]  # rmi_data_create_spec → rmi_data_create
            if cmd_key in cmd_specs and cmd_key not in seen_specs:
                parts.append("\n" + cmd_specs[cmd_key])
                seen_specs.add(cmd_key)
        elif fn_name.endswith("_rule"):
            cmd_key = fn_name[:-5]
            if cmd_key in cmd_rules and cmd_key not in seen_rules:
                parts.append("\n" + cmd_rules[cmd_key])
                seen_rules.add(cmd_key)

    # Append any remaining specs/rules not in gold order (shouldn't happen)
    for cmd_key in sorted(cmd_specs):
        if cmd_key not in seen_specs:
            parts.append("\n" + cmd_specs[cmd_key])
    for cmd_key in sorted(cmd_rules):
        if cmd_key not in seen_rules:
            parts.append("\n" + cmd_rules[cmd_key])

    # Load epilogue (e.g. fn main() {}) if present, else use standard one
    epilogue_path = os.path.join(BASE_DIR, "specs", version, "epilogue.rs")
    if os.path.exists(epilogue_path):
        epilogue = load_file(epilogue_path).strip()
        parts.append("\n" + epilogue)
    else:
        parts.append("\nfn main() {\n}")

    # Close verus! block
    parts.append("\n}")

    return "\n".join(parts) + "\n"


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    parser = argparse.ArgumentParser(description="Oracle end-to-end pipeline test")
    parser.add_argument("--version", required=True, help="Version to test (e.g. eac5)")
    parser.add_argument("--out", help="Output file path (default: {version}_assembled.rs)")
    parser.add_argument("--no-diff", action="store_true", help="Skip diff output")
    args = parser.parse_args()

    version = args.version
    out_path = args.out or os.path.join(BASE_DIR, f"{version}_assembled.rs")
    gold_path = os.path.join(BASE_DIR, f"{version}_gold.rs")

    # Check required files exist
    preamble_path = os.path.join(BASE_DIR, "specs", version, "preamble.rs")
    if not os.path.exists(preamble_path):
        print(f"[ERROR] specs/{version}/preamble.rs not found. "
              f"Run split_specs.py first.", file=sys.stderr)
        sys.exit(1)

    print(f"[ORACLE TEST] Version: {version}")

    # Load and count parts
    type_defs    = load_type_defs(version)
    helper_stubs = load_helper_stubs(version)
    cmd_specs    = load_cmd_specs(version)
    cmd_rules    = load_cmd_rules(version)

    print(f"  Types   : {len(type_defs)}")
    print(f"  Helpers : {len(helper_stubs)}")
    print(f"  Specs   : {len(cmd_specs)}")
    print(f"  Rules   : {len(cmd_rules)}")

    # Assemble
    assembled = assemble_from_preamble(version)
    with open(out_path, "w") as fh:
        fh.write(assembled)
    print(f"  Assembled → {out_path} ({len(assembled)} chars)")

    # Diff against gold
    if not os.path.exists(gold_path):
        print(f"[WARN] Gold file not found: {gold_path}")
        print("[RESULT] Cannot diff — assembly written but not verified")
        return

    gold_text = load_file(gold_path)
    assembled_lines = assembled.splitlines(keepends=True)
    gold_lines      = gold_text.splitlines(keepends=True)

    diff = list(difflib.unified_diff(
        gold_lines, assembled_lines,
        fromfile=f"{version}_gold.rs",
        tofile=f"{version}_assembled.rs",
        n=3,
    ))

    if not diff:
        print("[RESULT] PASS — assembled file matches gold exactly")
    else:
        print(f"[RESULT] DIFF — {len(diff)} diff lines (first 60 shown):")
        if not args.no_diff:
            for line in diff[:60]:
                print(line, end="")
        else:
            print(f"  (use without --no-diff to see diff)")


if __name__ == "__main__":
    main()
