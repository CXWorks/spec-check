#!/usr/bin/env python3
"""Internal-consistency sweep for a document with no gold specs.

    python3 scripts/psci_sweep.py --gen-dir predictions/psci-9b/psci_13 \
        --preamble training-dataset/specs/psci_13/preamble.rs --out results/psci/9b.json
    python3 scripts/psci_sweep.py --self-test          # validate the detector first

ARM PSCI (DEN0022F.b) has no gold Verus specs, and none can be written cheaply --
so "does it agree with gold" is unavailable, and with it every number the alp14
work reports. What survives without gold is what a spec says about *itself*:

  unsat    `proof fn f(p..) requires SPEC(p..) ensures false {}` VERIFIES.
           No input satisfies the spec. It forbids everything, so any proof that
           assumes a legal call is unprovable. This is a real defect, and it is
           the check `training/BUG_REPORT.md` used for the 4B's Bugs 1-3.
  vacuous  `proof fn f(p..) ensures SPEC(p..) {}` VERIFIES.
           Every input satisfies the spec. It forbids nothing, so it would accept
           any implementation. BUG_REPORT Bug 4.

Both are decided by Z3 and neither consults an answer key, which is the entire
reason they are the metric here.

WHY --self-test EXISTS. This repo already contains a sweep of this shape that
reports 0/98 on specs with known bugs; `OPUS5_RESULTS.md` calls it "near-blind".
A sweep that finds nothing is indistinguishable from a sweep that is broken, so a
count from this script means nothing on its own. `--self-test` runs the detector
over fixtures reconstructed from BUG_REPORT's four bugs plus honest negatives,
and prints a confusion matrix. Run it in the same job, on the same Verus build,
and report its result next to any finding count.

The fixtures are RECONSTRUCTIONS. The 4B's `psci_generated_clean.rs` is on no
branch, so BUG_REPORT's four bugs cannot be re-run; the fixtures encode the bug
SHAPES that report describes. They validate the detector. They are not evidence
about the 4B, and a fixture hit must never be counted as a PSCI finding.
"""

import argparse
import json
import os
import re
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))

_SIG_RE = re.compile(
    r'pub\s+open\s+spec\s+fn\s+(\w+)\s*'
    r'(\((?:[^(){};]|\([^)]*\))*\))'
    r'\s*->\s*bool\s*\{',
    re.DOTALL,
)


def extract_fn_body(text, brace_start):
    depth, i = 0, brace_start
    while i < len(text):
        if text[i] == '{':
            depth += 1
        elif text[i] == '}':
            depth -= 1
            if depth == 0:
                return text[brace_start:i + 1]
        i += 1
    return text[brace_start:]


def split_params(inner):
    parts, depth, cur = [], 0, ''
    for ch in inner:
        if ch in '(<[':
            depth += 1
        elif ch in ')>]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur.strip())
            cur = ''
        else:
            cur += ch
    if cur.strip():
        parts.append(cur.strip())
    out = []
    for p in parts:
        p = p.strip()
        if not p:
            continue
        c = p.find(':')
        out.append((p[:c].strip(), p[c + 1:].strip()) if c > 0 else (p, '?'))
    return out


def is_trivial(body):
    inner = re.sub(r'//[^\n]*', '', body.strip().lstrip('{').rstrip('}')).strip()
    return inner in ('true', 'true;', '')


def parse_spec_fns(source):
    """Every `pub open spec fn ... -> bool { .. }` in a file.

    A trivially-`true` body is reported rather than skipped. It IS the vacuous
    case, and dropping it would hide the exact defect this sweep exists to find.
    """
    fns = []
    for m in _SIG_RE.finditer(source):
        body = extract_fn_body(source, m.end() - 1)
        fns.append({
            'name': m.group(1),
            'params_str': m.group(2),
            'params': split_params(m.group(2).strip().lstrip('(').rstrip(')')),
            'body': body,
            'trivial': is_trivial(body),
        })
    return fns


def _obligation(fn, kind):
    args = ', '.join(p[0] for p in fn['params'])
    call = f"{fn['name']}({args})"
    if kind == 'unsat':
        return (f"proof fn zz_unsat_{fn['name']}{fn['params_str']}\n"
                f"    requires {call}\n    ensures false\n{{}}")
    return (f"proof fn zz_vac_{fn['name']}{fn['params_str']}\n"
            f"    ensures {call}\n{{}}")


def build_case(preamble, fn, kind):
    """Preamble + this one spec fn + one proof obligation, as a whole crate.

    The preamble ends with `} // verus!`, so it is reopened rather than appended
    to: a proof fn outside the `verus!` block is plain Rust and would compile
    while proving nothing at all -- a silent all-clear.
    """
    # The closing brace is not the last character: the file ends `} // verus!`.
    # Cut at the last brace rather than requiring it to be final.
    cut = preamble.rstrip().rfind('}')
    assert cut != -1, "preamble has no closing brace for the verus! block"
    body = preamble.rstrip()[:cut].rstrip()
    fn_def = (f"pub open spec fn {fn['name']}{fn['params_str']} -> bool "
              f"{fn['body']}")
    return f"{body}\n\n{fn_def}\n\n{_obligation(fn, kind)}\n\n}} // verus!\n"


def run_verus(verus, path, timeout):
    """Compile one obligation file.

    cwd is the file's own directory because rustc writes its output archive to
    the CURRENT directory and names it after the crate, i.e. the source stem. Two
    threads compiling `vacuous.rs` therefore both write `libvacuous.rlib` into
    the shared cwd and tear each other's archive apart, which surfaces as

        failed to build archive at `libvacuous.rlib`: LLVM error: section table
        goes past the end of the file

    -- a linker error that this script's classifier then reported as
    `compile_error`, indistinguishable from a spec that genuinely does not
    compile. It cost 4 of 22 commands in the first run, all four of which do
    compile. The per-case stem below is the second half of the fix; either alone
    would do, and the failure is quiet enough to want both.
    """
    try:
        p = subprocess.run([str(verus), '--crate-type', 'lib', str(path.name)],
                           capture_output=True, text=True, timeout=timeout,
                           cwd=str(path.parent))
        return p.returncode, (p.stdout or '') + (p.stderr or '')
    except subprocess.TimeoutExpired:
        return -9, 'TIMEOUT'


def classify(rc, out):
    """(verdict, detail) for one obligation.

    `rc == 0` is the only outcome that means the obligation was PROVED. Every
    other outcome is separated by cause, because "Verus rejected it" covers both
    "the spec is fine" and "this file never compiled", and collapsing those is
    how a sweep reports a clean bill of health for specs it never checked.
    """
    if out == 'TIMEOUT':
        return 'timeout', 'verus timed out'
    if rc == 0:
        return 'proved', ''
    m = re.search(r'(\d+) verified, (\d+) error', out)
    if 'postcondition not satisfied' in out:
        return 'refuted', 'postcondition not satisfied'
    first = next((l for l in out.split('\n') if l.startswith('error')), '')
    # A failure to build or map the output archive is the toolchain, not the
    # spec. Kept as its own verdict rather than folded into compile_error: the
    # two are reported identically by rustc and mean opposite things about the
    # model, and the first run silently counted four working specs as broken.
    if re.search(r'failed to (build archive|map object file)|LLVM error', out):
        return 'toolchain_error', first[:200] or 'archive/link failure'
    if m and int(m.group(2)) > 0 and not first:
        return 'refuted', 'obligation failed'
    return 'compile_error', first[:200] or 'unknown'


def sweep_file(verus, preamble, path, timeout):
    src = Path(path).read_text(encoding='utf-8', errors='replace')
    fns = parse_spec_fns(src)
    if not fns:
        return [{'file': Path(path).name, 'fn': None, 'verdict': 'no_spec_fn'}]
    rows = []
    for fn in fns:
        row = {'file': Path(path).name, 'fn': fn['name'], 'trivial': fn['trivial']}
        with tempfile.TemporaryDirectory() as td:
            for kind in ('unsat', 'vacuous'):
                # Stem -> crate name -> output archive name. Unique per case so
                # concurrent workers cannot collide on one .rlib; see run_verus.
                stem = re.sub(r'\W', '_', f"{Path(path).stem}_{fn['name']}_{kind}")
                p = Path(td) / f"{stem}.rs"
                p.write_text(build_case(preamble, fn, kind))
                rc, out = run_verus(verus, p, timeout)
                row[kind] = classify(rc, out)[0]
                row[f'{kind}_detail'] = classify(rc, out)[1]
        # A PROVED obligation is decisive and outranks a failure on the other
        # one: `unsat` proved means no input satisfies the spec no matter what
        # the vacuity run did. Ordering this the other way is what let a linker
        # failure on the vacuity run mask four specs that compile.
        if row['unsat'] == 'proved':
            row['verdict'] = 'unsat'
        elif row['vacuous'] == 'proved':
            row['verdict'] = 'vacuous'
        elif row['unsat'] == 'compile_error' or row['vacuous'] == 'compile_error':
            row['verdict'] = 'compile_error'
        elif 'toolchain_error' in (row['unsat'], row['vacuous']):
            # Compiles -- one obligation got far enough to be refuted -- but the
            # other never ran. Not consistent, not broken: unmeasured.
            row['verdict'] = 'toolchain_error'
        elif 'timeout' in (row['unsat'], row['vacuous']):
            row['verdict'] = 'timeout'
        else:
            row['verdict'] = 'consistent'
        rows.append(row)
    return rows


# --------------------------------------------------------------------------
# Fixtures: the bug SHAPES from training/BUG_REPORT.md, plus honest negatives.
# --------------------------------------------------------------------------

FIXTURES = [
    # Bug 1 shape: one implication per error code, conjoined. When the antecedent
    # holds and the call errors, `code` must equal several distinct constants.
    ("bug1_conjoined_error_codes", "unsat", """
pub open spec fn fx_bug1(s: S, target: Bits64, err: bool, code: int) -> bool {
    (CpuIsValid(s, target) ==> (err ==> code == PSCI_ALREADY_ON))
    && (CpuIsValid(s, target) ==> (err ==> code == PSCI_ON_PENDING))
    && (CpuIsValid(s, target) ==> (err ==> code == PSCI_DENIED))
    && CpuIsValid(s, target) && err
}"""),
    # Bug 2 shape: overlapping antecedents, contradictory consequents.
    ("bug2_overlapping_antecedents", "unsat", """
pub open spec fn fx_bug2(s: S, target: Bits64, code: int) -> bool {
    (CpuIsValid(s, target) ==> code == PSCI_SUCCESS)
    && (CpuIsOn(s, target) ==> code == PSCI_ALREADY_ON)
    && CpuIsValid(s, target) && CpuIsOn(s, target)
}"""),
    # Bug 4 shape: every clause is about a free parameter, so the spec is a
    # tautology in its real arguments.
    ("bug4_free_parameter_tautology", "vacuous", """
pub open spec fn fx_bug4(s: S, target: Bits64, new_result: int) -> bool {
    new_result == new_result
}"""),
    # A body that is literally `true`.
    ("vacuous_literal_true", "vacuous", """
pub open spec fn fx_true(s: S, target: Bits64) -> bool {
    true
}"""),
    # NEGATIVES -- a detector that flags these is useless.
    ("ok_disjoint_error_codes", "consistent", """
pub open spec fn fx_ok1(s: S, target: Bits64, err: bool, code: int) -> bool {
    CpuIsValid(s, target) ==> (err ==> (code == PSCI_ALREADY_ON
                                        || code == PSCI_ON_PENDING
                                        || code == PSCI_DENIED))
}"""),
    ("ok_guarded_transition", "consistent", """
pub open spec fn fx_ok2(s: S, target: Bits64, code: int) -> bool {
    (CpuIsValid(s, target) && !CpuIsOn(s, target)) ==> code == PSCI_SUCCESS
}"""),
]


def self_test(verus, preamble, timeout):
    print("[self-test] fixtures reconstructed from training/BUG_REPORT.md\n")
    rows, ok = [], 0
    for name, expect, src in FIXTURES:
        fns = parse_spec_fns(src)
        assert len(fns) == 1, f"fixture {name} did not parse to one fn"
        with tempfile.TemporaryDirectory() as td:
            got = {}
            for kind in ('unsat', 'vacuous'):
                p = Path(td) / f"{re.sub(r'\W', '_', name)}_{kind}.rs"
                p.write_text(build_case(preamble, fns[0], kind))
                rc, out = run_verus(verus, p, timeout)
                got[kind] = classify(rc, out)
        if got['unsat'][0] == 'compile_error' or got['vacuous'][0] == 'compile_error':
            verdict = 'compile_error'
        elif got['unsat'][0] == 'proved':
            verdict = 'unsat'
        elif got['vacuous'][0] == 'proved':
            verdict = 'vacuous'
        else:
            verdict = 'consistent'
        hit = verdict == expect
        ok += hit
        rows.append({'fixture': name, 'expected': expect, 'got': verdict,
                     'pass': hit, 'detail': got})
        print(f"  {'PASS' if hit else 'FAIL'}  {name:34s} expected={expect:11s} got={verdict}")
        if not hit:
            print(f"        unsat={got['unsat']}  vacuous={got['vacuous']}")
    print(f"\n[self-test] {ok}/{len(FIXTURES)} fixtures classified correctly")
    if ok < len(FIXTURES):
        print("[self-test] DETECTOR IS NOT SOUND ON ITS OWN FIXTURES -- "
              "a finding count from this run is not interpretable.")
    return ok == len(FIXTURES), rows


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--gen-dir", help="directory of <command>.rs files")
    ap.add_argument("--preamble", default="training-dataset/specs/psci_13/preamble.rs")
    ap.add_argument("--out", default=None)
    ap.add_argument("--verus-bin", default=None)
    ap.add_argument("--timeout", type=int, default=180)
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--skip-self-test", action="store_true",
                    help="Sweep without validating the detector. The count is "
                         "then uninterpretable; only for debugging.")
    args = ap.parse_args()

    from verify_generated_verus import find_verus_bin, read_preamble
    verus = args.verus_bin or find_verus_bin(None)
    if not verus:
        sys.exit("verus not found - set VERUS_BIN")
    print(f"[sweep] verus: {verus}", flush=True)

    # read_preamble, not read_text: it rewrites `struct S` to `pub struct S`.
    # Without that every obligation fails to compile -- S is private and appears
    # in the public signature of every spec fn -- and the sweep reports
    # compile_error for all 22 commands, which reads as "nothing to see" rather
    # than "nothing was checked". The self-test catches it, so this is belt and
    # braces, but the failure is silent enough to deserve both.
    preamble = read_preamble(Path(args.preamble))

    sound, st_rows = (None, [])
    if args.self_test or not args.skip_self_test:
        sound, st_rows = self_test(verus, preamble, args.timeout)
        print()
    if args.self_test and not args.gen_dir:
        if args.out:
            Path(args.out).parent.mkdir(parents=True, exist_ok=True)
            Path(args.out).write_text(json.dumps(
                {'self_test': st_rows, 'sound': sound}, indent=2))
        return 0 if sound else 1

    if not args.gen_dir:
        sys.exit("--gen-dir is required unless --self-test is used alone")

    files = sorted(Path(args.gen_dir).glob("*.rs"))
    if not files:
        sys.exit(f"no .rs files in {args.gen_dir} - nothing to sweep, refusing "
                 f"to report 0 findings over an empty directory")
    print(f"[sweep] {len(files)} files from {args.gen_dir}", flush=True)

    rows = []
    with ThreadPoolExecutor(max_workers=args.jobs) as ex:
        for r in ex.map(lambda f: sweep_file(verus, preamble, f, args.timeout), files):
            rows.extend(r)
            done = len({x['file'] for x in rows})
            if done % 5 == 0:
                print(f"[sweep] {done}/{len(files)} files", flush=True)

    from collections import Counter
    c = Counter(r['verdict'] for r in rows)
    print("\n=== verdicts over %d spec fns in %d files ===" % (len(rows), len(files)))
    for k in ('unsat', 'vacuous', 'consistent', 'compile_error',
              'toolchain_error', 'timeout', 'no_spec_fn'):
        if c.get(k):
            print(f"  {k:14s} {c[k]}")
    findings = [r for r in rows if r['verdict'] in ('unsat', 'vacuous')]
    print(f"\nfindings (unsat + vacuous): {len(findings)}")
    for r in findings:
        print(f"  {r['verdict']:8s} {r['file']}::{r['fn']}")
    if sound is False:
        print("\n[sweep] NOTE: the detector failed its own fixtures above. "
              "Treat the count as unvalidated.")

    if args.out:
        Path(args.out).parent.mkdir(parents=True, exist_ok=True)
        Path(args.out).write_text(json.dumps(
            {'gen_dir': str(args.gen_dir), 'self_test': st_rows,
             'detector_sound': sound, 'counts': dict(c), 'rows': rows}, indent=2))
        print(f"\n[sweep] wrote {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
