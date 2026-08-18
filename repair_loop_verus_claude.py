#!/usr/bin/env python3
"""
repair_loop_verus_claude.py

Same self-repair agent loop as repair_loop_verus.py, but targeting
Claude-generated specs (prompt_engineering_v3.py's V3-Structured track)
instead of the locally fine-tuned Qwen model. Two differences from the
Qwen version, beyond swapping the model:

1) The Qwen version calls V3_PROMPT.format(..., "", cmd, ...) with an
   empty context, because the fine-tuned Qwen model had the preamble
   baked into its training data and V3_PROMPT's template historically
   had no {context} placeholder anyway. Since prompt_engineering_v3.py's
   template now includes a real {context} block (Claude was never
   fine-tuned and needs to see the preamble to call its helpers
   correctly), this version passes sample.preamble through for real.

2) Uses ClaudeModel (Anthropic API) instead of QwenLocalModel (local
   HF model), including the temperature-escalation schedule already used
   by the Qwen repair loop to break out of stuck 2-cycles.

Usage (on server, needs `pip install anthropic` and ANTHROPIC_API_KEY set):
    python3 repair_loop_verus_claude.py \
        --verus-summary results/ab_test/v3/alp14_verus_check_summary.json \
        --results-root  results/ab_test/v3/alp14 \
        --specs-dir     training-dataset/specs/alp14 \
        --verus         verus_src/verus-x86-linux/verus \
        --limit 3   # smoke test; omit for all failing commands
"""

import argparse
import json
import os
import sys
from pathlib import Path

SPEC_GEN = Path(__file__).resolve().parent
sys.path.insert(0, str(SPEC_GEN / "prompt_engineering"))

from dataset_loader import load_dataset  # noqa: E402
from prompt_engineering import (  # noqa: E402
    ClaudeModel,
    normalize_verus_with_verusfmt,
    compute_codebleu,
    resolve_verusfmt_binary,
    load_dotenv_fallback,
)
from prompt_engineering_v3 import V3_PROMPT  # noqa: E402
from verify_generated_verus import find_verus_bin, read_preamble, check_text, check_one  # noqa: E402

from repair_loop_verus import (  # noqa: E402
    FIX_INSTRUCTIONS,
    extract_symbol_snippets,
    summarize_attempt,
    build_repair_prompt,
    load_failing_entries,
    already_done,
    save_repair_result,
    rerun_full_check,
)


def repair_one(
    model: ClaudeModel,
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

    # Pass the real preamble (unlike the Qwen repair loop's ""), since
    # Claude was never fine-tuned on it and needs it in-context.
    messages_dict = V3_PROMPT.format(sample.section_text, sample.preamble, cmd, retrieved_rules="")
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
    ruled_out: list[str] = [summarize_attempt(1, first_output_head)]
    # Anthropic API temperature range is [0, 1]; escalate to break stuck cycles.
    temp_schedule = [0.6, 0.7, 0.8, 0.9, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]

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


def parse_args():
    p = argparse.ArgumentParser(description="Verus self-repair agent loop for Claude-generated specs")
    p.add_argument("--verus-summary", required=True, help="Path to existing *_verus_check_summary.json")
    p.add_argument("--results-root", required=True, help="Directory containing per-command result dirs (e.g. results/ab_test/v3/alp14)")
    p.add_argument("--specs-dir", required=True, help="Directory containing preamble.rs")
    p.add_argument("--verus", default=None, help="Path to verus binary (optional)")
    p.add_argument("--api-key", default=None, help="Anthropic API key (else ANTHROPIC_API_KEY env / .env)")
    p.add_argument("--model", default=None, help="Claude model ID (default: claude-haiku-4-5-20251001). E.g. claude-opus-4-8")
    p.add_argument("--effort", default=None, choices=["low", "medium", "high", "xhigh", "max"], help="output_config.effort, only used on models with adaptive thinking (i.e. not Haiku)")
    p.add_argument("--max-retries", type=int, default=10, help="Max total attempts per command (including the already-failing one)")
    p.add_argument("--timeout", type=int, default=45, help="Timeout per verus check (seconds)")
    p.add_argument("--limit", type=int, default=0, help="Only repair the first N failing commands (debugging)")
    p.add_argument("--resume", action="store_true", help="Skip commands already resolved/exhausted per repair_log.json")
    return p.parse_args()


def main():
    args = parse_args()
    load_dotenv_fallback(SPEC_GEN / ".env")

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

    api_key = args.api_key or os.getenv("ANTHROPIC_API_KEY")
    if not api_key:
        raise SystemExit("ANTHROPIC_API_KEY not set. Pass --api-key or set the env var / .env.")
    model = ClaudeModel(api_key=api_key, model=args.model, effort=args.effort)

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
