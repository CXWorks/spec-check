#!/usr/bin/env python3
"""
eval_item_split.py

Run inference on dataset/test.jsonl with a trained model, then compute CodeBLEU
against the gold assistant turns.

Usage:
    python3 eval_item_split.py --model models/item_split_e2_best \
                               --test  dataset/test.jsonl \
                               --out   results/item_split_e2.jsonl
"""

import os
os.environ["CUDA_VISIBLE_DEVICES"] = "5"

import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path

from unsloth import FastLanguageModel
import torch
from codebleu import calc_codebleu

BASE      = Path(__file__).parent
VERUSFMT  = BASE / "verusfmt/target/release/verusfmt"

# ---------------------------------------------------------------------------
parser = argparse.ArgumentParser()
parser.add_argument("--model", default="models/item_split_e2_best")
parser.add_argument("--test",  default="dataset/test.jsonl")
parser.add_argument("--out",   default="results/item_split_e2.jsonl")
parser.add_argument("--max-new-tokens", type=int, default=1024)
args = parser.parse_args()

os.makedirs(Path(args.out).parent, exist_ok=True)

# ---------------------------------------------------------------------------
# Load model
# ---------------------------------------------------------------------------
print(f"Loading model from {args.model} ...")
model, tokenizer = FastLanguageModel.from_pretrained(
    args.model,
    max_seq_length=4096,
    load_in_4bit=True,
    dtype=None,
)
FastLanguageModel.for_inference(model)
print("Model loaded.\n")

# ---------------------------------------------------------------------------
# Load test examples
# ---------------------------------------------------------------------------
test_examples = [json.loads(l) for l in open(args.test) if l.strip()]
print(f"Test examples: {len(test_examples)}")

# ---------------------------------------------------------------------------
# Inference
# ---------------------------------------------------------------------------
def generate(messages: list[dict]) -> str:
    # Strip the last assistant turn (gold) to get prompt-only messages
    prompt_msgs = [m for m in messages if m["role"] != "assistant"]
    ids = tokenizer.apply_chat_template(
        prompt_msgs,
        tokenize=True,
        add_generation_prompt=True,
        return_tensors="pt",
    )
    if hasattr(ids, "input_ids"):
        ids = ids.input_ids
    ids = ids.to(model.device)
    input_len = ids.shape[-1]
    with torch.no_grad():
        out = model.generate(
            ids,
            max_new_tokens=args.max_new_tokens,
            do_sample=False,
            pad_token_id=tokenizer.eos_token_id,
        )
    return tokenizer.decode(out[0][input_len:], skip_special_tokens=True).strip()


results = []
for i, ex in enumerate(test_examples):
    meta  = ex["metadata"]
    gold  = next(m["content"] for m in ex["messages"] if m["role"] == "assistant")
    pred  = generate(ex["messages"])
    results.append({"metadata": meta, "gold": gold, "pred": pred})
    label = meta.get("command") or meta.get("type") or meta.get("function") or "?"
    print(f"  [{i+1}/{len(test_examples)}] {meta.get('kind','?')} / {label}: {pred[:60]}...")

with open(args.out, "w") as f:
    for r in results:
        f.write(json.dumps(r, ensure_ascii=False) + "\n")
print(f"\nPredictions saved → {args.out}")

# ---------------------------------------------------------------------------
# CodeBLEU
# ---------------------------------------------------------------------------
def try_format(code: str) -> str:
    if not VERUSFMT.exists():
        return code
    with tempfile.NamedTemporaryFile(suffix=".rs", mode="w", delete=False) as f:
        f.write(f"use vstd::prelude::*;\nverus! {{\n{code}\n}}\n")
        fname = f.name
    try:
        r = subprocess.run([str(VERUSFMT), fname], capture_output=True, timeout=10)
        if r.returncode == 0:
            txt = Path(fname).read_text()
            inner = re.search(r'verus!\s*\{(.*)\}\s*(?://[^\n]*)?\s*$', txt, re.DOTALL)
            if inner:
                return inner.group(1).strip()
    except Exception:
        pass
    finally:
        os.unlink(fname)
    return code


def codebleu_for_kind(kind_filter):
    subset = [r for r in results if r["metadata"].get("kind") == kind_filter]
    if not subset:
        return None, 0
    refs = [[try_format(r["gold"])] for r in subset]
    hyps = [try_format(r["pred"])   for r in subset]
    score = calc_codebleu(refs, hyps, lang="rust", weights=(0.25, 0.25, 0.25, 0.25))
    return score, len(subset)


print("\n" + "="*58)
print("  CodeBLEU Results — item-split test set")
print("="*58)

all_refs, all_hyps = [], []
for kind in ("command", "type_definition", "helper_stub"):
    score, n = codebleu_for_kind(kind)
    if score is None:
        continue
    print(f"\n  Kind: {kind}  (n={n})")
    print(f"    CodeBLEU       : {score['codebleu']:.4f}")
    print(f"    ngram          : {score['ngram_match_score']:.4f}")
    print(f"    weighted_ngram : {score['weighted_ngram_match_score']:.4f}")
    print(f"    syntax         : {score['syntax_match_score']:.4f}")
    print(f"    dataflow       : {score['dataflow_match_score']:.4f}")
    # accumulate for overall
    subset = [r for r in results if r["metadata"].get("kind") == kind]
    all_refs.extend([[try_format(r["gold"])] for r in subset])
    all_hyps.extend([try_format(r["pred"])   for r in subset])

if all_refs:
    overall = calc_codebleu(all_refs, all_hyps, lang="rust", weights=(0.25, 0.25, 0.25, 0.25))
    print(f"\n  OVERALL (n={len(all_refs)})")
    print(f"    CodeBLEU       : {overall['codebleu']:.4f}")
    print(f"    ngram          : {overall['ngram_match_score']:.4f}")
    print(f"    weighted_ngram : {overall['weighted_ngram_match_score']:.4f}")
    print(f"    syntax         : {overall['syntax_match_score']:.4f}")
    print(f"    dataflow       : {overall['dataflow_match_score']:.4f}")

print("="*58)

# Per-command breakdown (commands only)
cmd_results = [r for r in results if r["metadata"].get("kind") == "command"]
if cmd_results:
    per_cmd = []
    for r in cmd_results:
        s = calc_codebleu(
            [[try_format(r["gold"])]],
            [try_format(r["pred"])],
            lang="rust",
        )
        per_cmd.append((r["metadata"].get("command", "?"), s["codebleu"]))
    per_cmd.sort(key=lambda x: x[1])
    print("\n  Worst commands:")
    for n, s in per_cmd[:5]:
        print(f"    {s:.3f}  {n}")
    print("\n  Best commands:")
    for n, s in per_cmd[-5:][::-1]:
        print(f"    {s:.3f}  {n}")

print("\nDone.")
