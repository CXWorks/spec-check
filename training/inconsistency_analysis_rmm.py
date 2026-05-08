#!/usr/bin/env python3
"""
inconsistency_analysis_rmm.py

Iterate over all gold spec functions in specs/alp14/*_spec.rs, skip known
bugs (and known annotation false-positives), and check remaining functions
for logical inconsistency using Verus.

Usage:
    python3 inconsistency_analysis_rmm.py [--skip-known]

Exit code 0 always; inconsistencies are printed to stdout.
"""

import os
import re
import subprocess
import sys
import argparse
from pathlib import Path

BASE     = Path(__file__).parent
PREAMBLE = BASE / "specs" / "alp14" / "preamble.rs"
SPECS_DIR = BASE / "specs" / "alp14"
VERUS    = BASE / "verus-x86-linux" / "verus"

# ---------------------------------------------------------------------------
# Known bugs — skip so we don't re-report them as "new"
# ---------------------------------------------------------------------------
KNOWN_BUGS = {
    # RMM alp14 bugs (machine-checked)
    "rmi_pdev_stop_spec",
    "rsi_attestation_token_continue_spec",
    # Annotation false positive (spec is fine, annotation is wrong)
    "rsi_vdev_validate_mapping_spec",
}

# Specs known to use invalid Verus syntax (:: concat / [N:0] bit-range)
SYNTAX_EXCLUDED = {
    "rmi_psmmu_msi_config_spec",
    "rsi_attestation_token_init_spec",
    "rsi_measurement_extend_spec",
    "rsi_mem_set_perm_index_spec",
    "rsi_plane_sysreg_read_spec",
    "rsi_plane_sysreg_write_spec",
}


def read_preamble() -> str:
    """Return preamble content without the closing verus! brace.
    Also makes all struct definitions pub so that pub open spec fn
    field accesses compile in a standalone crate."""
    text = PREAMBLE.read_text()
    # Make non-pub structs pub so field accesses work in pub open spec fns
    text = re.sub(r'(?m)^struct\b', 'pub struct', text)
    return text.rstrip()


def list_spec_files() -> list[Path]:
    """Return all *_spec.rs files (exclude rule files, preamble, epilogue)."""
    files = sorted(SPECS_DIR.glob("*_spec.rs"))
    return files


def extract_fn_name(source: str) -> str | None:
    m = re.search(r'pub\s+open\s+spec\s+fn\s+(\w+)', source)
    return m.group(1) if m else None


def is_trivial(source: str) -> bool:
    """Returns True if the spec body is just `{ true }` or empty."""
    m = re.search(r'pub\s+open\s+spec\s+fn\s+\w+[^{]*\{(.*)\}\s*$',
                  source.strip(), re.DOTALL)
    if not m:
        return False
    body = re.sub(r'//[^\n]*', '', m.group(1)).strip()
    return body in ('', 'true', 'true;')


def extract_params(source: str) -> tuple[str, list[tuple[str, str]]]:
    """Extract params_str and list of (name, type) pairs from spec fn signature."""
    m = re.search(
        r'pub\s+open\s+spec\s+fn\s+\w+\s*(\((?:[^(){};]|\([^)]*\))*\))\s*->',
        source, re.DOTALL
    )
    if not m:
        return '()', []
    params_str = m.group(1)
    inner = params_str.strip().lstrip('(').rstrip(')')
    pairs = []
    depth = 0
    current = ''
    for ch in inner:
        if ch in '(<':
            depth += 1
        elif ch in ')>':
            depth -= 1
        if ch == ',' and depth == 0:
            pairs.append(current.strip())
            current = ''
        else:
            current += ch
    if current.strip():
        pairs.append(current.strip())
    result = []
    for p in pairs:
        p = p.strip()
        if not p:
            continue
        colon = p.find(':')
        if colon > 0:
            result.append((p[:colon].strip(), p[colon+1:].strip()))
        else:
            result.append((p, '?'))
    return params_str, result


def make_proof_obligation(fn_name: str, params_str: str, params: list) -> str:
    arg_names = ', '.join(p[0] for p in params)
    return (
        f"proof fn check_inconsistency_{fn_name}{params_str}\n"
        f"    requires {fn_name}({arg_names})\n"
        f"    ensures false\n"
        f"{{}}"
    )


def run_verus(filepath: str) -> tuple[bool, str]:
    result = subprocess.run(
        [str(VERUS), '--crate-type', 'lib', filepath],
        capture_output=True, text=True, timeout=60,
    )
    return result.returncode == 0, result.stdout + result.stderr


def parse_counts(output: str) -> tuple[int, int]:
    m = re.search(r'(\d+) verified, (\d+) errors', output)
    if m:
        return int(m.group(1)), int(m.group(2))
    return 0, -1


def check_spec_file(preamble: str, spec_path: Path) -> dict:
    spec_src = spec_path.read_text().strip()
    fn_name = extract_fn_name(spec_src)
    if not fn_name:
        return {'name': spec_path.stem, 'inconsistent': None, 'reason': 'no fn found'}

    if is_trivial(spec_src):
        return {'name': fn_name, 'inconsistent': False, 'reason': 'trivial'}

    params_str, params = extract_params(spec_src)
    obligation = make_proof_obligation(fn_name, params_str, params)

    test_src = (
        preamble
        + f"\n\n{spec_src}\n\n"
        + "// --- Inconsistency check ---\n\n"
        + obligation
        + "\n\n} // verus!\n"
    )

    tmp_path = f"/tmp/rmm_incon_{fn_name}.rs"
    with open(tmp_path, 'w') as f:
        f.write(test_src)

    try:
        ok, out = run_verus(tmp_path)
    except subprocess.TimeoutExpired:
        return {'name': fn_name, 'inconsistent': None, 'reason': 'timeout', 'output': ''}
    finally:
        if os.path.exists(tmp_path):
            os.unlink(tmp_path)

    verified, errors = parse_counts(out)
    postcond_fail = 'postcondition not satisfied' in out
    type_error    = errors > 0 and not postcond_fail

    if errors == 0:
        inconsistent = True
    elif postcond_fail:
        inconsistent = False
    else:
        inconsistent = None  # type error

    return {
        'name':         fn_name,
        'inconsistent': inconsistent,
        'verified':     verified,
        'errors':       errors,
        'type_error':   type_error,
        'reason':       'type_error' if type_error else ('ok' if not inconsistent else 'INCONSISTENT'),
        'output':       out,
    }


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--skip-known', action='store_true', default=True,
                        help='Skip already-known bugs (default: true)')
    parser.add_argument('--no-skip-known', dest='skip_known', action='store_false')
    args = parser.parse_args()

    preamble = read_preamble()
    spec_files = list_spec_files()

    skipped_known = []
    skipped_syntax = []
    skipped_trivial = []
    to_check = []

    for sp in spec_files:
        stem = sp.stem
        fn_name_guess = stem  # usually stem == fn_name

        if stem in SYNTAX_EXCLUDED:
            skipped_syntax.append(stem)
            continue
        if args.skip_known and stem in KNOWN_BUGS:
            skipped_known.append(stem)
            continue
        to_check.append(sp)

    print(f"\n{'='*60}")
    print(f"RMM inconsistency analysis (specs/alp14/)")
    print(f"{'='*60}")
    print(f"Total spec files: {len(spec_files)}")
    print(f"Skipped (known bugs / false-positives): {len(skipped_known)} — {skipped_known}")
    print(f"Skipped (syntax issues): {len(skipped_syntax)}")
    print(f"To check: {len(to_check)}\n")

    results = []
    for i, sp in enumerate(to_check):
        r = check_spec_file(preamble, sp)
        results.append(r)

        if r['inconsistent'] is True:
            status = "INCONSISTENT"
        elif r['inconsistent'] is False:
            if r.get('reason') == 'trivial':
                status = "trivial/skip"
            else:
                status = f"consistent (v={r.get('verified','?')}, e={r.get('errors','?')})"
        else:
            status = f"type-error/skip (e={r.get('errors','?')})"
        print(f"  [{i+1}/{len(to_check)}] {r['name']}: {status}")

    print(f"\n{'='*60}")
    print(f"Results")
    print(f"{'='*60}")

    inconsistent = [r for r in results if r['inconsistent'] is True]
    consistent   = [r for r in results if r['inconsistent'] is False and r.get('reason') != 'trivial']
    trivial      = [r for r in results if r.get('reason') == 'trivial']
    errored      = [r for r in results if r['inconsistent'] is None]

    print(f"\nNEW INCONSISTENCIES ({len(inconsistent)}):")
    for r in inconsistent:
        print(f"  [!] {r['name']}")
        # Show a snippet of the verus output
        out_lines = r.get('output', '').splitlines()
        for line in out_lines[:10]:
            print(f"      {line}")

    print(f"\nConsistent ({len(consistent)}):")
    for r in consistent:
        print(f"  [ok] {r['name']}")

    if trivial:
        print(f"\nTrivial/skipped ({len(trivial)}): {[r['name'] for r in trivial]}")

    if errored:
        print(f"\nType errors / skipped ({len(errored)}):")
        for r in errored:
            print(f"  [?] {r['name']} (errors={r.get('errors','?')})")

    print(f"\nSummary: {len(inconsistent)} new inconsistent, "
          f"{len(consistent)} consistent, {len(trivial)} trivial, "
          f"{len(errored)} type-errors, {len(skipped_known)} known-bugs skipped")

    if inconsistent:
        print(f"\n{'='*60}")
        print("DETAILED OUTPUT FOR INCONSISTENT SPECS")
        print(f"{'='*60}")
        for r in inconsistent:
            print(f"\n--- {r['name']} ---")
            print(r.get('output', ''))


if __name__ == '__main__':
    main()
