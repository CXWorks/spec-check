#!/usr/bin/env python3
"""Score a checkpoint on the 40 held-out commands: generate, compile, guard.

    # greedy, comparable to every earlier run
    python scripts/eval_checkpoint.py \
        --base Qwen/Qwen3-4B --adapter jisenli/spec-check-ckpt --subfolder sft2-0/final \
        --out /work/eval/sft2-0-final.json

    # best-of-k: greedy plus 8 sampled, to measure the model's reachable ceiling
    python scripts/eval_checkpoint.py ... --samples 8 --temperature 0.8

Four numbers, in increasing order of what they actually tell you:

  codebleu   similarity to gold. Weak — the project has already shown its rank
             order disagrees with Verus pass rate across models.
  pass_rate  does it compile, greedy. The ceiling is NOT 100%: gold itself
             compiles on only 33/40, so that is what a run is measured against.
  non_degen  is the output non-trivial. `spec fn f(..) -> bool { true }` compiles
             perfectly and is worthless, so a pass rate that rises while this
             falls is the model gaming the metric, not improving.
  pass_at_k  does ANY of k samples compile. This separates two very different
             situations that a greedy pass rate cannot: a model that knows the
             answer but does not rank it first, versus one that does not know it.
             Only the first is worth chasing with rejection-sampling fine-tuning,
             and pass@k measures the headroom that would be available to it.

pass@k is a measurement, not a training signal. Compiling is NOT evidence of
faithfulness to the specification text — a spec can compile and say the wrong
thing — so nothing here may be promoted to a training target before the
faithfulness work in docs/gpu-and-runs.md Phase 2 exists.
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


SENTINEL = "<<<SPEC_ANSWER_GOES_HERE>>>"


def render_generation_prompt(tok, msgs):
    """Render the prompt by cutting the training-shaped render at the answer.

    Rather than trusting `add_generation_prompt=True` to reproduce what training
    fed the model, render the full conversation with a sentinel assistant message
    and cut there. That is a prefix of the training sequence by construction, for
    any chat template.

    Verified: BOTH Qwen3-4B and Qwen3.5-9B render an empty think block into the
    training conversation, but `add_generation_prompt=True` cuts them in
    different places, so the two models are trained to emit different things.

      training render (both)  ...assistant\\n<think>\\n\\n</think>\\n\\n<spec><|im_end|>
      4B generation prompt    ...assistant\\n
      9B generation prompt    ...assistant\\n<think>\\n

    train.py masks by prompt length, so the 4B is supervised on the whole
    `<think>\\n\\n</think>\\n\\n<spec>` and learns to emit the empty block itself,
    while the 9B is supervised only from `\\n</think>` onward. Both are internally
    consistent, which is why the 4B evals were fine — but the arrangement is
    accidental, and it made the two models' eval paths differ in a way nothing in
    the code stated. Cutting the training render at the answer removes the
    difference: generation starts at the spec for every model and template.

    Verified by inspecting sft2-2's outputs: this is what cost that run 16 of 40
    commands. Handed an OPEN `<think>`, the 9B reasoned in prose — "Let me
    analyze this RMI_DATA_CREATE_UNKNOWN command specification: 1. **Inputs**…" —
    and hit the token cap before writing any spec. All 16 failures are truncated
    at 4.8k–9.7k characters, i.e. exactly `--max-new-tokens`. Fine-tuning taught
    it to close the block immediately, but on longer sections the base model's
    reasoning prior wins. Its 25.0% pass rate is therefore not a capability
    measurement: of the 24 commands that produced a parseable spec, 10 compiled
    (41.7%), which is in line with the 4B.

    Closing the block in the prompt removes the choice, so the model has nowhere
    to put reasoning even when it would otherwise start.
    """
    train_render = tok.apply_chat_template(
        msgs + [{"role": "assistant", "content": SENTINEL}], tokenize=False)
    if SENTINEL not in train_render:
        # No sentinel means the template dropped or transformed the content and
        # this derivation is not valid. Guessing here is how the 9B run was lost.
        sys.exit("cannot derive training prefix: chat template did not emit the "
                 "assistant content verbatim")
    return train_render.split(SENTINEL)[0]


def report_prompt_alignment(tok, prompt):
    """State whether plain add_generation_prompt would have matched. Diagnostic
    only — the returned prompt is always the derived one."""
    for kw in ({}, {"enable_thinking": False}):
        try:
            got = tok.apply_chat_template(
                [{"role": "system", "content": "s"}, {"role": "user", "content": "u"}],
                tokenize=False, add_generation_prompt=True, **kw)
        except Exception:
            continue
        ref = render_generation_prompt(
            tok, [{"role": "system", "content": "s"}, {"role": "user", "content": "u"}])
        tag = "add_generation_prompt" + ("(enable_thinking=False)" if kw else "")
        print(f"[eval] prompt: {tag} {'==' if got == ref else '!='} training prefix",
              flush=True)
    print(f"[eval] using derived training prefix, tail={prompt[-60:]!r}", flush=True)


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
    ap.add_argument("--max-new-tokens", type=int, default=6144,
                    help="Was 2048, which silently truncated the long commands: "
                         "the longest gold spec is 12837 chars (~4400 tokens), so "
                         "2048 could not fit it even in principle. 7/40, 8/40 and "
                         "14/40 generations were cut off mid-expression in "
                         "sft2-0/1/3, and half the commands that never passed under "
                         "any run were simply never allowed to finish.")
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument("--verus-timeout", type=int, default=600)
    ap.add_argument("--samples", type=int, default=0,
                    help="Sampled generations per command, IN ADDITION to the "
                         "greedy one. 0 keeps the run identical to earlier runs.")
    ap.add_argument("--temperature", type=float, default=0.8)
    ap.add_argument("--top-p", type=float, default=0.95)
    ap.add_argument("--jobs", type=int, default=8,
                    help="Parallel Verus processes. Verus is a subprocess, so "
                         "threads are enough; this is what makes k>1 affordable.")
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

    report_prompt_alignment(tok, render_generation_prompt(tok, build_prompt(dataset[0], V3_PROMPT)))

    def generate(s):
        """Greedy first, then args.samples sampled continuations of the same prompt."""
        text = render_generation_prompt(tok, build_prompt(s, V3_PROMPT))
        # add_special_tokens=False: the template already emitted every special
        # token as text, so letting the tokenizer add more would shift the prompt
        # away from the training prefix this function just went to the trouble of
        # reproducing.
        raw = tok(text, return_tensors="pt", add_special_tokens=False)
        # transformers 5.x returns a BatchEncoding here where 4.x returns a
        # tensor, and generate() then fails on .shape. The project hit the same
        # shape of bug via Unsloth (STATUS.md lesson 6); handle both.
        ids = raw["input_ids"] if hasattr(raw, "keys") else raw
        ids = ids.to(model.device)
        outs = []
        with torch.no_grad():
            g = model.generate(ids, max_new_tokens=args.max_new_tokens,
                               do_sample=False, pad_token_id=tok.eos_token_id)
            outs.append(g[0][ids.shape[1]:])
            if args.samples:
                # One call with num_return_sequences rather than a loop: the
                # prompt is encoded once and the samples share the batch.
                g = model.generate(ids, max_new_tokens=args.max_new_tokens,
                                   do_sample=True, temperature=args.temperature,
                                   top_p=args.top_p,
                                   num_return_sequences=args.samples,
                                   pad_token_id=tok.eos_token_id)
                outs.extend(g[:, ids.shape[1]:])
        # Keep the decoder output before strip_output touches it, and keep the
        # special tokens: when extraction fails, the thing you need to see is
        # exactly what the model emitted and where it stopped. sft2-2 returned
        # `no_pub_open_spec_fn_found` on most commands and left nothing behind to
        # explain it, which cost a whole eval cycle.
        raw = [tok.decode(o, skip_special_tokens=False) for o in outs]
        # Two signals, because either alone misses cases: hitting the cap is the
        # direct one, and an unbalanced brace catches a stop that landed
        # mid-expression for any other reason. A truncated spec fails with
        # "mismatched closing delimiter", which reads as a syntax error the model
        # made rather than an answer it was never allowed to finish.
        texts = [strip_output(tok.decode(o, skip_special_tokens=True)) for o in outs]
        n_trunc = sum(1 for o, t in zip(outs, texts)
                      if len(o) >= args.max_new_tokens or t.count("{") > t.count("}"))
        return texts, raw, n_trunc

    def check_all(command, texts):
        """Verus-check each distinct text once, in parallel.

        Deduplicated because a model this thoroughly fitted repeats itself even
        at temperature 0.8, and each check is a whole Verus process.
        """
        uniq = list(dict.fromkeys(texts))
        with ThreadPoolExecutor(max_workers=args.jobs) as pool:
            checked = dict(zip(uniq, pool.map(
                lambda t: check_text(verus, preamble, command, t, args.verus_timeout),
                uniq)))
        return [checked[t] for t in texts], len(uniq)

    results = []
    for i, s in enumerate(dataset, 1):
        gens, raws, n_trunc = generate(s)
        chks, n_uniq = check_all(s.command, gens)

        cb = None
        if calc_codebleu:
            try:
                cb = calc_codebleu([s.oracle], [gens[0]], lang="rust")["codebleu"]
            except Exception:
                pass

        samples = [{
            "mode": "greedy" if j == 0 else "sampled",
            "pass": c.status == "pass",
            "reason": c.reason,
            # The compiler's own words. Without these no failure can be diagnosed
            # after the fact, and the reason bucket alone has already proved too
            # coarse to trust — see classify_failure in verify_generated_verus.py.
            "output_head": c.output_head,
            "fn_name": c.fn_name,
            "generated": g,
            # Only when extraction failed: that is when the raw text is needed,
            # and storing it for all k samples would bloat the file.
            "generated_raw": r if c.reason == "no_pub_open_spec_fn_found" else None,
        } for j, (g, c, r) in enumerate(zip(gens, chks, raws))]

        sampled = samples[1:]
        results.append({
            "command": s.command,
            "pass": samples[0]["pass"],           # greedy: comparable to earlier runs
            "reason": samples[0]["reason"],
            "codebleu": cb,
            "degeneracy": degeneracy_flags(gens[0], s),
            "generated": gens[0],
            "any_pass": any(x["pass"] for x in samples),
            "n_sampled": len(sampled),
            "n_sampled_pass": sum(x["pass"] for x in sampled),
            "n_distinct": n_uniq,
            # Hitting the token cap is a decode-budget failure, not a modelling
            # one, and the two are indistinguishable from the reason bucket alone.
            "n_truncated": n_trunc,
            "samples": samples if args.samples else None,
        })
        extra = "  TRUNCATED" if n_trunc else ""
        if args.samples:
            extra += (f"  best-of-{len(samples)}="
                     f"{'PASS' if results[-1]['any_pass'] else 'fail'}"
                     f" ({results[-1]['n_sampled_pass']}/{len(sampled)} sampled,"
                     f" {n_uniq} distinct)")
        print(f"[eval] {i}/{len(dataset)} {s.command}: "
              f"{'PASS' if samples[0]['pass'] else 'FAIL(' + str(samples[0]['reason']) + ')'}"
              f"{extra}", flush=True)

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
        "truncated": sum(r["n_truncated"] for r in results),
        "no_fn_found": sum(1 for r in results
                           if r["reason"] == "no_pub_open_spec_fn_found"),
        "gold_ceiling_note": "gold compiles on 33/40 (82.5%) with this Verus build",
    }
    if args.samples:
        k = args.samples
        tot_s = sum(r["n_sampled"] for r in results)
        hit_s = sum(r["n_sampled_pass"] for r in results)
        anyp = sum(r["any_pass"] for r in results)
        summary.update({
            "sampling": {"samples": k, "temperature": args.temperature,
                         "top_p": args.top_p},
            # Per-sample rate at this temperature. Compare against pass_rate to
            # see what sampling costs relative to greedy decoding.
            "pass_at_1_sampled": round(100 * hit_s / tot_s, 2) if tot_s else None,
            "pass_at_k": anyp, "pass_at_k_rate": round(100 * anyp / n, 2),
            "mean_distinct": round(sum(r["n_distinct"] for r in results) / n, 2),
            "headroom_note": (
                "pass_at_k - pass_rate is the gap a reranker or rejection-sampling "
                "pass could close WITHOUT new training. It is an upper bound and "
                "says nothing about faithfulness to the spec text."),
        })
    Path(args.out).parent.mkdir(parents=True, exist_ok=True)
    Path(args.out).write_text(json.dumps({"summary": summary, "results": results}, indent=2))

    print("\n" + json.dumps(summary, indent=2))


if __name__ == "__main__":
    main()
