#!/usr/bin/env python3
"""
repair_loop_verus.py

Self-repair agent loop for Verus-failing Qwen-generated specs: for each
command that currently fails verus verification, feed the real Verus
error back to the local Qwen model (item_split_v3_e2_best) and ask it to
fix its own previous output, up to --max-retries total attempts (the
already-generated failing candidate counts as attempt 1).

Each retry prompt is compact and stateless-per-round: system + spec +
"here is your one most recent code version and its error" — it does NOT
replay the full multi-turn conversation. A running list of one-line
"ruled out" summaries (one per prior attempt) is carried forward instead,
so the model knows what's already failed without the prompt growing
unboundedly across retries.

If a repair attempt passes verus, it replaces the saved generated.*.rs
files. If all attempts are exhausted without passing, the candidate with
the highest CodeBLEU against the oracle is kept, and the command is
recorded as still-failing.

Usage (on server):
    CUDA_VISIBLE_DEVICES=6 python3 repair_loop_verus.py \
        --verus-summary results/ab_test_qwen_v3retrained/v3_qwen_verus_check_summary.json \
        --results-root  results/ab_test_qwen_v3retrained/v3_qwen/alp14 \
        --specs-dir     specs/alp14 \
        --verus         verus_src/verus-x86-linux/verus \
        --limit 3   # smoke test; omit for all failing commands
"""

import argparse
import json
import os
import re
import sys
from pathlib import Path

os.environ.setdefault("CUDA_VISIBLE_DEVICES", "6")
os.environ.setdefault("HF_HOME", "/mnt/md0/zhushan/hf_cache")

SPEC_GEN = Path(__file__).resolve().parent
sys.path.insert(0, str(SPEC_GEN / "prompt_engineering"))

from dataset_loader import load_dataset  # noqa: E402
from prompt_engineering import (  # noqa: E402
    normalize_verus_with_verusfmt,
    compute_codebleu,
    resolve_verusfmt_binary,
)
from prompt_engineering_v3 import V3_PROMPT  # noqa: E402
from verify_generated_verus import find_verus_bin, read_preamble, check_text, check_one  # noqa: E402

from run_qwen_v3 import QwenLocalModel, MODEL_PATH  # noqa: E402


FIX_INSTRUCTIONS = (
    "Common causes seen in this model's output, and the exact fix for each:\n"
    "\n"
    "1) RMI_* commands: the `result` parameter's type must be "
    "`Result<(), RmiStatusCode>` (a real Rust Result — this is the ONLY type "
    "RMI commands ever use for `result`, always with `()` as the Ok type even "
    "when there are separate success-path output parameters). With this type, "
    "`result.is_Ok()` and `result.is_Err()` work correctly. Specific errors are "
    "asserted with `ResultEqual(result, RMI_ERROR_INPUT)`, `ResultEqual(result, "
    "RMI_ERROR_REALM(0))`, `ResultEqual(result, RMI_ERROR_RTT(level))`, etc. — "
    "never compare `result` to `RMI_SUCCESS`, it does not exist anywhere.\n"
    "   WRONG:   result: RmiCommandReturnCode   (bare struct — .is_Ok()/.is_Err() do not exist on it)\n"
    "   CORRECT: result: Result<(), RmiStatusCode>\n"
    "\n"
    "2) RSI_* commands: the `result` parameter's type must be the bare enum "
    "`RsiCommandReturnCode` (NOT wrapped in Result<...>). Compare it directly "
    "with `==`: `result == RSI_SUCCESS`, `result == RSI_ERROR_INPUT`, "
    "`result == RSI_ERROR_STATE`, `result == RSI_INCOMPLETE`, "
    "`result == RSI_ERROR_UNKNOWN`. Never call `.is_Ok()`/`.is_Err()` on it — "
    "it is a plain enum, not a Result.\n"
    "   WRONG:   result: Result<..., RsiCommandReturnCode>  or  result.is_Ok()\n"
    "   CORRECT: result: RsiCommandReturnCode   ...   result == RSI_SUCCESS\n"
    "\n"
    "3) Also check whether the parameters match what the specification text "
    "above actually describes: most commands take a `result` parameter with "
    "success/failure branching as above, but a few only describe an "
    "unconditional state transition with no distinguishable success/failure "
    "outcome. For those, drop the result parameter and any branching entirely:\n"
    "    pub open spec fn example_spec(old_s: S, new_s: S) -> bool {\n"
    "        CurrentRealm(new_s).state == SOME_FIXED_STATE\n"
    "    }\n"
    "\n"
    "4) Other causes: calling UInt(...) as if it were a function, or "
    "referencing a symbol not defined in the preamble.\n"
    "\n"
    "5) Wrong overload / wrong arity on a helper function: the preamble defines "
    "several near-identical helpers for different command families (e.g. "
    "`VersionEqual` for PSCI vs `VersionEqualRmi` for RMI vs `VersionEqualRsi` "
    "for RSI; `DeviceCommunicate` vs `DeviceCommunicate1` vs `DeviceCommunicate2` "
    "with different argument counts; `VdevAttestInfoEqual` (4 args: 3 ints + a "
    "RmmVdevAttestInfo) vs `VdevAttestInfoEqual1` (2 args: two RmmVdevAttestInfo); "
    "`RecAuxCount` and similar helpers that take a leading `s: S` state argument "
    "which is easy to drop by mistake). If Verus reports \"this function takes N "
    "arguments but M were supplied\" or \"expected function, found enum\", check "
    "the preamble for the exact matching overload for THIS command's family and "
    "its exact parameter list — do not guess the closest-looking name.\n"
    "\n"
    "6) \"expected int, found u64/i64\" type mismatch: the preamble helper you're "
    "calling declares that parameter as Verus's mathematical `int` type. Verus "
    "does NOT auto-coerce `u64`/`i64`/`UInt64` values (including struct fields "
    "and expressions like `level - 1`) into `int` — add an explicit `as int` "
    "cast at the call site.\n"
    "   WRONG:   AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start)\n"
    "   CORRECT: AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int)\n"
    "\n"
    "Fix the specification so it verifies; the signature may change if needed. "
    "Return only the corrected `pub open spec fn ...` block, nothing else."
)

_BACKTICK_IDENT_RE = re.compile(r"`([A-Za-z_][A-Za-z0-9_]*)`")
_DEF_RE_TMPL = r"\b(?:pub\s+)?(struct|enum|fn|const|type)\s+{name}\b"


def extract_symbol_snippets(preamble: str, error_text: str, max_snippets: int = 4, max_lines: int = 20) -> str:
    """Pull the preamble definitions of any identifiers named in a verus error.

    The model frequently gets the *type* right but the *member*/*variant*
    wrong (e.g. calling .is_Ok() on RmiCommandReturnCode, or confusing
    RMI_SUCCESS/RSI_SUCCESS). Grounding the fix turn with the actual
    definition text gives it something concrete to correct against instead
    of guessing again.
    """
    if not error_text:
        return ""

    seen = set()
    idents = []
    for m in _BACKTICK_IDENT_RE.finditer(error_text):
        name = m.group(1)
        if name not in seen:
            seen.add(name)
            idents.append(name)

    lines = preamble.splitlines()
    snippets = []
    for name in idents:
        if len(snippets) >= max_snippets:
            break
        pat = re.compile(_DEF_RE_TMPL.format(name=re.escape(name)))
        for i, line in enumerate(lines):
            if not pat.search(line):
                continue
            snippet_lines = [line]
            depth = line.count("{") - line.count("}")
            j = i + 1
            while depth > 0 and j < len(lines) and len(snippet_lines) < max_lines:
                snippet_lines.append(lines[j])
                depth += lines[j].count("{") - lines[j].count("}")
                j += 1
            snippets.append(f"// `{name}` is defined in the preamble as:\n" + "\n".join(snippet_lines))
            break

    return "\n\n".join(snippets)


def summarize_attempt(attempt_num: int, output_head: str) -> str:
    """One-line summary of an attempt's failure, for the ruled-out state list."""
    first_line = (output_head or "").strip().splitlines()[0] if output_head else "unknown error"
    return f"attempt {attempt_num}: {first_line}"


def build_repair_prompt(
    spec_user_msg: str,
    last_code: str,
    last_output_head: str,
    ruled_out: list[str],
    symbol_context: str,
) -> str:
    """Compact, stateless-per-round repair prompt.

    Deliberately does NOT replay the full multi-turn conversation history
    (that caused unbounded growth and context truncation). Instead it
    carries forward only: the single most recent code version, its error,
    and a compact bullet-point "ruled out" state built from one-line
    summaries of every prior attempt — so the model knows what's already
    been tried without re-reading full old code/error blocks.
    """
    head = last_output_head or "(no output captured)"
    ruled_out_block = ""
    if ruled_out:
        ruled_out_block = (
            "\n\nApproaches already tried and ruled out — do not repeat these:\n"
            + "\n".join(f"- {r}" for r in ruled_out)
        )
    ctx_block = f"\n\nRelevant definitions from the preamble:\n{symbol_context}\n" if symbol_context else ""
    return (
        f"{spec_user_msg}\n\n"
        f"--- Your previous attempt ---\n{last_code}\n\n"
        f"--- Verus error on that attempt ---\n{head}"
        f"{ruled_out_block}"
        f"{ctx_block}\n\n"
        f"{FIX_INSTRUCTIONS}"
    )


def load_failing_entries(summary_path: Path) -> list[dict]:
    data = json.loads(summary_path.read_text(encoding="utf-8"))
    return data["failed"]


def already_done(cmd_dir: Path, max_retries: int) -> bool:
    log_path = cmd_dir / "repair_log.json"
    if not log_path.exists():
        return False
    try:
        log = json.loads(log_path.read_text(encoding="utf-8"))
    except Exception:
        return False
    if log.get("resolved"):
        return True
    return log.get("attempts", 0) >= max_retries


def repair_one(
    model: QwenLocalModel,
    verus_bin: Path,
    preamble: str,
    sample,
    first_raw: str,
    first_reason: str,
    first_output_head: str,
    max_retries: int,
    timeout_s: int,
) -> dict:
    cmd = sample.command
    oracle_fmt = normalize_verus_with_verusfmt(sample.oracle)

    messages_dict = V3_PROMPT.format(sample.section_text, "", cmd, retrieved_rules="")
    system_msg = messages_dict["system"]
    spec_user_msg = messages_dict["user"]

    first_fmt = normalize_verus_with_verusfmt(first_raw)
    history = [
        {
            "attempt": 1,
            "raw": first_raw,
            "formatted": first_fmt,
            "status": "fail",
            "reason": first_reason,
            "output_head": first_output_head,
            "codebleu": compute_codebleu(first_fmt, oracle_fmt),
        }
    ]

    resolved = False
    attempt = 1
    seen_fmt = {first_fmt}
    # Compact accumulated state: one-line summaries of every ruled-out
    # attempt, carried forward instead of replaying full conversation
    # history. Keeps every prompt bounded regardless of retry count and
    # prevents oscillating between the same two wrong fixes.
    ruled_out: list[str] = [summarize_attempt(1, first_output_head)]
    # Escalate temperature across retries so a stuck 2-cycle (e.g. flipping
    # between the same two wrong fixes) has a chance to break out.
    temp_schedule = [0.7, 0.8, 0.9, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]

    while attempt < max_retries:
        attempt += 1
        last = history[-1]
        symbol_ctx = extract_symbol_snippets(preamble, last["output_head"])
        prompt = build_repair_prompt(spec_user_msg, last["raw"], last["output_head"], ruled_out, symbol_ctx)
        messages = [
            {"role": "system", "content": system_msg},
            {"role": "user", "content": prompt},
        ]

        temperature = temp_schedule[min(attempt - 2, len(temp_schedule) - 1)]
        raw = model.generate(messages, temperature=temperature)
        fmt = normalize_verus_with_verusfmt(raw)
        result = check_text(verus_bin, preamble, cmd, fmt, timeout_s)
        codebleu = compute_codebleu(fmt, oracle_fmt)

        history.append(
            {
                "attempt": attempt,
                "raw": raw,
                "formatted": fmt,
                "status": result.status,
                "reason": result.reason,
                "output_head": result.output_head,
                "codebleu": codebleu,
                "temperature": temperature,
            }
        )

        if result.status == "pass":
            resolved = True
            break

        if fmt in seen_fmt:
            # Repeating a previous candidate verbatim means it's cycling
            # without new information — stop burning attempts on it.
            break
        seen_fmt.add(fmt)
        ruled_out.append(summarize_attempt(attempt, result.output_head))

    if resolved:
        final = history[-1]
    else:
        final = max(history, key=lambda h: h["codebleu"])

    return {
        "command": cmd,
        "resolved": resolved,
        "attempts": len(history),
        "final_codebleu": final["codebleu"],
        "final_raw": final["raw"],
        "final_formatted": final["formatted"],
        "history": [
            {k: v for k, v in h.items() if k not in ("raw", "formatted")} for h in history
        ],
    }


def save_repair_result(cmd_dir: Path, outcome: dict) -> None:
    (cmd_dir / "generated.raw.rs").write_text(outcome["final_raw"], encoding="utf-8")
    (cmd_dir / "generated.formatted.rs").write_text(outcome["final_formatted"], encoding="utf-8")

    meta_path = cmd_dir / "meta.json"
    meta = {}
    if meta_path.exists():
        try:
            meta = json.loads(meta_path.read_text(encoding="utf-8"))
        except Exception:
            meta = {}
    candidate_scores = list(meta.get("candidate_scores") or [])
    candidate_scores.extend(h["codebleu"] for h in outcome["history"][1:])
    meta["n_samples"] = len(candidate_scores)
    meta["best_index"] = len(candidate_scores) - 1
    meta["best_codebleu"] = outcome["final_codebleu"]
    meta["candidate_scores"] = candidate_scores
    meta_path.write_text(json.dumps(meta, indent=2), encoding="utf-8")

    log_path = cmd_dir / "repair_log.json"
    log_path.write_text(
        json.dumps(
            {
                "command": outcome["command"],
                "resolved": outcome["resolved"],
                "attempts": outcome["attempts"],
                "final_codebleu": outcome["final_codebleu"],
                "history": outcome["history"],
            },
            indent=2,
        ),
        encoding="utf-8",
    )


def rerun_full_check(verus_bin: Path, preamble: str, results_root: Path, generated_name: str, timeout_s: int) -> dict:
    """Re-verify every command dir. Uses the same schema as
    verify_generated_verus.py's CLI output (a "failed" list of full
    CheckResult dicts, including output_head) so this can be fed straight
    back into --verus-summary for a follow-up repair round.
    """
    from dataclasses import asdict

    cmd_dirs = sorted(p for p in results_root.iterdir() if p.is_dir())
    passed, failed, skipped = [], [], []
    for d in cmd_dirs:
        r = check_one(verus_bin, preamble, d, generated_name, timeout_s)
        if r.status == "pass":
            passed.append(r)
        elif r.status == "fail":
            failed.append(r)
        else:
            skipped.append(r)
    checked = len(passed) + len(failed)
    return {
        "summary": {
            "total_commands": len(cmd_dirs),
            "checked": checked,
            "pass": len(passed),
            "fail": len(failed),
            "skipped": len(skipped),
            "pass_rate": (len(passed) / checked * 100.0) if checked else 0.0,
        },
        "failed": [asdict(r) for r in failed],
        "skipped": [asdict(r) for r in skipped],
        "passed": [asdict(r) for r in passed],
    }


def parse_args():
    p = argparse.ArgumentParser(description="Verus self-repair agent loop for Qwen-generated specs")
    p.add_argument("--verus-summary", required=True, help="Path to existing *_verus_check_summary.json")
    p.add_argument("--results-root", required=True, help="Directory containing per-command result dirs (e.g. results/.../v3_qwen/alp14)")
    p.add_argument("--specs-dir", required=True, help="Directory containing preamble.rs")
    p.add_argument("--verus", default=None, help="Path to verus binary (optional)")
    p.add_argument("--max-retries", type=int, default=10, help="Max total attempts per command (including the already-failing one)")
    p.add_argument("--timeout", type=int, default=45, help="Timeout per verus check (seconds)")
    p.add_argument("--limit", type=int, default=0, help="Only repair the first N failing commands (debugging)")
    p.add_argument("--resume", action="store_true", help="Skip commands already resolved/exhausted per repair_log.json")
    return p.parse_args()


def main():
    args = parse_args()

    results_root = Path(args.results_root).expanduser().resolve()
    specs_dir = Path(args.specs_dir).expanduser().resolve()
    summary_path = Path(args.verus_summary).expanduser().resolve()
    preamble_path = specs_dir / "preamble.rs"

    verus_bin = find_verus_bin(args.verus)
    if not verus_bin:
        raise SystemExit("Verus binary not found. Pass --verus /path/to/verus.")
    preamble = read_preamble(preamble_path)

    verusfmt = resolve_verusfmt_binary()
    print(f"[info] verus: {verus_bin}")
    print(f"[info] verusfmt: {verusfmt or 'not found (will skip formatting)'}")
    print(f"[info] results_root: {results_root}")

    failing = load_failing_entries(summary_path)
    if args.limit:
        failing = failing[: args.limit]
    print(f"[info] {len(failing)} failing commands to repair (max_retries={args.max_retries})\n")

    print("[data] Loading test split ...")
    dataset = load_dataset(split="test")
    by_command = {s.command.upper(): s for s in dataset}

    model = QwenLocalModel(str(MODEL_PATH))

    resolved_count = 0
    still_failing_count = 0

    for i, entry in enumerate(failing, 1):
        cmd = entry["command"]
        sample = by_command.get(cmd.upper())
        if sample is None:
            print(f"[{i}/{len(failing)}] {cmd}: [skip] not found in test split")
            continue

        cmd_dir = results_root / cmd.lower()
        if not cmd_dir.exists():
            print(f"[{i}/{len(failing)}] {cmd}: [skip] no result dir {cmd_dir}")
            continue

        if args.resume and already_done(cmd_dir, args.max_retries):
            log = json.loads((cmd_dir / "repair_log.json").read_text(encoding="utf-8"))
            resolved_count += 1 if log["resolved"] else 0
            still_failing_count += 0 if log["resolved"] else 1
            print(f"[{i}/{len(failing)}] {cmd}: [cached] resolved={log['resolved']} attempts={log['attempts']}")
            continue

        raw_path = cmd_dir / "generated.raw.rs"
        first_raw = raw_path.read_text(encoding="utf-8", errors="ignore") if raw_path.exists() else "ERROR"

        outcome = repair_one(
            model,
            verus_bin,
            preamble,
            sample,
            first_raw,
            entry.get("reason", "unknown"),
            entry.get("output_head", ""),
            args.max_retries,
            args.timeout,
        )
        save_repair_result(cmd_dir, outcome)

        if outcome["resolved"]:
            resolved_count += 1
        else:
            still_failing_count += 1

        print(
            f"[{i}/{len(failing)}] {cmd}: "
            f"{'RESOLVED' if outcome['resolved'] else 'still failing'} "
            f"after {outcome['attempts']} attempt(s), codebleu={outcome['final_codebleu']:.3f}"
        )

    print("\n" + "=" * 60)
    print("Repair loop summary")
    print("=" * 60)
    print(f"resolved      : {resolved_count}")
    print(f"still_failing : {still_failing_count}")

    print("\n[info] Re-running full verus check over all commands ...")
    final = rerun_full_check(verus_bin, preamble, results_root, "generated.formatted.rs", args.timeout)
    s = final["summary"]
    print(
        f"[info] pass rate after repair: {s['pass']}/{s['checked']} "
        f"({s['pass_rate']:.2f}%)"
    )

    out_path = summary_path.parent / f"{summary_path.stem}_repaired.json"
    out_path.write_text(json.dumps(final, indent=2), encoding="utf-8")
    print(f"[info] wrote {out_path}")


if __name__ == "__main__":
    main()
