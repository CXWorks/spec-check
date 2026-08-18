#!/usr/bin/env python3
"""Generate one Verus spec file per command, in the layout the rule-check scorer reads.

    python scripts/gen_specs.py --base Qwen/Qwen3-4B \
        --adapter jisenli/spec-check-ckpt --subfolder sft3-0/final \
        --versions eac5 rel0 --out-dir predictions/sft3-0

`eval_checkpoint.py` answers "does it compile, and does it agree with gold" on the
held-out alp14 commands. This answers a different question and needs a different
output shape: SCOPE's eight dangling-output findings are **eac5 and rel0** items,
and `benchmark/rule_check_8bugs/score.py` reads one `<version>/<command>.rs` per
command rather than a JSON. Nothing existing produced that — the script the
original Qwen row was generated with (`run_qwen_baseline1_eac5_rel0.py`) is
referenced in BASELINE1_SCOPE_REPRODUCTION.md but is in neither branch, so the
published row cannot be re-derived. This is the replacement.

**Why this is only meaningful for sft3-*.** The check asks whether the generated
body leaves a declared output unconstrained. Gold leaves it unconstrained too --
that IS the spec gap -- so a model trained on gold for that command reproduces the
omission whether or not it read the document. Five of the eight commands are in
every sft2-* training set at all six versions, including eac5 and rel0 themselves,
which makes the eac5/rel0 training example and the test the identical
(input, target) pair. `dataset_bench` holds all eight out, so sft3-* is the first
checkpoint for which a flag here is evidence of anything.

No Verus is needed: the check is textual. Compilation is a separate axis, measured
by eval_checkpoint.py.
"""

import argparse
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))
sys.path.insert(0, str(ROOT / "scripts"))


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True)
    ap.add_argument("--adapter", default=None, help="HF repo id, or a local path")
    ap.add_argument("--subfolder", default=None, help="e.g. sft3-0/final")
    ap.add_argument("--versions", nargs="+", default=["eac5", "rel0"])
    ap.add_argument("--out-dir", required=True,
                    help="Written as <out-dir>/<version>/<command_lower>.rs")
    ap.add_argument("--prompt-variant", default=None, choices=["v3", "v3.1"],
                    help="Must match the checkpoint's training prompt. Default v3, "
                         "or $SPEC_CHECK_PROMPT_VARIANT. sft3-* is v3.1.")
    ap.add_argument("--max-new-tokens", type=int, default=6144)
    ap.add_argument("--with-preamble", action="store_true",
                    help="Restore the 200-line preamble tail that training embedded "
                         "in every prompt. Matching training is the correct inference "
                         "condition and worth +22.5pp compile rate on the 9B "
                         "(McNemar p=0.004) -- but the published Qwen baseline-1 row "
                         "was generated WITHOUT it, so pass nothing to reproduce that "
                         "and pass this to measure the model properly. Report which.")
    ap.add_argument("--limit", type=int, default=None)
    args = ap.parse_args()

    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer
    from dataset_loader import load_dataset
    from prompt_engineering_v3 import get_v3_prompt
    from eval_checkpoint import (build_prompt, load_train_preamble,
                                 render_generation_prompt, strip_output)

    prompt = get_v3_prompt(args.prompt_variant)
    print(f"[gen] prompt variant: {prompt.name}", flush=True)

    tok = AutoTokenizer.from_pretrained(args.base)
    is_adapter = False
    if args.adapter:
        from huggingface_hub import file_exists
        cfg = f"{args.subfolder}/adapter_config.json" if args.subfolder else "adapter_config.json"
        try:
            is_adapter = file_exists(args.adapter, cfg, token=os.environ.get("HF_TOKEN"))
        except Exception:
            is_adapter = Path(args.adapter, cfg).exists()
    kw = {"subfolder": args.subfolder} if args.subfolder else {}
    if args.adapter and not is_adapter:
        model = AutoModelForCausalLM.from_pretrained(
            args.adapter, dtype=torch.bfloat16, device_map="auto", **kw)
    else:
        model = AutoModelForCausalLM.from_pretrained(
            args.base, dtype=torch.bfloat16, device_map="auto")
        if args.adapter:
            from peft import PeftModel
            model = PeftModel.from_pretrained(model, args.adapter, **kw)
    model.eval()

    out_root = Path(args.out_dir)
    for version in args.versions:
        # all_commands=True on purpose: this is not the held-out-set evaluation.
        # The benchmark scores whichever of a version's commands SCOPE parsed, and
        # restricting to the alp14 test split would silently drop most of them.
        # It is safe here only because dataset_bench holds every scored command out
        # of training -- see the module docstring.
        samples = load_dataset(versions=[version], all_commands=True)
        samples = [s for s in samples if getattr(s, "command", None)]
        if args.limit:
            samples = samples[: args.limit]
        if not samples:
            sys.exit(f"no commands loaded for {version} — is "
                     f"training-dataset/sections/{version}/ present?")

        preamble = load_train_preamble(ROOT / "training-dataset" / "specs" / version) \
            if args.with_preamble else None
        vdir = out_root / version
        vdir.mkdir(parents=True, exist_ok=True)
        print(f"[gen] {version}: {len(samples)} commands -> {vdir}"
              f"{' (with preamble)' if preamble else ''}", flush=True)

        for i, s in enumerate(samples, 1):
            msgs = build_prompt(s, prompt, preamble)
            text = render_generation_prompt(tok, msgs)
            raw = tok(text, return_tensors="pt", add_special_tokens=False)
            ids = (raw["input_ids"] if hasattr(raw, "keys") else raw).to(model.device)
            with torch.no_grad():
                g = model.generate(ids, max_new_tokens=args.max_new_tokens,
                                   do_sample=False, pad_token_id=tok.eos_token_id)
            spec = strip_output(tok.decode(g[0][ids.shape[1]:], skip_special_tokens=True))
            # Written even when empty. A missing file is scored as "not checked",
            # which would quietly shrink the denominator; an empty one is a
            # generation failure and should be visible as a miss.
            (vdir / f"{s.command.lower()}.rs").write_text(spec)
            print(f"[gen] {i}/{len(samples)} {s.command}: {len(spec)} chars", flush=True)

    print(f"\n[gen] done. Score with:\n"
          f"  python3 benchmark/rule_check_8bugs/score.py --predictions <parent-of> "
          f"--model {out_root.name}", flush=True)


if __name__ == "__main__":
    main()
