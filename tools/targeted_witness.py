#!/usr/bin/env python3
"""
targeted_witness.py

The *targeted* half of the Z3 inconsistency analysis (the method used by
training/rmm_spec_bug_report.md's Bugs 4-6, and by BASELINE2 Iteration 7b to
find the RMI_RTT_SET_S2AP dual-error contradiction).

The blind sweep (training/inconsistency_analysis_model.py) asks "does
spec_fn(x) hold for ANY x" -- with fully unconstrained parameters that is
almost always satisfiable, so it is close to blind for V3_PROMPT-style output.
This tool instead lets you add *extra hypotheses* that narrow Z3 toward a
specific suspected overlap: two failure conditions that map to different error
codes with no stated ordering between them.

Two modes:

  --scan     rank commands by how likely they are to hide a dual-error
             contradiction: 3+ distinct RMI_ERROR_*/RSI_ERROR_* targets and no
             obvious mutual-exclusion guard between them.

  --command  build `proof fn ... requires <your hypotheses>, <spec_fn>(args)
             ensures false {}` for one command and run Verus.  Verified =>
             the hypotheses cannot hold together with the spec => confirmed
             inconsistency witness.

Examples:
    python3 tools/targeted_witness.py --scan \
        --results-root results/ab_test/v3_opus5/alp14

    python3 tools/targeted_witness.py \
        --results-root results/ab_test/v3_opus5/alp14 \
        --specs-dir training-dataset/specs/alp14 --verus $VERUS_BIN \
        --command rmi_rtt_set_s2ap \
        --requires '!AddrIsGranuleAligned(old_s, rd)' \
        --requires 'RecAt(old_s, rec_ptr).state == REC_RUNNING'
"""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "prompt_engineering"))

from verify_generated_verus import (  # noqa: E402
    extract_fn_block,
    find_verus_bin,
    parse_verified_errors,
    read_preamble,
    split_params,
)

_ERR_RE = re.compile(r"\b(RMI_ERROR_[A-Z_]+|RSI_ERROR_[A-Z_]+)\b")
# Cheap proxies for "these branches are explicitly kept apart".
_GUARD_RE = re.compile(r"\b(else|!\s*\(|&&\s*!)", re.I)


def scan(results_root: Path, generated_name: str) -> list[dict]:
    rows = []
    for d in sorted(p for p in results_root.iterdir() if p.is_dir()):
        gen = d / generated_name
        if not gen.exists():
            continue
        src = gen.read_text(encoding="utf-8", errors="ignore")
        errs = sorted(set(_ERR_RE.findall(src)))
        if len(errs) < 3:
            continue
        implications = src.count("==>")
        guards = len(_GUARD_RE.findall(src))
        rows.append({
            "command": d.name,
            "n_error_targets": len(errs),
            "errors": errs,
            "implications": implications,
            "guards": guards,
            # More distinct error codes, more implication branches, fewer
            # explicit guards => more room for two branches to fire at once.
            "score": len(errs) * 2 + implications - guards,
        })
    rows.sort(key=lambda r: -r["score"])
    return rows


def run_witness(verus_bin: Path, preamble: str, cmd: str, src: str,
                extra_requires: list[str], timeout_s: int) -> dict:
    fn_name, params_str, fn_text = extract_fn_block(src)
    if not fn_name:
        return {"command": cmd, "verdict": "skipped", "reason": "no_pub_open_spec_fn_found"}

    params = split_params(params_str)
    args = ", ".join(n for n, _ in params)
    clauses = [c.rstrip(",") for c in extra_requires] + [f"{fn_name}({args})"]
    requires_block = ",\n        ".join(clauses)

    proof = (
        f"proof fn witness_{fn_name}{params_str}\n"
        f"    requires\n        {requires_block},\n"
        f"    ensures false\n"
        f"{{}}"
    )
    test_src = (preamble + "\n\n" + fn_text
                + "\n\n// --- targeted inconsistency witness ---\n" + proof
                + "\n\n} // verus!\n")

    with tempfile.NamedTemporaryFile(mode="w", suffix=f"_{cmd}_witness.rs", delete=False) as tf:
        tf.write(test_src)
        tmp_path = Path(tf.name)
    try:
        proc = subprocess.run([str(verus_bin), "--crate-type", "lib", str(tmp_path)],
                              capture_output=True, text=True, timeout=timeout_s)
        out = (proc.stdout or "") + "\n" + (proc.stderr or "")
    except subprocess.TimeoutExpired:
        return {"command": cmd, "verdict": "timeout", "proof": proof}
    finally:
        tmp_path.unlink(missing_ok=True)

    verified, errors = parse_verified_errors(out)
    postcond_fail = "postcondition not satisfied" in out
    if errors == 0:
        verdict = "INCONSISTENT"
    elif postcond_fail:
        verdict = "satisfiable"
    else:
        verdict = "type_error"

    lines = out.strip().splitlines()
    i = next((k for k, l in enumerate(lines) if re.match(r"^error(\[|:)", l)), 0)
    return {
        "command": cmd, "fn_name": fn_name, "verdict": verdict,
        "verified": verified, "errors": errors,
        "proof": proof,
        "output_head": "\n".join(lines[i:i + 14]),
    }


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--results-root", required=True)
    ap.add_argument("--specs-dir", default="training-dataset/specs/alp14")
    ap.add_argument("--verus", default=None)
    ap.add_argument("--generated-name", default="generated.formatted.rs")
    ap.add_argument("--scan", action="store_true", help="Rank dual-error candidates and exit")
    ap.add_argument("--top", type=int, default=15)
    ap.add_argument("--command", default="", help="Command dir name (lowercase)")
    ap.add_argument("--requires", action="append", default=[],
                    help="Extra requires clause (repeatable)")
    ap.add_argument("--timeout", type=int, default=90)
    args = ap.parse_args()

    results_root = Path(args.results_root).resolve()

    if args.scan:
        rows = scan(results_root, args.generated_name)
        print(f"{'command':38s} {'#err':>4s} {'==>':>4s} {'grd':>4s} {'score':>5s}  errors")
        for r in rows[: args.top]:
            print(f"{r['command']:38s} {r['n_error_targets']:4d} {r['implications']:4d} "
                  f"{r['guards']:4d} {r['score']:5d}  {','.join(e.replace('RMI_ERROR_','RMI_').replace('RSI_ERROR_','RSI_') for e in r['errors'])}")
        print(f"\n{len(rows)} commands with 3+ distinct error targets")
        return

    if not args.command:
        raise SystemExit("Pass --scan or --command <name>")

    verus_bin = find_verus_bin(args.verus)
    if not verus_bin:
        raise SystemExit("Verus binary not found. Pass --verus /path/to/verus.")
    preamble = read_preamble(Path(args.specs_dir).resolve() / "preamble.rs")

    gen = results_root / args.command / args.generated_name
    if not gen.exists():
        raise SystemExit(f"No such generated file: {gen}")

    r = run_witness(verus_bin, preamble, args.command,
                    gen.read_text(encoding="utf-8", errors="ignore"),
                    args.requires, args.timeout)
    print(r["proof"])
    print("\n" + "=" * 60)
    print(f"{r['command']}: {r['verdict']}  (verified={r.get('verified')}, errors={r.get('errors')})")
    if r["verdict"] == "INCONSISTENT":
        print("=> The hypotheses cannot hold together with this spec: confirmed witness.")
    elif r["verdict"] == "satisfiable":
        print("=> Spec is satisfiable under these hypotheses (no contradiction here).")
    else:
        print("=> Could not decide; the wrapped code did not type-check:")
        print(r["output_head"])


if __name__ == "__main__":
    main()
