#!/usr/bin/env python3
"""
run_qwen_v3.py

Run the V3 structured prompt with the locally trained Qwen model
(item_split_e2_best) instead of Claude, then report CodeBLEU.

Usage (on server):
    CUDA_VISIBLE_DEVICES=6,7 python3 run_qwen_v3.py
    CUDA_VISIBLE_DEVICES=6,7 python3 run_qwen_v3.py --limit 10   # smoke test
    CUDA_VISIBLE_DEVICES=6,7 python3 run_qwen_v3.py --resume      # resume partial run
"""

import argparse
import os
import re
import sys
from pathlib import Path

# ── GPU & cache config ────────────────────────────────────────────────────────
# Override from environment if already set (e.g. CUDA_VISIBLE_DEVICES=6,7)
os.environ.setdefault("CUDA_VISIBLE_DEVICES", "6,7")
os.environ.setdefault("HF_HOME", "/mnt/md0/zhushan/hf_cache")

# ── Paths ─────────────────────────────────────────────────────────────────────
SPEC_GEN   = Path(__file__).resolve().parent          # repo root (spec-gen on server)
MODEL_PATH = SPEC_GEN / "models" / "item_split_v5_best"
RESULTS_ROOT = SPEC_GEN / "results" / "ab_test_qwen_v5"

# Add prompt_engineering/ to import path
sys.path.insert(0, str(SPEC_GEN / "prompt_engineering"))

from dataset_loader import load_dataset
from prompt_engineering import (
    evaluate_prompt_variant,
    aggregate_metrics,
    resolve_verusfmt_binary,
)
from prompt_engineering_v3 import V3_PROMPT


# ── Local Qwen model wrapper ──────────────────────────────────────────────────

class QwenLocalModel:
    """
    Drop-in replacement for ClaudeModel that runs inference
    on the locally trained Qwen LoRA adapter.
    """

    def __init__(self, model_path: str):
        from unsloth import FastLanguageModel

        self.call_count = 0
        print(f"[model] Loading Qwen from {model_path} ...")
        self.model, self.tokenizer = FastLanguageModel.from_pretrained(
            model_path,
            max_seq_length=12288,
            load_in_4bit=True,
            dtype=None,           # auto: fp16 on Turing GPUs
        )
        FastLanguageModel.for_inference(self.model)
        print("[model] Ready.")

    def generate(self, messages: list, temperature: float = 0.1) -> str:
        import torch

        self.call_count += 1

        # Pass the full conversation through (system, user, and any
        # assistant/user follow-up turns) so multi-turn repair loops work.
        ids = self.tokenizer.apply_chat_template(
            messages,
            return_tensors="pt",
            add_generation_prompt=True,
        ).to("cuda")

        with torch.no_grad():
            out = self.model.generate(
                ids,
                max_new_tokens=6144,
                temperature=max(temperature, 1e-4),
                do_sample=temperature > 0,
            )

        text = self.tokenizer.decode(out[0][ids.shape[1]:], skip_special_tokens=True)

        # Qwen3 outputs a <think>…</think> block before the answer — strip it
        text = re.sub(r"<think>.*?</think>\s*", "", text, flags=re.DOTALL).strip()
        return text


# ── Main ──────────────────────────────────────────────────────────────────────

def parse_args():
    p = argparse.ArgumentParser(description="V3 prompt + Qwen local model")
    p.add_argument("--limit",    type=int, default=None,
                   help="Max commands to evaluate (default: all 98)")
    p.add_argument("--resume",   action="store_true",
                   help="Reuse already-saved results, rerun only missing")
    p.add_argument("--rag-index", default=None,
                   help="Path to RAG index.pkl (e.g. rag/index.pkl)")
    p.add_argument("--rag-top-k", type=int, default=0,
                   help="Number of retrieved rules to inject (0 = disabled)")
    return p.parse_args()


def main():
    args = parse_args()

    verusfmt = resolve_verusfmt_binary()
    print(f"[info] verusfmt: {verusfmt or 'not found (will skip formatting)'}")

    # Load alp14 test split
    print("[data] Loading test split ...")
    dataset = load_dataset(split="test")
    if not dataset:
        print("[error] No data loaded — make sure training-dataset/ symlinks exist:")
        print("  ln -sf /mnt/md0/zhushan/spec-gen/sections training-dataset/sections")
        print("  ln -sf /mnt/md0/zhushan/spec-gen/specs    training-dataset/specs")
        print("  ln -sf /mnt/md0/zhushan/spec-gen/dataset  training-dataset/dataset")
        sys.exit(1)

    limit = args.limit or len(dataset)
    print(f"[data] {len(dataset)} samples loaded, running first {limit}\n")

    # Load model
    model = QwenLocalModel(str(MODEL_PATH))

    # Optional RAG retriever
    retriever = None
    rag_top_k = max(0, args.rag_top_k)
    if rag_top_k > 0:
        if not args.rag_index:
            print("[warn] --rag-top-k > 0 but --rag-index not set; RAG disabled")
            rag_top_k = 0
        else:
            try:
                sys.path.insert(0, str(SPEC_GEN))
                from rag.retriever import RuleRetriever
                retriever = RuleRetriever(args.rag_index)
                print(f"[rag] enabled: top_k={rag_top_k}, index={args.rag_index}")
            except Exception as e:
                print(f"[warn] RAG init failed: {e}; RAG disabled")
                rag_top_k = 0

    # Run V3 prompt evaluation
    result = evaluate_prompt_variant(
        V3_PROMPT,
        variant_key="v3_qwen" if rag_top_k == 0 else f"v3_qwen_rag{rag_top_k}",
        dataset=dataset,
        model=model,
        limit=limit,
        n_samples=1,
        save_results=True,
        resume=args.resume,
        retriever=retriever,
        rag_top_k=rag_top_k,
        results_root=RESULTS_ROOT,
    )

    # Report
    metrics = aggregate_metrics(result["problem_results"], ks=(1,))
    rag_label = f" + RAG top{args.rag_top_k}" if args.rag_top_k > 0 else ""
    print(f"\n{'='*55}")
    print(f"  Qwen (item_split_e2_best) + V3 Prompt{rag_label}")
    print(f"  Commands evaluated : {limit}")
    print(f"  CodeBLEU Best@1    : {metrics['best@1']:.4f}")
    print(f"  Results saved to   : {RESULTS_ROOT}/")
    print(f"{'='*55}\n")


if __name__ == "__main__":
    main()
