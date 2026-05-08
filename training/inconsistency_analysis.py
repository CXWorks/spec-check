#!/usr/bin/env python3
"""
inconsistency_analysis.py

For each non-trivial spec function in a generated_clean.rs file, generate
a Verus proof fn with `requires spec_fn(...)` and `ensures false`. If Verus
verifies it, the spec is logically inconsistent.

Usage:
    python3 inconsistency_analysis.py <spec_name>
    e.g.: python3 inconsistency_analysis.py sdei
"""

import re
import subprocess
import sys
import os

# ---------------------------------------------------------------------------
# Regex: match pub open spec fn signatures + bodies
# ---------------------------------------------------------------------------

_SIG_RE = re.compile(
    r'pub\s+open\s+spec\s+fn\s+(\w+)\s*'
    r'(\((?:[^(){};]|\([^)]*\))*\))'
    r'\s*->\s*bool\s*\{',
    re.DOTALL,
)


def extract_fn_body(text: str, brace_start: int) -> str:
    depth = 0
    i = brace_start
    while i < len(text):
        c = text[i]
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                return text[brace_start: i + 1]
        i += 1
    return text[brace_start:]


def is_trivial(body: str) -> bool:
    """Returns True if the body is essentially `{ true }` with no real logic."""
    inner = body.strip().lstrip('{').rstrip('}').strip()
    # Remove comments
    inner = re.sub(r'//[^\n]*', '', inner).strip()
    return inner in ('true', 'true;', '')


def parse_spec_fns(source: str) -> list[dict]:
    """Extract all pub open spec fn -> bool functions with their signatures."""
    fns = []
    for m in _SIG_RE.finditer(source):
        fn_name = m.group(1)
        params_str = m.group(2)
        brace_pos = m.end() - 1
        body = extract_fn_body(source, brace_pos)

        if is_trivial(body):
            continue

        # Parse parameter list into (name, type) pairs for call-site reconstruction
        params_inner = params_str.strip().lstrip('(').rstrip(')')
        # Split on commas, respecting angle brackets and parens
        params = split_params(params_inner)

        fns.append({
            'name': fn_name,
            'params_str': params_str,
            'params': params,
            'body': body,
            'body_trivial': False,
        })
    return fns


def split_params(params_inner: str) -> list[tuple[str, str]]:
    """Split 'a: T1, b: T2, ...' respecting nested <> and ()."""
    result = []
    depth = 0
    current = ''
    for ch in params_inner:
        if ch in '(<':
            depth += 1
        elif ch in ')>':
            depth -= 1
        if ch == ',' and depth == 0:
            result.append(current.strip())
            current = ''
        else:
            current += ch
    if current.strip():
        result.append(current.strip())

    pairs = []
    for p in result:
        p = p.strip()
        if not p:
            continue
        # Handle "name: Type"
        colon = p.find(':')
        if colon > 0:
            pname = p[:colon].strip()
            ptype = p[colon+1:].strip()
            pairs.append((pname, ptype))
        else:
            pairs.append((p, '?'))
    return pairs


def make_proof_obligation(fn: dict, idx: int) -> str:
    """Generate a proof fn that requires spec_fn and ensures false."""
    name = fn['name']
    params_str = fn['params_str']
    # Build call-site arg list: just the param names
    arg_names = ', '.join(p[0] for p in fn['params'])

    return (
        f"proof fn check_inconsistency_{name}_{idx}{params_str}\n"
        f"    requires {name}({arg_names})\n"
        f"    ensures false\n"
        f"{{}}"
    )


def build_test_file(layer1_source: str, spec_fns: list[dict]) -> str:
    """Build a complete Verus test file with all proof obligations."""
    obligations = []
    for i, fn in enumerate(spec_fns):
        obligations.append(make_proof_obligation(fn, i))

    return (
        layer1_source.rstrip()
        + "\n\n// --- Inconsistency proof obligations ---\n\n"
        + "\n\n".join(obligations)
        + "\n\n} // verus!\n"
    )


def run_verus(filepath: str) -> tuple[bool, str]:
    """Run verus on filepath, return (success, output)."""
    result = subprocess.run(
        ['verus-x86-linux/verus', '--crate-type', 'lib', filepath],
        capture_output=True, text=True
    )
    out = result.stdout + result.stderr
    return result.returncode == 0, out


def parse_verification_results(output: str) -> tuple[int, int]:
    """Parse 'N verified, M errors' from verus output."""
    m = re.search(r'(\d+) verified, (\d+) errors', output)
    if m:
        return int(m.group(1)), int(m.group(2))
    return 0, -1


def run_individual_checks(
    layer1_source: str,
    spec_fns: list[dict],
    tmp_prefix: str,
) -> list[dict]:
    """
    Test each spec fn individually. Returns list of results with
    'inconsistent' flag and verus output.
    """
    results = []
    for i, fn in enumerate(spec_fns):
        # Build a file with just this one proof obligation
        obligation = make_proof_obligation(fn, i)
        # We need the spec fn definition too
        fn_def = f"pub open spec fn {fn['name']}{fn['params_str']} -> bool {fn['body']}"
        test_src = (
            layer1_source.rstrip()
            + f"\n\n{fn_def}\n\n"
            + "// --- Inconsistency check ---\n\n"
            + obligation
            + "\n\n} // verus!\n"
        )

        tmp_path = f"{tmp_prefix}_{i}_{fn['name']}.rs"
        with open(tmp_path, 'w') as f:
            f.write(test_src)

        ok, out = run_verus(tmp_path)
        verified, errors = parse_verification_results(out)
        # "postcondition not satisfied" on the proof fn means spec is satisfiable (consistent)
        # If errors==0 and the proof fn itself verified, spec is INCONSISTENT
        postcond_fail = 'postcondition not satisfied' in out
        type_error = errors > 0 and not postcond_fail

        if errors == 0:
            inconsistent = True
        elif postcond_fail:
            inconsistent = False
        else:
            inconsistent = None  # type error — can't determine

        results.append({
            'name': fn['name'],
            'inconsistent': inconsistent,
            'verified': verified,
            'errors': errors,
            'type_error': type_error,
            'output': out,
        })
        os.unlink(tmp_path)

        if inconsistent is True:
            status = "INCONSISTENT"
        elif inconsistent is False:
            status = "consistent"
        else:
            status = "type-error/skip"
        print(f"  {fn['name']}: {status} (verified={verified}, errors={errors})")

    return results


def load_clean_file_split(clean_path: str) -> tuple[str, str]:
    """
    Split a clean .rs file into:
    - layer1_part: everything up to and including '// --- Commands ---'
    - commands_part: the rest (without closing } // verus!)
    Returns (layer1_part, commands_part).
    """
    with open(clean_path) as f:
        text = f.read()

    # Strip the closing verus!
    text = re.sub(r'\n\}\s*//\s*verus!\s*$', '', text.rstrip())

    marker = '// --- Commands ---'
    idx = text.find(marker)
    if idx >= 0:
        layer1_part = text[:idx + len(marker)]
    else:
        layer1_part = text  # no marker, use whole file

    return layer1_part, text


def main():
    if len(sys.argv) < 2:
        print("Usage: python3 inconsistency_analysis.py <spec_name>")
        print("  spec_name: sdei, drtm, scmi, ffa")
        sys.exit(1)

    spec = sys.argv[1].lower()
    clean_path = f"{spec}_generated_clean.rs"

    if not os.path.exists(clean_path):
        print(f"Error: {clean_path} not found")
        sys.exit(1)

    print(f"\n{'='*60}")
    print(f"Inconsistency analysis: {spec.upper()}")
    print(f"{'='*60}")

    layer1_part, full_text = load_clean_file_split(clean_path)

    # Extract spec fns from the commands section
    commands_text = full_text[len(layer1_part):]
    spec_fns = parse_spec_fns(commands_text)

    print(f"Non-trivial spec functions found: {len(spec_fns)}")
    for fn in spec_fns:
        print(f"  - {fn['name']} ({len(fn['params'])} params)")

    if not spec_fns:
        print("Nothing to check.")
        return

    print(f"\nRunning individual inconsistency checks...")
    tmp_prefix = f"/tmp/incon_{spec}"
    results = run_individual_checks(layer1_part, spec_fns, tmp_prefix)

    print(f"\n{'='*60}")
    print(f"Results for {spec.upper()}")
    print(f"{'='*60}")
    inconsistent = [r for r in results if r['inconsistent'] is True]
    consistent = [r for r in results if r['inconsistent'] is False]
    errored = [r for r in results if r['inconsistent'] is None]

    print(f"\nINCONSISTENT ({len(inconsistent)}):")
    for r in inconsistent:
        print(f"  [!] {r['name']}")

    print(f"\nConsistent ({len(consistent)}):")
    for r in consistent:
        print(f"  [ok] {r['name']}")

    if errored:
        print(f"\nType errors (check skipped) ({len(errored)}):")
        for r in errored:
            print(f"  [?] {r['name']} (errors={r['errors']})")

    print(f"\nSummary: {len(inconsistent)} inconsistent, {len(consistent)} consistent, {len(errored)} type errors")


if __name__ == '__main__':
    main()
