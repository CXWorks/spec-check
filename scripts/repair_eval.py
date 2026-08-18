#!/usr/bin/env python3
"""Feed Verus errors back to the model and let it repair its own spec.

    python scripts/repair_eval.py --base Qwen/Qwen3.5-9B \
        --adapter jisenli/spec-check-ckpt --subfolder sft2-2/final \
        --rounds 2 --out /work/eval/sft2-2-repair.json

Why this and not more training. Three independent lines say next-token training
is spent: loss reaches 0.002, three epochs score identically, and neither
capacity nor method separates on pass@1. Yet pass@9 is 42.5% for the 4B and
60.0% for the 9B against a gold ceiling of 82.5% — the answers are in the
distribution. Repair is an inference-time way to reach them that needs no
training, and therefore does not touch the rule that compiling is not evidence
of faithfulness: nothing here is promoted to a training target.

**A repair that works by deleting constraints is not a repair.** `spec fn f() ->
bool { true }` compiles perfectly and says nothing, and a loop rewarded for
compiling will find that. So every round records the conjunct count alongside the
verdict, and the summary reports pass rate and constraint retention together. A
rise in one with a fall in the other is the loop gaming the metric.

The fine-tune only ever saw single-turn conversations, so a repair turn is off
its training distribution. That is the experiment, not a flaw in it.
"""

import argparse
import json
import os
import re
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))
sys.path.insert(0, str(ROOT / "scripts"))


def n_clauses(src):
    """Conjuncts in the body — the crude size measure a weakening repair moves."""
    body = src[src.find("{") + 1: src.rfind("}")] if "{" in src else ""
    return len([c for c in body.split("&&") if c.strip()])


def build_repair_turn(prev_spec, chk):
    """The failing spec plus what Verus said about it.

    Only `output_head` is passed back, not the whole compiler run: the preamble
    alone emits hundreds of `uninterp` warnings before any real error, and
    burying the error in them is how the model would learn nothing from it.
    """
    err = (chk.output_head or f"failed: {chk.reason}").strip()
    return [
        {"role": "assistant", "content": prev_spec},
        {"role": "user", "content":
            f"That specification does not compile. Verus reports:\n\n```\n{err}\n```\n\n"
            "Return a corrected `pub open spec fn`. Fix the error without removing "
            "conditions that the specification text requires — a spec that compiles "
            "by constraining less is worse than one that does not compile."},
    ]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True)
    ap.add_argument("--adapter", default=None)
    ap.add_argument("--subfolder", default=None)
    ap.add_argument("--out", required=True)
    ap.add_argument("--prompt-variant", default=None, choices=["v3", "v3.1"],
                    help="Must match the checkpoint's training prompt. "
                         "Default v3, or $SPEC_CHECK_PROMPT_VARIANT.")
    ap.add_argument("--specs-dir", default=str(ROOT / "training-dataset/specs/alp14"))
    ap.add_argument("--max-new-tokens", type=int, default=6144)
    ap.add_argument("--rounds", type=int, default=2, help="repair attempts per command")
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument("--verus-timeout", type=int, default=600)
    ap.add_argument("--jobs", type=int, default=8)
    args = ap.parse_args()

    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer
    from dataset_loader import load_dataset
    from prompt_engineering_v3 import get_v3_prompt
    from verify_generated_verus import check_text, find_verus_bin, read_preamble

    # Same rule as eval_checkpoint.py: the prompt has to be the one the checkpoint
    # was trained on. Defaults to v3.
    V3_PROMPT = get_v3_prompt(args.prompt_variant)
    print(f"[repair] prompt variant: {V3_PROMPT.name}", flush=True)
    from eval_checkpoint import (build_prompt, render_generation_prompt,
                                 strip_output, degeneracy_flags)

    verus = find_verus_bin(None)
    if not verus:
        sys.exit("verus not found — set VERUS_BIN")
    dataset = load_dataset(split="test")
    if args.limit:
        dataset = dataset[: args.limit]
    if not dataset:
        sys.exit("no commands loaded - check training-dataset/sections/<version>/")
    print(f"[repair] {len(dataset)} commands, {args.rounds} repair rounds", flush=True)

    tok = AutoTokenizer.from_pretrained(args.base)
    is_adapter = False
    if args.adapter:
        from huggingface_hub import file_exists
        cfg = f"{args.subfolder}/adapter_config.json" if args.subfolder else "adapter_config.json"
        try:
            is_adapter = file_exists(args.adapter, cfg, token=os.environ.get("HF_TOKEN"))
        except Exception:
            is_adapter = Path(args.adapter, cfg).exists()
    if args.adapter and not is_adapter:
        kw = {"subfolder": args.subfolder} if args.subfolder else {}
        model = AutoModelForCausalLM.from_pretrained(
            args.adapter, dtype=torch.bfloat16, device_map="auto", **kw)
    else:
        model = AutoModelForCausalLM.from_pretrained(
            args.base, dtype=torch.bfloat16, device_map="auto")
        if args.adapter:
            from peft import PeftModel
            kw = {"subfolder": args.subfolder} if args.subfolder else {}
            model = PeftModel.from_pretrained(model, args.adapter, **kw)
    model.eval()
    preamble = read_preamble(Path(args.specs_dir) / "preamble.rs")

    def generate(msgs):
        text = render_generation_prompt(tok, msgs)
        raw = tok(text, return_tensors="pt", add_special_tokens=False)
        ids = (raw["input_ids"] if hasattr(raw, "keys") else raw).to(model.device)
        with torch.no_grad():
            out = model.generate(ids, max_new_tokens=args.max_new_tokens,
                                 do_sample=False, pad_token_id=tok.eos_token_id)
        return strip_output(tok.decode(out[0][ids.shape[1]:], skip_special_tokens=True))

    results = []
    for i, s in enumerate(dataset, 1):
        msgs = build_prompt(s, V3_PROMPT)
        rounds, spec = [], None
        for r in range(args.rounds + 1):          # round 0 is the initial attempt
            spec = generate(msgs)
            chk = check_text(verus, preamble, s.command, spec, args.verus_timeout)
            rounds.append({
                "round": r, "pass": chk.status == "pass", "reason": chk.reason,
                "n_clauses": n_clauses(spec),
                "degeneracy": degeneracy_flags(spec, s),
                "output_head": chk.output_head, "generated": spec,
            })
            if chk.status == "pass":
                break
            if r < args.rounds:
                msgs = msgs + build_repair_turn(spec, chk)

        first, last = rounds[0], rounds[-1]
        results.append({
            "command": s.command,
            "pass_round0": first["pass"], "pass_final": last["pass"],
            "rounds_used": len(rounds) - 1,
            "clauses_round0": first["n_clauses"], "clauses_final": last["n_clauses"],
            "rounds": rounds,
        })
        print(f"[repair] {i}/{len(dataset)} {s.command}: "
              f"r0={'PASS' if first['pass'] else first['reason']} -> "
              f"final={'PASS' if last['pass'] else last['reason']} "
              f"(clauses {first['n_clauses']}->{last['n_clauses']})", flush=True)

    n = len(results)
    p0 = sum(r["pass_round0"] for r in results)
    pf = sum(r["pass_final"] for r in results)
    fixed = [r for r in results if r["pass_final"] and not r["pass_round0"]]
    # Constraint retention among the repairs that succeeded. If these come in
    # well under 1.0, the loop is buying compilation by saying less.
    ratios = [r["clauses_final"] / max(r["clauses_round0"], 1) for r in fixed]
    summary = {
        "base": args.base, "subfolder": args.subfolder, "rounds": args.rounds, "n": n,
        "prompt_variant": V3_PROMPT.name,
        "pass_round0": p0, "pass_rate_round0": round(100 * p0 / n, 2),
        "pass_final": pf, "pass_rate_final": round(100 * pf / n, 2),
        "repaired": len(fixed),
        "repaired_commands": [r["command"] for r in fixed],
        "clause_ratio_of_repaired": round(sum(ratios) / len(ratios), 3) if ratios else None,
        "shrank_while_repairing": sum(1 for x in ratios if x < 0.9),
        "non_degenerate_final": sum(
            1 for r in results
            if not r["rounds"][-1]["degeneracy"]["trivial_body"]
            and not r["rounds"][-1]["degeneracy"]["no_implication"]
            and not r["rounds"][-1]["degeneracy"]["repetitive"]),
        "gold_ceiling_note": "gold compiles on 33/40 (82.5%) with this Verus build",
        "caveat": "A rise in pass rate with clause_ratio_of_repaired well below 1.0 "
                  "means the loop weakened specs rather than fixing them.",
    }
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps({"summary": summary, "results": results}, indent=2))
    print("\n" + json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
