#!/usr/bin/env python3
"""
prewarm_cache.py

Second generation worker that fills ClaudeCLIModel's on-disk response cache
from the *opposite end* of the command list, so the main sequential run finds
its calls already cached when it gets there.

Why this is safe:
  * It calls the very same evaluate_prompt_variant() code path with the same
    V3_PROMPT and the same RAG retriever, so the prompts -- and therefore the
    cache keys (sha256 of model+effort+system+user) -- are byte-identical to
    what the main run will compute.  Anything else would silently miss.
  * It walks the dataset in REVERSE while the main run walks forward, so the
    two processes work on disjoint cache keys until they meet.  Cache writes
    are atomic (tmp + os.replace); the only race is a lost append if both hit
    the same command at the crossover, which just regenerates later.
  * Its own results are written to a throwaway variant key, so it never
    touches the real results tree.  Only the shared cache matters.

Usage:
    python3 tools/prewarm_cache.py --n-samples 5 --skip-done
"""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "prompt_engineering"))

from dataset_loader import load_dataset  # noqa: E402
from prompt_engineering import evaluate_prompt_variant, ROOT_DIR  # noqa: E402
from prompt_engineering_v3 import V3_PROMPT  # noqa: E402
from claude_cli_model import build_cli_model_for_results  # noqa: E402


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--key", default="v3_opus5", help="Real run's variant key (for cache location)")
    ap.add_argument("--warm-key", default="v3_opus5_warm", help="Throwaway results key")
    ap.add_argument("--n-samples", type=int, default=5)
    ap.add_argument("--model", default="claude-opus-5")
    ap.add_argument("--effort", default="high")
    ap.add_argument("--rag-index", default="/mnt/sdc/xiang/spec-check/rag/index.json")
    ap.add_argument("--rag-top-k", type=int, default=3)
    ap.add_argument("--skip-done", action="store_true",
                    help="Skip commands the real run has already completed")
    args = ap.parse_args()

    results_root = ROOT_DIR / "results" / "ab_test"
    real_dir = results_root / args.key / "alp14"

    dataset = load_dataset(split="test")

    if args.skip_done:
        def done(s) -> bool:
            m = real_dir / s.command.lower() / "meta.json"
            if not m.exists():
                return False
            import json
            try:
                return len(json.loads(m.read_text()).get("candidate_scores") or []) >= args.n_samples
            except Exception:
                return False
        dataset = [s for s in dataset if not done(s)]

    dataset = list(reversed(dataset))
    print(f"[warm] {len(dataset)} commands to pre-warm (reverse order)")

    retriever = None
    if args.rag_top_k > 0:
        sys.path.insert(0, str(ROOT_DIR))
        from rag.retriever import RuleRetriever
        retriever = RuleRetriever(args.rag_index)

    # Same cache dir as the real run -- that is the whole point.
    model = build_cli_model_for_results(
        results_root / args.key, model=args.model, effort=args.effort, stage="prewarm")
    print(f"[warm] cache: {model.cache_dir}")

    evaluate_prompt_variant(
        V3_PROMPT,
        variant_key=args.warm_key,
        dataset=dataset,
        model=model,
        limit=len(dataset),
        n_samples=args.n_samples,
        save_results=True,
        resume=True,
        retriever=retriever,
        rag_top_k=args.rag_top_k,
    )
    print(f"[warm] done: {model.summary()}")


if __name__ == "__main__":
    main()
