#!/usr/bin/env python3
"""
inconsistency_analysis_model.py

Automated Z3 `ensures false` sweep over a *model-generated* results tree
(results/ab_test/<variant>/<version>/<command>/generated.formatted.rs), rather
than over a single concatenated `*_generated_clean.rs` file the way
training/inconsistency_analysis.py does.

For every non-trivial generated spec function, emit

    proof fn check_inconsistency_<fn>(<same params>)
        requires <fn>(<args>)
        ensures false
    {}

on top of the version's real preamble and run Verus.  If that proof *verifies*,
no assignment of the parameters can satisfy the spec function -- i.e. the spec
is logically inconsistent (self-contradictory preconditions).  If Verus instead
reports "postcondition not satisfied", the spec is satisfiable (consistent).
Anything else is a type error in the generated code, reported separately so it
is never silently counted as "consistent".

This is the blind sweep referenced by BASELINE2_GENERAL_MODEL_COMPARISON.md
Iteration 7b; the file was missing from this checkout and is reconstructed here
from training/inconsistency_analysis.py's proof-emission logic plus
prompt_engineering/verify_generated_verus.py's preamble/extraction machinery,
so both paths wrap code identically.

Usage:
    python3 training/inconsistency_analysis_model.py \
        --results-root results/ab_test/v3_opus5/alp14 \
        --specs-dir    training-dataset/specs/alp14 \
        --verus        /path/to/verus \
        --json-out     results/ab_test/v3_opus5/alp14_inconsistency_sweep.json
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Optional

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "prompt_engineering"))

from verify_generated_verus import (  # noqa: E402
    extract_fn_block,
    find_verus_bin,
    parse_verified_errors,
    read_preamble,
    split_params,
)


def is_trivial(fn_text: str) -> bool:
    """True if the body is essentially `{ true }` (no constraints to contradict)."""
    brace = fn_text.find("{")
    if brace < 0:
        return True
    inner = fn_text[brace:].strip().lstrip("{").rstrip("}").strip()
    inner = re.sub(r"//[^\n]*", "", inner).strip()
    return inner in ("true", "true;", "")


def make_obligation(fn_name: str, params_str: str, params: list[tuple[str, str]]) -> str:
    args = ", ".join(name for name, _ in params)
    return (
        f"proof fn check_inconsistency_{fn_name}{params_str}\n"
        f"    requires {fn_name}({args})\n"
        f"    ensures false\n"
        f"{{}}"
    )


def check_one(verus_bin: Path, preamble: str, cmd: str, src: str, timeout_s: int) -> dict:
    fn_name, params_str, fn_text = extract_fn_block(src)
    if not fn_name or not params_str or not fn_text:
        return {"command": cmd, "verdict": "skipped", "reason": "no_pub_open_spec_fn_found"}

    if is_trivial(fn_text):
        return {"command": cmd, "verdict": "skipped", "reason": "trivial_true_body",
                "fn_name": fn_name}

    params = split_params(params_str)
    test_src = (
        preamble
        + "\n\n"
        + fn_text
        + "\n\n// --- inconsistency proof obligation ---\n"
        + make_obligation(fn_name, params_str, params)
        + "\n\n} // verus!\n"
    )

    with tempfile.NamedTemporaryFile(mode="w", suffix=f"_{cmd}_incons.rs", delete=False) as tf:
        tf.write(test_src)
        tmp_path = Path(tf.name)

    try:
        proc = subprocess.run(
            [str(verus_bin), "--crate-type", "lib", str(tmp_path)],
            capture_output=True, text=True, timeout=timeout_s,
        )
        out = (proc.stdout or "") + "\n" + (proc.stderr or "")
    except subprocess.TimeoutExpired:
        return {"command": cmd, "verdict": "timeout", "fn_name": fn_name,
                "reason": f"verus timeout after {timeout_s}s"}
    finally:
        tmp_path.unlink(missing_ok=True)

    verified, errors = parse_verified_errors(out)
    postcond_fail = "postcondition not satisfied" in out

    if errors == 0:
        verdict = "INCONSISTENT"     # ensures-false proof went through
    elif postcond_fail:
        verdict = "consistent"       # spec is satisfiable, as it should be
    else:
        verdict = "type_error"       # generated code does not type-check here

    head = ""
    if verdict == "type_error":
        lines = out.strip().splitlines()
        i = next((k for k, l in enumerate(lines) if re.match(r"^error(\[|:)", l)), 0)
        head = "\n".join(lines[i:i + 12])

    return {
        "command": cmd,
        "fn_name": fn_name,
        "verdict": verdict,
        "verified": verified,
        "errors": errors,
        "output_head": head,
    }


def main() -> None:
    ap = argparse.ArgumentParser(description="Z3 ensures-false sweep over generated specs")
    ap.add_argument("--results-root", required=True)
    ap.add_argument("--specs-dir", required=True)
    ap.add_argument("--verus", default=None)
    ap.add_argument("--generated-name", default="generated.formatted.rs")
    ap.add_argument("--timeout", type=int, default=60)
    ap.add_argument("--limit", type=int, default=0)
    ap.add_argument("--json-out", default="")
    args = ap.parse_args()

    verus_bin = find_verus_bin(args.verus)
    if not verus_bin:
        raise SystemExit("Verus binary not found. Pass --verus /path/to/verus.")

    results_root = Path(args.results_root).resolve()
    preamble = read_preamble(Path(args.specs_dir).resolve() / "preamble.rs")

    cmd_dirs = sorted(p for p in results_root.iterdir() if p.is_dir())
    if args.limit:
        cmd_dirs = cmd_dirs[: args.limit]

    print(f"[info] verus: {verus_bin}")
    print(f"[info] {len(cmd_dirs)} command dirs under {results_root}\n")

    results = []
    for i, d in enumerate(cmd_dirs, 1):
        gen = d / args.generated_name
        if not gen.exists():
            results.append({"command": d.name, "verdict": "skipped", "reason": "no_generated_file"})
            continue
        src = gen.read_text(encoding="utf-8", errors="ignore")
        r = check_one(verus_bin, preamble, d.name, src, args.timeout)
        results.append(r)
        print(f"[{i}/{len(cmd_dirs)}] {d.name}: {r['verdict']}"
              + (f" ({r.get('reason')})" if r.get("reason") else ""))

    counts: dict[str, int] = {}
    for r in results:
        counts[r["verdict"]] = counts.get(r["verdict"], 0) + 1

    inconsistent = [r["command"] for r in results if r["verdict"] == "INCONSISTENT"]

    print("\n" + "=" * 60)
    print("Inconsistency sweep summary")
    print("=" * 60)
    for k in sorted(counts):
        print(f"{k:14s}: {counts[k]}")
    if inconsistent:
        print(f"\nINCONSISTENT commands: {', '.join(inconsistent)}")
    else:
        print("\nNo logically inconsistent specs found by the blind sweep.")

    report = {
        "results_root": str(results_root),
        "generated_name": args.generated_name,
        "summary": counts,
        "inconsistent": inconsistent,
        "results": results,
    }
    out_path = Path(args.json_out) if args.json_out else results_root.parent / (
        results_root.name + "_inconsistency_sweep.json")
    out_path.write_text(json.dumps(report, indent=2), encoding="utf-8")
    print(f"\n[info] wrote {out_path}")


if __name__ == "__main__":
    main()
