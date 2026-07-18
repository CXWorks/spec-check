#!/usr/bin/env python3
"""train.py — fine-tune Qwen3-4B with Unsloth + LoRA via SFTTrainer"""

import os
os.environ["CUDA_VISIBLE_DEVICES"] = "5"

# unsloth must be imported before trl/transformers/peft
from unsloth import FastLanguageModel

import argparse
import json

from datasets import Dataset
from trl import SFTTrainer, SFTConfig

MODEL_ID = "unsloth/Qwen3-4B"


def load_jsonl(path):
    with open(path) as f:
        return [json.loads(l) for l in f if l.strip()]


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--train",   required=True)
    parser.add_argument("--val",     required=True)
    parser.add_argument("--out",     required=True)
    parser.add_argument("--max-seq",   type=int, default=4096)
    parser.add_argument("--batch-size", type=int, default=4)
    parser.add_argument("--epochs",    type=int, default=10)
    args = parser.parse_args()

    model, tokenizer = FastLanguageModel.from_pretrained(
        MODEL_ID,
        max_seq_length=args.max_seq,
        load_in_4bit=True,
        dtype=None,  # auto-detect bfloat16
    )
    model = FastLanguageModel.get_peft_model(
        model,
        r=16,
        lora_alpha=32,
        lora_dropout=0.05,
        target_modules=["q_proj", "k_proj", "v_proj", "o_proj",
                        "gate_proj", "up_proj", "down_proj"],
        bias="none",
    )
    model.print_trainable_parameters()
    tokenizer.pad_token = tokenizer.eos_token

    def fmt(ex):
        return {"text": tokenizer.apply_chat_template(
            ex["messages"], tokenize=False, add_generation_prompt=False)}

    train_ds = Dataset.from_list(load_jsonl(args.train)).map(fmt)
    val_ds   = Dataset.from_list(load_jsonl(args.val)).map(fmt)

    trainer = SFTTrainer(
        model=model,
        args=SFTConfig(
            output_dir=args.out,
            max_length=args.max_seq,
            packing=False,
            padding_free=False,  # this GPU has no flash-attn/xformers; padding-free's flattened-batch
                                 # path silently falls back to a much more memory-hungry attention kernel
            num_train_epochs=args.epochs,
            per_device_train_batch_size=args.batch_size,
            gradient_accumulation_steps=4,
            learning_rate=2e-4,
            lr_scheduler_type="cosine",
            warmup_ratio=0.03,
            fp16=True,
            gradient_checkpointing=True,
            gradient_checkpointing_kwargs={"use_reentrant": False},
            eval_strategy="epoch",
            save_strategy="epoch",
            load_best_model_at_end=True,
            metric_for_best_model="eval_loss",
            logging_steps=10,
            report_to="none",
        ),
        train_dataset=train_ds,
        eval_dataset=val_ds,
        tokenizer=tokenizer,
        dataset_text_field="text",
    )
    trainer.train()
    trainer.save_model(args.out + "_best")


if __name__ == "__main__":
    main()
