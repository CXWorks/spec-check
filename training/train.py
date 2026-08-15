#!/usr/bin/env python3
"""Fine-tune a causal LM on the spec dataset. peft + trl, bf16, multi-GPU.

Ported off Unsloth/4-bit/fp16/xformers, which existed to fit a 48 GB Turing card
and are liabilities on H100. See docs/gpu-and-runs.md.

One behavioural fix worth calling out: loss is computed on the assistant turn
only. The previous version trained on the whole formatted conversation, and the
answer is a median 4.9% of a command example (~276 answer tokens against ~5325
of prompt) — so ~91% of the gradient went into predicting the PDF text back.

Launch (single node, 8 GPU):
    torchrun --nproc_per_node=8 training/train.py \
        --train  training-dataset/dataset_clean/train.jsonl \
        --val    training-dataset/dataset_clean/val.jsonl \
        --out    /work/out/sft2-0 \
        --run-id sft2-0 \
        --model  Qwen/Qwen3-4B --precision bf16 --method lora --push
"""

import argparse
import json
import os
import sys
from pathlib import Path

import torch
from datasets import Dataset
from transformers import AutoModelForCausalLM, AutoTokenizer


def log(msg):
    """Rank-0-only stdout. These logs are read via `kubectl logs`."""
    if int(os.environ.get("RANK", "0")) == 0:
        print(f"[train] {msg}", flush=True)


def load_jsonl(path):
    with open(path) as f:
        return [json.loads(l) for l in f if l.strip()]


def parse_args():
    p = argparse.ArgumentParser()
    p.add_argument("--train", required=True)
    p.add_argument("--val", required=True)
    p.add_argument("--out", required=True)
    p.add_argument("--run-id", required=True,
                   help="Shared key across k8s / W&B / HF. e.g. sft2-0")
    p.add_argument("--model", default="Qwen/Qwen3-4B")
    p.add_argument("--precision", choices=["bf16", "fp16"], default="bf16")
    p.add_argument("--method", choices=["lora", "full"], default="lora")
    p.add_argument("--lora-r", type=int, default=16)
    p.add_argument("--lora-alpha", type=int, default=32)
    p.add_argument("--epochs", type=int, default=3,
                   help="3 keeps every epoch's checkpoint so the epoch curve is "
                        "free at eval time; the old '2 is optimal' was tuned on a "
                        "different dataset and precision.")
    p.add_argument("--batch-size", type=int, default=1)
    p.add_argument("--grad-accum", type=int, default=4)
    p.add_argument("--lr", type=float, default=2e-4)
    p.add_argument("--max-seq", type=int, default=12288)
    p.add_argument("--max-steps", type=int, default=-1,
                   help="Cap total steps. Used for smoke tests (gate G2); -1 = off.")
    p.add_argument("--full-sequence-loss", action="store_true",
                   help="Train on prompt tokens too (the old behaviour). Off by "
                        "default; see the module docstring.")
    p.add_argument("--push", action="store_true",
                   help="Upload each epoch checkpoint to $HF_CKPT_REPO/<run-id>/")
    p.add_argument("--no-wandb", action="store_true")
    return p.parse_args()


def build_model(args):
    dtype = torch.bfloat16 if args.precision == "bf16" else torch.float16

    kwargs = dict(dtype=dtype)
    # flash-attn is a hard requirement for 12k sequences at this batch size, but
    # a missing wheel should degrade rather than abort — sdpa is correct, slower.
    try:
        import flash_attn  # noqa: F401
        kwargs["attn_implementation"] = "flash_attention_2"
        log("attention: flash_attention_2")
    except ImportError:
        kwargs["attn_implementation"] = "sdpa"
        log("attention: sdpa (flash_attn not installed — slower, not wrong)")

    model = AutoModelForCausalLM.from_pretrained(args.model, **kwargs)
    model.config.use_cache = False

    if args.method == "lora":
        from peft import LoraConfig, get_peft_model
        model = get_peft_model(model, LoraConfig(
            r=args.lora_r,
            lora_alpha=args.lora_alpha,
            lora_dropout=0.05,
            bias="none",
            task_type="CAUSAL_LM",
            target_modules=["q_proj", "k_proj", "v_proj", "o_proj",
                            "gate_proj", "up_proj", "down_proj"],
        ))
        if int(os.environ.get("RANK", "0")) == 0:
            model.print_trainable_parameters()
    else:
        log(f"full fine-tune: {sum(p.numel() for p in model.parameters())/1e9:.2f}B params")

    return model


def build_config(args, trl):
    """SFTConfig, adapting to whichever trl is in the image.

    `max_length` vs `max_seq_length` has bitten this project before: the wrong
    name is accepted silently and the cap never applies, which showed up only as
    unexplained OOMs. Probe the signature instead of guessing.
    """
    import inspect
    sig = inspect.signature(trl.SFTConfig.__init__).parameters

    cfg = dict(
        output_dir=args.out,
        num_train_epochs=args.epochs,
        max_steps=args.max_steps,
        per_device_train_batch_size=args.batch_size,
        gradient_accumulation_steps=args.grad_accum,
        learning_rate=args.lr,
        lr_scheduler_type="cosine",
        warmup_ratio=0.03,
        gradient_checkpointing=True,
        gradient_checkpointing_kwargs={"use_reentrant": False},
        logging_steps=5,
        eval_strategy="epoch",
        save_strategy="epoch",
        save_total_limit=None,          # keep every epoch — that is the point
        load_best_model_at_end=False,   # val is type/helper-only, biased for commands
        report_to=("none" if args.no_wandb else "wandb"),
        run_name=args.run_id,
        bf16=(args.precision == "bf16"),
        fp16=(args.precision == "fp16"),
        seed=42,
    )

    seq_key = "max_length" if "max_length" in sig else "max_seq_length"
    cfg[seq_key] = args.max_seq
    log(f"sequence cap via SFTConfig({seq_key}={args.max_seq})")

    if not args.full_sequence_loss:
        if "assistant_only_loss" in sig:
            cfg["assistant_only_loss"] = True
            log("loss: assistant turn only (assistant_only_loss)")
        else:
            log("loss: assistant turn only (manual mask — trl lacks the flag)")
    else:
        log("loss: FULL SEQUENCE (prompt included) — old behaviour")

    for k in list(cfg):
        if k not in sig and k not in ("output_dir",):
            log(f"  dropping SFTConfig kwarg not in this trl: {k}")
            cfg.pop(k)

    return trl.SFTConfig(**cfg)


def mask_prompt_tokens(ds, tokenizer, max_seq):
    """Fallback for trl versions without assistant_only_loss.

    Tokenise the prompt alone and the full conversation, then set the label of
    every prompt token to -100 so only the assistant turn contributes.
    """
    def encode(ex):
        msgs = ex["messages"]
        prompt = tokenizer.apply_chat_template(
            msgs[:-1], tokenize=False, add_generation_prompt=True)
        full = tokenizer.apply_chat_template(
            msgs, tokenize=False, add_generation_prompt=False)
        p_ids = tokenizer(prompt, add_special_tokens=False)["input_ids"]
        f_ids = tokenizer(full, add_special_tokens=False)["input_ids"][:max_seq]
        labels = list(f_ids)
        for i in range(min(len(p_ids), len(labels))):
            labels[i] = -100
        return {"input_ids": f_ids, "labels": labels,
                "attention_mask": [1] * len(f_ids)}

    return ds.map(encode, remove_columns=ds.column_names)


def push_to_hub(args):
    repo = os.environ.get("HF_CKPT_REPO")
    if not repo:
        log("HF_CKPT_REPO unset — skipping upload; artifacts are on the PVC only")
        return
    from huggingface_hub import HfApi
    api = HfApi(token=os.environ.get("HF_TOKEN"))
    out = Path(args.out)
    ckpts = sorted(p for p in out.glob("checkpoint-*") if p.is_dir())
    targets = ckpts + [out] if ckpts else [out]
    for src in targets:
        dest = f"{args.run_id}/{src.name if src is not out else 'final'}"
        try:
            api.upload_folder(folder_path=str(src), path_in_repo=dest,
                              repo_id=repo, repo_type="model",
                              ignore_patterns=["optimizer.pt", "*.safetensors.index.json.tmp"])
            log(f"uploaded {src.name} -> {repo}/{dest}")
        except Exception as e:
            # An upload failure must not look like a training failure; the rescue
            # pod can still get these off the PVC.
            log(f"UPLOAD FAILED for {src.name}: {e}")


def main():
    args = parse_args()
    import trl

    log(f"run={args.run_id} model={args.model} precision={args.precision} "
        f"method={args.method} epochs={args.epochs}")
    log(f"torch={torch.__version__} trl={trl.__version__} "
        f"cuda={torch.cuda.is_available()} gpus={torch.cuda.device_count()}")
    if torch.cuda.is_available():
        log(f"bf16 supported: {torch.cuda.is_bf16_supported()}")
        if args.precision == "bf16" and not torch.cuda.is_bf16_supported():
            sys.exit("bf16 requested but unsupported on this GPU")

    tokenizer = AutoTokenizer.from_pretrained(args.model)
    if tokenizer.pad_token is None:
        tokenizer.pad_token = tokenizer.eos_token

    train_ds = Dataset.from_list(load_jsonl(args.train))
    val_ds = Dataset.from_list(load_jsonl(args.val))
    log(f"data: {len(train_ds)} train / {len(val_ds)} val")

    cfg = build_config(args, trl)
    model = build_model(args)

    manual_mask = (not args.full_sequence_loss
                   and not getattr(cfg, "assistant_only_loss", False))
    if manual_mask:
        train_ds = mask_prompt_tokens(train_ds, tokenizer, args.max_seq)
        val_ds = mask_prompt_tokens(val_ds, tokenizer, args.max_seq)
        log("applied manual prompt masking")

    trainer = trl.SFTTrainer(
        model=model,
        args=cfg,
        train_dataset=train_ds,
        eval_dataset=val_ds,
        processing_class=tokenizer,
    )

    trainer.train()
    trainer.save_model(args.out)
    log(f"saved to {args.out}")

    if int(os.environ.get("RANK", "0")) == 0:
        Path(args.out, ".done").write_text(args.run_id + "\n")
        if args.push:
            push_to_hub(args)


if __name__ == "__main__":
    main()
