#!/usr/bin/env python3
"""Score a checkpoint on the 40 held-out commands: generate, compile, guard.

    python scripts/eval_checkpoint.py \
        --base Qwen/Qwen3-4B --adapter jisenli/spec-check-ckpt --subfolder sft2-0/final \
        --out /work/eval/sft2-0-final.json

Three numbers, in increasing order of what they actually tell you:

  codebleu   similarity to gold. Weak — the project has already shown its rank
             order disagrees with Verus pass rate across models.
  pass_rate  does it compile. The ceiling is NOT 100%: gold itself compiles on
             only 33/40, so that is what a run is really being measured against.
  non_degen  is the output non-trivial. `spec fn f(..) -> bool { true }` compiles
             perfectly and is worthless, so a pass rate that rises while this
             falls is the model gaming the metric, not improving.
"""

import argparse
import json
import os
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))


def build_prompt(sample, v3):
    """Same shape the model was trained on: V3 system prompt, no preamble.

    build_dataset.py embeds the preamble at training time only — the model is
    expected to have learned the symbol names — so supplying it here would be a
    train/inference mismatch, the exact bug Iteration 6 was about.
    """
    user = (
        f"{sample.section_text}\n\n"
        f"Signature: pub open spec fn {sample.command.lower()}_spec(...) -> bool\n"
        "Prefer Bits64/UInt64/UInt32 aliases when present in context/spec, but do "
        "not sacrifice semantic correctness for alias formatting.\n"
        "Keep unchanged-state constraints when implied by the command behavior."
    )
    return [{"role": "system", "content": v3.system}, {"role": "user", "content": user}]


def strip_output(text):
    text = re.sub(r"<think>.*?</think>\s*", "", text, flags=re.DOTALL)
    m = re.search(r"```(?:rust|verus)?\s*(.*?)```", text, re.DOTALL)
    return (m.group(1) if m else text).strip()


def degeneracy_flags(src, sample):
    """Cheap structural checks. Not a faithfulness metric — see docs/gpu-and-runs.md
    Phase 2 — but enough to catch a spec that compiles by saying nothing."""
    body = src[src.find("{") + 1: src.rfind("}")] if "{" in src else ""
    stripped = body.strip()
    return {
        # `{ true }` and friends: compiles, constrains nothing.
        "trivial_body": stripped in ("true", "true,", "") or len(stripped) < 20,
        # No implication at all means no failure/success structure was extracted.
        "no_implication": "==>" not in body,
        # The signature should mention the command's declared outputs.
        "has_result_param": "result" in src.split("{")[0],
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True)
    ap.add_argument("--adapter", default=None, help="HF repo id, or a local path")
    ap.add_argument("--subfolder", default=None, help="e.g. sft2-0/final")
    ap.add_argument("--out", required=True)
    ap.add_argument("--specs-dir", default=str(ROOT / "training-dataset/specs/alp14"))
    ap.add_argument("--max-new-tokens", type=int, default=2048)
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument("--verus-timeout", type=int, default=600)
    args = ap.parse_args()

    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer
    from dataset_loader import load_dataset
    from prompt_engineering_v3 import V3_PROMPT
    from verify_generated_verus import check_text, find_verus_bin, read_preamble

    verus = find_verus_bin(None)
    if not verus:
        sys.exit("verus not found — set VERUS_BIN")

    dataset = load_dataset(split="test")          # defaults to the held-out 40
    if args.limit:
        dataset = dataset[: args.limit]
    if not dataset:
        # An empty eval set reports 0/0 = a clean-looking result. Refuse instead:
        # the usual cause is missing section files, which are the model's input.
        sys.exit("no commands loaded - check training-dataset/sections/<version>/")
    print(f"[eval] {len(dataset)} commands", flush=True)

    tok = AutoTokenizer.from_pretrained(args.base)

    # A full fine-tune's checkpoint IS the model — no adapter_config.json — so
    # loading it through PeftModel 404s. Probe for the adapter config and pick
    # the right loader rather than assuming every run produced an adapter.
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
        print(f"[eval] full model {args.adapter} {args.subfolder or ''}", flush=True)
    else:
        model = AutoModelForCausalLM.from_pretrained(
            args.base, dtype=torch.bfloat16, device_map="auto")
        if args.adapter:
            from peft import PeftModel
            kw = {"subfolder": args.subfolder} if args.subfolder else {}
            model = PeftModel.from_pretrained(model, args.adapter, **kw)
            print(f"[eval] adapter {args.adapter} {args.subfolder or ''}", flush=True)
    model.eval()

    preamble = read_preamble(Path(args.specs_dir) / "preamble.rs")
    try:
        from codebleu import calc_codebleu
    except ImportError:
        calc_codebleu = None

    results = []
    for i, s in enumerate(dataset, 1):
        raw = tok.apply_chat_template(build_prompt(s, V3_PROMPT), return_tensors="pt",
                                      add_generation_prompt=True)
        # transformers 5.x returns a BatchEncoding here where 4.x returns a
        # tensor, and generate() then fails on .shape. The project hit the same
        # shape of bug via Unsloth (STATUS.md lesson 6); handle both.
        ids = raw["input_ids"] if hasattr(raw, "keys") else raw
        ids = ids.to(model.device)
        with torch.no_grad():
            out = model.generate(ids, max_new_tokens=args.max_new_tokens,
                                 do_sample=False, pad_token_id=tok.eos_token_id)
        gen = strip_output(tok.decode(out[0][ids.shape[1]:], skip_special_tokens=True))

        chk = check_text(verus, preamble, s.command, gen, args.verus_timeout)
        cb = None
        if calc_codebleu:
            try:
                cb = calc_codebleu([s.oracle], [gen], lang="rust")["codebleu"]
            except Exception:
                pass

        results.append({
            "command": s.command,
            "pass": chk.status == "pass",
            "reason": chk.reason,
            "codebleu": cb,
            "degeneracy": degeneracy_flags(gen, s),
            "generated": gen,
        })
        print(f"[eval] {i}/{len(dataset)} {s.command}: "
              f"{'PASS' if chk.status == 'pass' else 'FAIL(' + str(chk.reason) + ')'}",
              flush=True)

    n = len(results)
    npass = sum(r["pass"] for r in results)
    cbs = [r["codebleu"] for r in results if r["codebleu"] is not None]
    ndeg = sum(1 for r in results
               if not r["degeneracy"]["trivial_body"]
               and not r["degeneracy"]["no_implication"])
    summary = {
        "base": args.base, "adapter": args.adapter, "subfolder": args.subfolder,
        "n": n,
        "pass": npass, "pass_rate": round(100 * npass / n, 2),
        "codebleu": round(sum(cbs) / len(cbs), 4) if cbs else None,
        "non_degenerate": ndeg, "non_degenerate_rate": round(100 * ndeg / n, 2),
        "gold_ceiling_note": "gold compiles on 33/40 (82.5%) with this Verus build",
    }
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps({"summary": summary, "results": results}, indent=2))

    print("\n" + json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
