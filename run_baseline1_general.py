#!/usr/bin/env python3
"""
run_baseline1_general.py

Baseline 1 (see BASELINE1_SCOPE_REPRODUCTION.md) with general SOTA models
instead of our fine-tuned Qwen: generate Verus spec functions for the eac5/rel0
RMM command sets through the `codex` / `claude` subscription CLIs, so the same
SCOPE dangling-output check can be applied to their output.

Sampling is round-based: each invocation adds exactly one sample per command
(`--round N`), so an n=3 evaluation is three separate, individually reportable
runs. Everything is idempotent -- a round skips commands that already have their
sample on disk, so a run killed by a 5-hour subscription limit resumes with the
same command line.

Usage:
    python3 run_baseline1_general.py --model codex  --round 1
    python3 run_baseline1_general.py --model claude --round 1
    python3 run_baseline1_general.py --model codex  --round 1 --limit 2   # smoke test
"""

import argparse
import json
import re
import sys
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / "prompt_engineering"))

from dataset_loader import load_version                                   # noqa: E402
from prompt_engineering import (                                          # noqa: E402
    PromptVariant,
    compute_codebleu,
    format_retrieved_rules_block,
    normalize_verus_with_verusfmt,
    resolve_verusfmt_binary,
)
from prompt_engineering_v3 import PROMPT_V3_SYSTEM                        # noqa: E402
from cli_models import MODELS, QuotaExhausted                             # noqa: E402

# Iteration 6 dropped the preamble from the template because the fine-tuned Qwen
# had it baked into its training examples. The general models have never seen
# this DSL, so it is restored here -- the same call Baseline 2 made for GPT.
PROMPT_V3_TEMPLATE_WITH_PREAMBLE = """Context -- Verus type, constant and helper signatures available to you (copy names from here verbatim):
{context}

Command specification text:
{spec}

{retrieved_rules}Signature: pub open spec fn {cmd_name_lower}_spec(...) -> bool
Prefer Bits64/UInt64/UInt32 aliases when present in spec, but do not sacrifice semantic correctness for alias formatting.
Keep unchanged-state constraints when implied by the command behavior."""

V3_WITH_PREAMBLE = PromptVariant(
    "V3-Structured+Preamble", PROMPT_V3_SYSTEM, PROMPT_V3_TEMPLATE_WITH_PREAMBLE
)

MODEL_KEYS = {"codex": "gpt56sol", "claude": "claude_opus5"}
SAMPLE_RE = re.compile(r"^sample_(\d+)\.formatted\.rs$")


def parse_args():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--model", choices=sorted(MODELS), required=True)
    ap.add_argument("--round", type=int, required=True,
                    help="Sample index to produce (1-based); existing samples are kept")
    ap.add_argument("--versions", nargs="+", default=["eac5", "rel0"])
    ap.add_argument("--results-root", type=Path, default=ROOT / "results" / "baseline1_general")
    ap.add_argument("--rag-index", default=str(ROOT / "rag" / "index"))
    ap.add_argument("--rag-top-k", type=int, default=3)
    ap.add_argument("--limit", type=int, default=0, help="Only the first N commands per version")
    ap.add_argument("--workers", type=int, default=1,
                    help="Parallel CLI calls (default 1; raise carefully, subscriptions rate-limit)")
    ap.add_argument("--timeout", type=int, default=600, help="Per-call timeout in seconds")
    ap.add_argument("--model-name", default=None, help="Override the CLI model id")
    ap.add_argument("--effort", default="high")
    ap.add_argument("--force", action="store_true", help="Regenerate this round even if present")
    return ap.parse_args()


def existing_samples(cmd_dir: Path):
    """Return {round_index: formatted_text} for samples already on disk."""
    out = {}
    if not cmd_dir.exists():
        return out
    for p in cmd_dir.iterdir():
        m = SAMPLE_RE.match(p.name)
        if m:
            text = p.read_text(encoding="utf-8", errors="ignore")
            if text.strip():
                out[int(m.group(1))] = text
    return out


def refresh_best(cmd_dir: Path, sample, oracle_fmt: str, model_label: str):
    """Recompute the best-by-CodeBLEU sample and rewrite generated.*.rs + meta.json."""
    samples = existing_samples(cmd_dir)
    if not samples:
        return None

    scores = {}
    for idx, text in samples.items():
        scores[idx] = compute_codebleu(text, oracle_fmt)

    best_idx = max(scores, key=lambda i: scores[i])
    best_fmt = samples[best_idx]
    raw_path = cmd_dir / f"sample_{best_idx}.raw.rs"
    best_raw = raw_path.read_text(encoding="utf-8", errors="ignore") if raw_path.exists() else best_fmt

    (cmd_dir / "generated.formatted.rs").write_text(best_fmt, encoding="utf-8")
    (cmd_dir / "generated.raw.rs").write_text(best_raw, encoding="utf-8")
    (cmd_dir / "meta.json").write_text(json.dumps({
        "command": sample.command,
        "version": sample.version,
        "model": model_label,
        "rounds": sorted(samples),
        "candidate_scores": {str(k): round(v, 4) for k, v in sorted(scores.items())},
        "best_round": best_idx,
        "best_codebleu": round(scores[best_idx], 4),
    }, indent=2) + "\n", encoding="utf-8")
    return scores


def build_retriever(args):
    if args.rag_top_k <= 0:
        return None
    from rag.retriever import RuleRetriever
    r = RuleRetriever(args.rag_index)
    print(f"[info] RAG enabled: top_k={args.rag_top_k}, index={args.rag_index}")
    return r


def main():
    args = parse_args()
    model_key = MODEL_KEYS[args.model]
    out_root = args.results_root / model_key

    fmt_bin = resolve_verusfmt_binary()
    print(f"[info] verusfmt: {fmt_bin or '<not found -- CodeBLEU will use unformatted text>'}")

    retriever = build_retriever(args)

    model_cls = MODELS[args.model]
    kwargs = dict(timeout=args.timeout, log_path=out_root / "calls.jsonl")
    if args.model_name:
        kwargs["model"] = args.model_name
    kwargs["effort"] = args.effort
    model = model_cls(**kwargs)
    print(f"[info] model: {model.name}  round: {args.round}")

    totals = {}
    quota_hit = False

    for version in args.versions:
        dataset = load_version(version)
        if args.limit:
            dataset = dataset[:args.limit]
        print(f"\n=== {version}: {len(dataset)} commands ===")

        # Pre-compute oracle formatting + prompts serially (cheap, no model calls).
        jobs = []
        for sample in dataset:
            cmd_dir = out_root / version / sample.command.lower()
            cmd_dir.mkdir(parents=True, exist_ok=True)
            oracle_fmt_path = cmd_dir / "oracle.formatted.rs"
            if oracle_fmt_path.exists():
                oracle_fmt = oracle_fmt_path.read_text(encoding="utf-8")
            else:
                oracle_fmt = normalize_verus_with_verusfmt(sample.oracle.strip())
                oracle_fmt_path.write_text(oracle_fmt, encoding="utf-8")
                (cmd_dir / "oracle.raw.rs").write_text(sample.oracle.strip(), encoding="utf-8")
            jobs.append((sample, cmd_dir, oracle_fmt))

        def run_one(job):
            sample, cmd_dir, oracle_fmt = job
            target = cmd_dir / f"sample_{args.round}.formatted.rs"
            if target.exists() and target.read_text(encoding="utf-8").strip() and not args.force:
                return sample.command, None, "cached"

            rules_block = ""
            if retriever is not None:
                try:
                    hits = retriever.search(f"{sample.command}\n{sample.section_text}",
                                            top_k=args.rag_top_k)
                    rules_block = format_retrieved_rules_block(hits)
                except Exception as e:                       # noqa: BLE001
                    print(f"    [warn] RAG failed for {sample.command}: {e}")

            msgs_dict = V3_WITH_PREAMBLE.format(
                sample.section_text, sample.preamble, sample.command,
                retrieved_rules=(rules_block + "\n\n") if rules_block else "",
            )
            messages = [
                {"role": "system", "content": msgs_dict["system"]},
                {"role": "user", "content": msgs_dict["user"]},
            ]

            raw = model.generate(messages)
            formatted = normalize_verus_with_verusfmt(raw)
            (cmd_dir / f"sample_{args.round}.raw.rs").write_text(raw, encoding="utf-8")
            target.write_text(formatted, encoding="utf-8")
            return sample.command, compute_codebleu(formatted, oracle_fmt), "generated"

        started = time.time()
        done = 0
        try:
            if args.workers > 1:
                with ThreadPoolExecutor(max_workers=args.workers) as ex:
                    results = list(ex.map(run_one, jobs))
            else:
                results = []
                for job in jobs:
                    results.append(run_one(job))
                    done += 1
                    cmd, score, status = results[-1]
                    tag = f"{score:.3f}" if score is not None else status
                    print(f"  [{done}/{len(jobs)}] {cmd}: {tag} "
                          f"({(time.time() - started) / done:.0f}s/cmd avg)")
        except QuotaExhausted as e:
            print(f"\n!! {e}")
            print("!! Stopping cleanly. Resume with the identical command line:")
            print(f"   python3 {Path(__file__).name} --model {args.model} --round {args.round} "
                  f"--versions {' '.join(args.versions)}")
            quota_hit = True
            results = []

        # Refresh best-of + metadata for every command that has any sample.
        per_cmd = {}
        for sample, cmd_dir, oracle_fmt in jobs:
            scores = refresh_best(cmd_dir, sample, oracle_fmt, model.name)
            if scores:
                per_cmd[sample.command] = scores

        if per_cmd:
            best_at = {}
            for k in range(1, args.round + 1):
                vals = [max((s[i] for i in s if i <= k), default=0.0) for s in per_cmd.values()]
                best_at[f"best@{k}"] = round(sum(vals) / len(vals), 4)
            totals[version] = {"commands": len(per_cmd), **best_at}
            print(f"  {version}: " + "  ".join(f"{k}={v:.4f}" for k, v in best_at.items()))

        if quota_hit:
            break

    summary_path = out_root / f"summary_round{args.round}.json"
    summary_path.parent.mkdir(parents=True, exist_ok=True)
    summary_path.write_text(json.dumps({
        "model": model.name,
        "round": args.round,
        "rag_top_k": args.rag_top_k,
        "prompt": V3_WITH_PREAMBLE.name,
        "verusfmt": fmt_bin,
        "quota_exhausted": quota_hit,
        "per_version": totals,
    }, indent=2) + "\n", encoding="utf-8")
    print(f"\nWrote {summary_path}")

    if quota_hit:
        sys.exit(2)


if __name__ == "__main__":
    main()
