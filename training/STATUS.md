# Fine-tuning Status — spec-gen (Qwen3-4B + Unsloth)

**Last updated:** 2026-03-28
**GPU server:** cuda_device=5 (Quadro RTX 8000, 48 GB)
**Base model:** `unsloth/Qwen3-4B` (4-bit QLoRA)

---

## Training Progress

| Layer | Task | Status | Model path | Notes |
|-------|------|--------|------------|-------|
| L2 | Type definitions | ✅ Done | `models/layer2_best/` | 179 train / 62 val examples |
| — | L2 inference → cascaded context | ✅ Done | `generated_types/` | 4 versions generated |
| — | substitute_context.py | ✅ Done | `dataset/train_cascaded.jsonl` | 938 total / 279 cmd examples |
| L3 | Helper stubs | ✅ Done | `models/layer3_best/` | 480 train / 159 val examples |
| CMD | Command specs (cascaded) | ✅ Done | `models/commands_best/` | train_loss=0.094, eval_loss=0.968 |

**All training complete.** Next step: end-to-end evaluation on alp14.

---

## Training Configuration

```
Model:      unsloth/Qwen3-4B (load_in_4bit=True)
LoRA:       r=16, alpha=32, dropout=0.05
Targets:    q/k/v/o/gate/up/down_proj
Epochs:     10
Batch:      4 (grad_acc=4, effective=16) for L2/L3
            2 (grad_acc=4, effective=8)  for CMD (longer sequences)
LR:         2e-4, cosine scheduler, 3% warmup
Precision:  fp16 (GPU is Turing, no bf16 support)
Attn:       xformers 0.0.35 (required for CMD — standard attn OOMs at seq>2k)
max_seq:    4096 for L2/L3 | 6144 for CMD (covers 89% of examples without truncation)
Grad ckpt:  gradient_checkpointing=True + use_reentrant=False (CMD only)
```

---

## Key Lessons Learned

1. **xformers is required for CMD training.** Without it, standard O(seq²) attention OOMs even with batch=1 on CMD examples (median 4114 tokens). VRAM dropped from 36 GB → 12 GB after installing xformers.

2. **`use_gradient_checkpointing="unsloth"` causes OOM on this GPU.** Unsloth's custom gradient checkpointing unexpectedly inflated memory usage on the Quadro RTX 8000 (Turing, CUDA 7.5, no FlashAttention). Standard PyTorch `gradient_checkpointing=True` works correctly.

3. **CMD needs separate settings from L2/L3.** L2/L3 examples are short (100–2000 tokens), CMD examples are long (median 4114, max 9128). Different batch size and gradient checkpointing needed.

4. **Inference_l2.py system prompt must match build_dataset.py exactly.** Originally used a shorter generic prompt; fixed to `SYSTEM_PROMPT_TYPES` from `build_dataset.py` before running inference.

---

## Next Steps

### Step 5 — Implement pipeline.py inference and evaluate on alp14

```bash
# Merge CMD adapter for standalone inference
python3 -c "
from unsloth import FastLanguageModel
model, tokenizer = FastLanguageModel.from_pretrained('models/commands_best')
model.save_pretrained_merged('models/commands_merged', tokenizer)
"

# End-to-end inference on held-out test set
CUDA_VISIBLE_DEVICES=5 python3 pipeline.py \
    --txt      ccaspec/alp14.txt \
    --target   alp14 \
    --l2-model models/layer2_best \
    --l3-model models/layer3_best \
    --cmd-model models/commands_merged \
    --out      alp14_generated.rs

# Compare to gold
diff alp14_gold.rs alp14_generated.rs | head -80
```

**Note:** `pipeline.py` still has `NotImplementedError` stubs in `load_model()` and `run_model()`. These need to be implemented using Unsloth inference:

```python
from unsloth import FastLanguageModel
import torch

def load_model(model_path):
    model, tokenizer = FastLanguageModel.from_pretrained(
        model_path, max_seq_length=8192, load_in_4bit=True)
    FastLanguageModel.for_inference(model)
    return model, tokenizer

def run_model(model_tokenizer, messages):
    model, tokenizer = model_tokenizer
    inputs = tokenizer.apply_chat_template(
        messages, return_tensors="pt", add_generation_prompt=True
    ).to("cuda")
    out = model.generate(inputs, max_new_tokens=2048, temperature=0.1, do_sample=True)
    return tokenizer.decode(out[0][inputs.shape[1]:], skip_special_tokens=True)
```

### Success criteria
- **Primary:** `alp14_generated.rs` compiles under Verus without errors
- **Secondary:** BLEU/CodeBLEU vs `alp14_gold.rs`, predicate recall (fraction of `&&`-clauses recovered)

---

## File Map

```
spec-gen/
├── train.py                  # Unsloth+Qwen3 SFTTrainer (L2/L3/CMD)
├── inference_l2.py           # Run L2 model → generated_types/ (cascaded context)
├── pipeline.py               # End-to-end inference (load_model/run_model stubs remain)
├── build_dataset.py          # Builds JSONL datasets from PDF sections + gold specs
├── extract_sections.py       # Extracts PDF text sections per command/type/helper
├── split_specs.py            # Splits gold .rs into per-command files
├── substitute_context.py     # Replaces golden preamble with L2-generated context
├── test_e2e_oracle.py        # Oracle assembly test (validates pipeline without GPU)
├── run_pipeline.sh           # Chains all training steps (L2→inference→L3→CMD)
├── run_cascade_and_cmd.sh    # Wait-for-L2/L3, then run cascade+CMD
├── README_training.md        # Full training guide
├── CONTEXT.md                # Project overview and architecture
└── boilerplate/layer1.rs     # Hardcoded L1 type aliases (never trained)
```
