#!/usr/bin/env python3
"""inference_l2.py — run the trained L2 model on all train-split type sections
to produce generated_types/{version}_types.rs for cascaded training (Step 2)."""

import os
os.environ["CUDA_VISIBLE_DEVICES"] = "5"

import argparse
import torch
from unsloth import FastLanguageModel

parser = argparse.ArgumentParser()
parser.add_argument("--model", default="models/layer2_best")
_args = parser.parse_args()
MODEL_PATH = _args.model
MAX_NEW_TOKENS = 512
VERSIONS = ["eac5", "rel0", "alp11", "alp12"]

SYSTEM = (
    "You are a formal specification assistant for Arm CCA (Confidential Compute "
    "Architecture) Realm Management Monitor (RMM). "
    "Given the specification text for an RMM type definition (enumeration, structure, "
    "or fieldset), generate the corresponding Verus/Rust type definition. "
    "Output only the type definition (pub enum or struct block) in valid Verus syntax."
)

model, tokenizer = FastLanguageModel.from_pretrained(
    MODEL_PATH,
    max_seq_length=4096,
    load_in_4bit=True,
    dtype=None,
)
FastLanguageModel.for_inference(model)

os.makedirs("generated_types", exist_ok=True)

for version in VERSIONS:
    types_dir = f"sections/{version}/types"
    all_defs = []
    fnames = sorted(os.listdir(types_dir))
    for fname in fnames:
        section_text = open(f"{types_dir}/{fname}").read().strip()
        messages = [
            {"role": "system", "content": SYSTEM},
            {"role": "user",   "content": f"## Type Specification (from RMM spec PDF)\n\n{section_text}"},
        ]
        raw = tokenizer.apply_chat_template(
            messages,
            tokenize=True,
            add_generation_prompt=True,
            return_tensors="pt",
        )
        if hasattr(raw, "input_ids"):
            input_ids = raw.input_ids.to(model.device)
        else:
            input_ids = raw.to(model.device)
        input_len = input_ids.shape[-1]

        with torch.no_grad():
            out = model.generate(
                input_ids,
                max_new_tokens=MAX_NEW_TOKENS,
                do_sample=False,
                pad_token_id=tokenizer.eos_token_id,
            )
        response = tokenizer.decode(
            out[0][input_len:], skip_special_tokens=True
        ).strip()
        all_defs.append(response)
        print(f"  [{version}] {fname[:-4]}: {response[:60]}...")

    out_path = f"generated_types/{version}_types.rs"
    with open(out_path, "w") as f:
        f.write("\n\n".join(all_defs) + "\n")
    print(f"[L2] {version}: {len(all_defs)} types → {out_path}")

print("Done. Now run:")
print("  python3 substitute_context.py --input dataset/train.jsonl "
      "--gen-dir generated_types/ --output dataset/train_cascaded.jsonl")
