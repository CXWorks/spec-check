#!/usr/bin/env python3
"""
Verify generated Verus specs command-by-command (SCOPE-style):
- Build a standalone Verus test file per command from preamble + generated spec
- Run Verus on each file
- Report pass rate and full failure list

Default target:
  results/ab_test/v3/alp14/*/generated.formatted.rs
"""

from __future__ import annotations

import argparse
import json
import os
import re
import shutil
import subprocess
import tempfile
from dataclasses import dataclass, asdict
from pathlib import Path
from typing import Optional


ROOT = Path(__file__).resolve().parents[1]
DEFAULT_RESULTS_ROOT = ROOT / "results" / "ab_test" / "v3" / "alp14"
DEFAULT_SPECS_DIR = ROOT / "training-dataset" / "specs" / "alp14"


_SIG_RE = re.compile(
    r"pub\s+open\s+spec\s+fn\s+(\w+)\s*(\((?:[^(){};]|\([^)]*\))*\))\s*->\s*bool\s*\{",
    re.DOTALL,
)


@dataclass
class CheckResult:
    command: str
    status: str  # pass | fail | skipped
    reason: str
    returncode: Optional[int] = None
    verified: Optional[int] = None
    errors: Optional[int] = None
    fn_name: Optional[str] = None
    generated_file: Optional[str] = None
    output_head: Optional[str] = None


def find_verus_bin(cli_verus: Optional[str]) -> Optional[Path]:
    candidates: list[Path] = []

    if cli_verus:
        candidates.append(Path(cli_verus).expanduser())

    env_verus = os.getenv("VERUS_BIN")
    if env_verus:
        candidates.append(Path(env_verus).expanduser())

    for p in [
        shutil.which("verus"),
        str((Path.home() / "verus" / "source" / "target-verus" / "release" / "verus")),
        str((Path.home() / "verus" / "target-verus" / "release" / "verus")),
        str((ROOT / "training" / "verus-x86-linux" / "verus")),
        str((ROOT / "scope" / "target-verus" / "release" / "verus")),
    ]:
        if p:
            candidates.append(Path(p).expanduser())

    for c in candidates:
        try:
            if c.exists() and os.access(c, os.X_OK):
                return c
        except OSError:
            continue
    return None


def read_preamble(preamble_path: Path) -> str:
    text = preamble_path.read_text(encoding="utf-8", errors="ignore")
    # Match existing inconsistency scripts: make structs pub for standalone checks
    text = re.sub(r"(?m)^struct\b", "pub struct", text)
    return text.rstrip()


def extract_fn_block(source: str) -> tuple[Optional[str], Optional[str], Optional[str]]:
    """Return (fn_name, params_str, full_fn_text) for the first pub open spec fn."""
    m = _SIG_RE.search(source)
    if not m:
        return None, None, None
    fn_name = m.group(1)
    params_str = m.group(2)

    brace_start = m.end() - 1
    depth = 0
    i = brace_start
    while i < len(source):
        ch = source[i]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return fn_name, params_str, source[m.start() : i + 1]
        i += 1

    return fn_name, params_str, source[m.start():]


def split_params(params_str: str) -> list[tuple[str, str]]:
    inner = params_str.strip().lstrip("(").rstrip(")")
    if not inner.strip():
        return []

    out: list[str] = []
    current = ""
    depth = 0
    for ch in inner:
        if ch in "(<":
            depth += 1
        elif ch in ")>":
            depth -= 1
        if ch == "," and depth == 0:
            out.append(current.strip())
            current = ""
        else:
            current += ch
    if current.strip():
        out.append(current.strip())

    pairs: list[tuple[str, str]] = []
    for p in out:
        idx = p.find(":")
        if idx > 0:
            pairs.append((p[:idx].strip(), p[idx + 1 :].strip()))
        else:
            pairs.append((p.strip(), "?"))
    return pairs


def make_compile_probe(fn_name: str, params_str: str, params: list[tuple[str, str]]) -> str:
    # Force a well-typed call in a proof context (similar spirit to inconsistency check).
    args = ", ".join(name for name, _ in params)
    return (
        f"proof fn check_compile_{fn_name}{params_str}\n"
        f"    requires {fn_name}({args})\n"
        f"    ensures true\n"
        f"{{}}"
    )


def parse_verified_errors(output: str) -> tuple[Optional[int], Optional[int]]:
    m = re.search(r"(\d+) verified, (\d+) errors", output)
    if not m:
        return None, None
    return int(m.group(1)), int(m.group(2))


def extract_output_head(output: str, max_lines: int = 24) -> str:
    """Return the most relevant slice of verus output.

    The preamble alone emits hundreds of `uninterp` warnings before any real
    error, so a naive head-of-output slice usually shows only noise. Prefer
    starting from the first real `error`/`error[Exxx]` line; fall back to the
    raw head if no error line is present (e.g. timeouts, parse-only failures).
    """
    lines = output.strip().splitlines()
    err_idx = next((i for i, l in enumerate(lines) if re.match(r"^error(\[|:)", l)), None)
    if err_idx is not None:
        return "\n".join(lines[err_idx : err_idx + max_lines])
    return "\n".join(lines[:max_lines])


def classify_failure(output: str, returncode: int) -> str:
    """Bucket a failed compile by its first real error.

    Order matters, and the previous order was wrong: a bare `"expected"` test sat
    third and claimed everything below it for `parse_error`. `expected` appears in
    most rustc diagnostics — "expected 2 arguments, found 3", "expected struct
    `Foo`" — so arity and type errors were being reported as syntax errors, which
    is what the failure taxonomy was then read off. Specific patterns now come
    first and the syntax test is narrowed to phrasing only a parser produces.

    Anything unrecognised becomes `other_compile_error` rather than being folded
    into a named bucket: an honest unknown is worth more than a wrong label, and
    `output_head` is stored alongside so it can be read directly.
    """
    low = output.lower()

    # Verus's own verification verdict, not a compile failure at all.
    if "postcondition not satisfied" in low or "precondition not satisfied" in low:
        return "proof_obligation_failed"

    # Unbalanced brackets, reported separately from other syntax errors because
    # the cause is almost never the model's grammar: it is a generation that
    # stopped mid-expression, usually at the token cap. Labelling it
    # `parse_error` is what hid a decode-budget problem behind what looked like
    # the model writing invalid code.
    if ("mismatched closing delimiter" in low or "unclosed delimiter" in low
            or "unexpected closing delimiter" in low):
        return "unbalanced_delimiter"

    # Genuine syntax errors, tested first because parsing precedes everything
    # else: if the parser failed there are no name or type diagnostics to find.
    # Narrowed to phrasing only a parser emits — note that "expected one of ...,
    # found ..." would otherwise be caught by the type-mismatch pattern below.
    if ("parse error" in low or "unexpected token" in low
            or "expected one of" in low or "unexpected end of" in low):
        return "parse_error"

    # Names the model invented. Kept first among semantic errors: these are the
    # ones a symbol list at inference time could plausibly remove.
    if "unresolved" in low or "cannot find" in low or "not found in this scope" in low:
        return "missing_symbol"

    # Right name, wrong call. Distinct from a type mismatch and previously
    # invisible — it was landing in parse_error via "expected N arguments".
    if re.search(r"(takes|expected) \d+ arguments?|this (function|method) takes", low):
        return "wrong_arity"

    if ("mismatched types" in low or "type mismatch" in low
            or re.search(r"expected .{0,40}?, found ", low)):
        return "type_mismatch"

    if "no field" in low or "no method named" in low or "unknown field" in low:
        return "bad_field_access"

    if returncode != 0:
        return "other_compile_error"
    return "unknown"


def check_text(
    verus_bin: Path,
    preamble: str,
    cmd: str,
    src: str,
    timeout_s: int,
) -> CheckResult:
    """Verify generated Verus source text directly (no file backing required)."""
    fn_name, params_str, fn_text = extract_fn_block(src)
    if not fn_name or not params_str or not fn_text:
        return CheckResult(
            command=cmd,
            status="fail",
            reason="no_pub_open_spec_fn_found",
            output_head=src[:280].replace("\n", " "),
        )

    params = split_params(params_str)
    probe = make_compile_probe(fn_name, params_str, params)

    test_src = (
        preamble
        + "\n\n"
        + fn_text
        + "\n\n// --- compile probe ---\n"
        + probe
        + "\n\n} // verus!\n"
    )

    with tempfile.NamedTemporaryFile(mode="w", suffix=f"_{cmd}.rs", delete=False) as tf:
        tf.write(test_src)
        tmp_path = Path(tf.name)

    try:
        proc = subprocess.run(
            [str(verus_bin), "--crate-type", "lib", str(tmp_path)],
            capture_output=True,
            text=True,
            timeout=timeout_s,
        )
        out = (proc.stdout or "") + "\n" + (proc.stderr or "")
        verified, errors = parse_verified_errors(out)

        warning_only = verified is not None and errors == 0
        if proc.returncode == 0 or warning_only:
            return CheckResult(
                command=cmd,
                status="pass",
                reason="ok",
                returncode=proc.returncode,
                verified=verified,
                errors=errors,
                fn_name=fn_name,
            )

        return CheckResult(
            command=cmd,
            status="fail",
            reason=classify_failure(out, proc.returncode),
            returncode=proc.returncode,
            verified=verified,
            errors=errors,
            fn_name=fn_name,
            output_head=extract_output_head(out),
        )

    except subprocess.TimeoutExpired:
        return CheckResult(
            command=cmd,
            status="fail",
            reason="timeout",
            fn_name=fn_name,
        )
    finally:
        try:
            tmp_path.unlink(missing_ok=True)
        except Exception:
            pass


def check_one(
    verus_bin: Path,
    preamble: str,
    cmd_dir: Path,
    generated_name: str,
    timeout_s: int,
) -> CheckResult:
    cmd = cmd_dir.name
    gen_file = cmd_dir / generated_name

    if not gen_file.exists():
        return CheckResult(
            command=cmd,
            status="skipped",
            reason=f"missing_{generated_name}",
            generated_file=str(gen_file),
        )

    src = gen_file.read_text(encoding="utf-8", errors="ignore")
    result = check_text(verus_bin, preamble, cmd, src, timeout_s)
    result.generated_file = str(gen_file)
    return result


def main() -> None:
    ap = argparse.ArgumentParser(description="Verify generated Verus specs command-by-command")
    ap.add_argument("--results-root", default=str(DEFAULT_RESULTS_ROOT), help="Path like results/ab_test/v3/alp14")
    ap.add_argument("--specs-dir", default=str(DEFAULT_SPECS_DIR), help="Directory containing preamble.rs")
    ap.add_argument("--generated-name", default="generated.formatted.rs", choices=["generated.formatted.rs", "generated.raw.rs"])
    ap.add_argument("--oracle", action="store_true", help="Check oracle.formatted.rs instead of generated output")
    ap.add_argument("--verus", default=None, help="Path to verus binary (optional)")
    ap.add_argument("--timeout", type=int, default=45, help="Timeout per command (seconds)")
    ap.add_argument("--limit", type=int, default=0, help="Optional limit for debugging")
    ap.add_argument("--json-out", default="", help="Optional output JSON path")
    args = ap.parse_args()

    if args.oracle:
        target_name = "oracle.formatted.rs"
    else:
        target_name = args.generated_name

    results_root = Path(args.results_root).expanduser().resolve()
    specs_dir = Path(args.specs_dir).expanduser().resolve()
    preamble_path = specs_dir / "preamble.rs"

    if not results_root.exists():
        raise SystemExit(f"results root not found: {results_root}")
    if not preamble_path.exists():
        raise SystemExit(f"preamble not found: {preamble_path}")

    verus_bin = find_verus_bin(args.verus)
    if not verus_bin:
        raise SystemExit(
            "Verus binary not found. Set --verus /path/to/verus or VERUS_BIN env.\n"
            "Tried PATH, ~/verus/source/target-verus/release/verus, ~/verus/target-verus/release/verus, repo-local defaults."
        )

    preamble = read_preamble(preamble_path)
    cmd_dirs = sorted([p for p in results_root.iterdir() if p.is_dir()])
    if args.limit and args.limit > 0:
        cmd_dirs = cmd_dirs[: args.limit]

    all_results: list[CheckResult] = []
    print(f"[info] verus: {verus_bin}")
    print(f"[info] results_root: {results_root}")
    print(f"[info] preamble: {preamble_path}")
    print(f"[info] mode: {'oracle' if args.oracle else 'generated'}")
    print(f"[info] checking {len(cmd_dirs)} commands using {target_name}\n")

    for i, d in enumerate(cmd_dirs, start=1):
        r = check_one(verus_bin, preamble, d, target_name, args.timeout)
        all_results.append(r)
        print(f"[{i}/{len(cmd_dirs)}] {r.command}: {r.status} ({r.reason})")

    passed = [r for r in all_results if r.status == "pass"]
    failed = [r for r in all_results if r.status == "fail"]
    skipped = [r for r in all_results if r.status == "skipped"]

    checked = len(passed) + len(failed)
    pass_rate = (len(passed) / checked * 100.0) if checked else 0.0

    print("\n" + "=" * 72)
    print("Verus Check Summary")
    print("=" * 72)
    print(f"total_commands: {len(all_results)}")
    print(f"checked      : {checked}")
    print(f"pass         : {len(passed)}")
    print(f"fail         : {len(failed)}")
    print(f"skipped      : {len(skipped)}")
    print(f"pass_rate    : {pass_rate:.2f}%")

    if failed:
        print("\nFailed commands:")
        for r in failed:
            print(f"- {r.command}: {r.reason}")

    if skipped:
        print("\nSkipped commands:")
        for r in skipped:
            print(f"- {r.command}: {r.reason}")

    payload = {
        "verus_bin": str(verus_bin),
        "results_root": str(results_root),
        "preamble": str(preamble_path),
        "mode": "oracle" if args.oracle else "generated",
        "target_name": target_name,
        "summary": {
            "total_commands": len(all_results),
            "checked": checked,
            "pass": len(passed),
            "fail": len(failed),
            "skipped": len(skipped),
            "pass_rate": pass_rate,
        },
        "failed": [asdict(x) for x in failed],
        "skipped": [asdict(x) for x in skipped],
        "passed": [asdict(x) for x in passed],
    }

    default_suffix = "oracle_check_summary.json" if args.oracle else "verus_check_summary.json"
    json_out = Path(args.json_out).expanduser().resolve() if args.json_out else (
        results_root.parent / f"{results_root.name}_{default_suffix}"
    )
    json_out.write_text(json.dumps(payload, indent=2), encoding="utf-8")
    print(f"\n[info] wrote report: {json_out}")


if __name__ == "__main__":
    main()
