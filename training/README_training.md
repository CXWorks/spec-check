# Fine-tuning LLM to Generate Verus Specs from RMM PDF

This directory contains everything needed to fine-tune an LLM to replace the
hand-written SCOPE pipeline for generating Verus constraint functions from the
Arm CCA RMM specification.

---

## Architecture Overview

The pipeline is split into **three trained layers** plus a hardcoded boilerplate layer:

| Layer | Input | Output | Training data |
|-------|-------|--------|---------------|
| L1 | — | type aliases, constants, `struct S` | hard-copied from `boilerplate/layer1.rs` |
| L2 | PDF type section (Part C) | `pub enum` / `struct` definitions | `train.jsonl` (kind=`type_definition`) |
| L3 | PDF B3.x helper fn section | `pub open spec fn ...;` stub | `train_helpers.jsonl` |
| CMD | PDF command section + L1–L3 context | `pub open spec fn {cmd}_spec(...)` | `train_cascaded.jsonl` |

**Training order matters**: L2 must be trained first, its output used to build
`train_cascaded.jsonl`, then CMD is trained on that cascaded data. This avoids
covariate shift — the command model trains on the same noisy generated context
it will see at inference time.

---

## Dataset

### Files

```
dataset/train.jsonl          938 examples  (eac5, rel0, alp11, alp12)
dataset/val.jsonl            320 examples  (alp13)
dataset/test.jsonl           326 examples  (alp14)
dataset/train_helpers.jsonl  480 examples  (L3 helper stubs, train versions only)
dataset/train_cascaded.jsonl (generated on GPU server — see Step 2 below)
```

### Per-version breakdown

| Split | Version | Cmd | Type | Helper | Total |
|-------|---------|-----|------|--------|-------|
| train | eac5    | 41  | 27   | 83     | 151   |
| train | rel0    | 41  | 29   | 83     | 153   |
| train | alp11   | 96  | 52   | 166    | 314   |
| train | alp12   | 101 | 55   | 164    | 320   |
| val   | alp13   | 99  | 56   | 165    | 320   |
| test  | alp14   | 98  | 57   | 171    | 326   |
| **total** |    | **476** | **276** | **832** | **1584** |

### Example format

Each line is a JSON object in the standard chat format:

```json
{
  "messages": [
    {"role": "system",    "content": "<system prompt>"},
    {"role": "user",      "content": "<input>"},
    {"role": "assistant", "content": "<verus output>"}
  ],
  "metadata": {
    "version": "eac5",
    "kind":    "type_definition | helper_stub | (absent = command)"
  }
}
```

The `metadata` field is informational only — strip or ignore during training.

### Token estimates (1 token ≈ 4 chars)

| Kind    | Typical range | Max  |
|---------|---------------|------|
| command | 2k – 8k       | ~10k |
| type    | 300 – 2k      | ~3k  |
| helper  | 100 – 600     | ~1k  |

All examples fit within an 8192-token context window.

---

## Recommended Base Model

**Qwen2.5-Coder-7B-Instruct** (or 14B if GPU memory allows).
Rationale: strong Rust/formal-verification awareness from pretraining,
good instruction following, fits on a single A100 80 GB with QLoRA.

Alternatives:
- `deepseek-coder-v2-lite-instruct` (16B MoE, 2×A100 with QLoRA)
- `meta-llama/Llama-3.1-8B-Instruct`

---

## Hyperparameters

| Hyperparameter         | L2 / L3       | CMD           |
|------------------------|---------------|---------------|
| Max sequence length    | 4096          | 8192          |
| LoRA rank (r)          | 16            | 16            |
| LoRA alpha             | 32            | 32            |
| LoRA target modules    | q/k/v/o/gate/up/down_proj | same |
| Dropout                | 0.05          | 0.05          |
| Batch size             | 4 (grad acc 4) | 4 (grad acc 4) |
| Learning rate          | 2e-4          | 2e-4          |
| LR scheduler           | cosine 3% warmup | cosine 3% warmup |
| Epochs                 | 5–10          | 5–10          |
| Optimizer              | adamw_torch_fused | adamw_torch_fused |
| BF16                   | true          | true          |
| Eval strategy          | every epoch   | every epoch   |

Monitor val loss and use early stopping (patience 3 epochs).

---

## Training Sequence

### Step 0 — Install dependencies

```bash
pip install trl peft transformers datasets accelerate bitsandbytes
```

### Step 1 — Train L2 model (type definitions)

Filter type examples and train:

```bash
python3 - <<'EOF'
import json
with open('dataset/train.jsonl') as f, \
     open('dataset/train_types.jsonl', 'w') as out:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') == 'type_definition':
            out.write(line)

with open('dataset/val.jsonl') as f, \
     open('dataset/val_types.jsonl', 'w') as out:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') == 'type_definition':
            out.write(line)
EOF

python3 train.py \
    --train dataset/train_types.jsonl \
    --val   dataset/val_types.jsonl \
    --out   models/layer2/
```

### Step 2 — Generate cascaded type context

Run the trained L2 model on all 4 train-split PDF type sections to produce
predicted type text, then substitute that into the command training examples:

```python
# inference_l2.py (adapt to your inference stack)
# For each version in [eac5, rel0, alp11, alp12]:
#   - load sections/{version}/types/*.txt
#   - run L2 model on each
#   - concatenate outputs → generated_types/{version}_types.rs

python3 substitute_context.py \
    --input   dataset/train.jsonl \
    --gen-dir generated_types/ \
    --output  dataset/train_cascaded.jsonl
```

`substitute_context.py` replaces the golden preamble context in each command
example with the model-generated type text. Non-command examples are passed
through unchanged.

### Step 3a — Train L3 model (helper stubs)

```bash
python3 - <<'EOF'
import json
with open('dataset/val.jsonl') as f, \
     open('dataset/val_helpers.jsonl', 'w') as out:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') == 'helper_stub':
            out.write(line)
EOF

python3 train.py \
    --train dataset/train_helpers.jsonl \
    --val   dataset/val_helpers.jsonl \
    --out   models/layer3/
```

### Step 3b — Train command model (cascaded)

```bash
python3 - <<'EOF'
import json
with open('dataset/train_cascaded.jsonl') as f, \
     open('dataset/train_cmds_cascaded.jsonl', 'w') as out:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') not in ('type_definition', 'helper_stub'):
            out.write(line)

with open('dataset/val.jsonl') as f, \
     open('dataset/val_cmds.jsonl', 'w') as out:
    for line in f:
        ex = json.loads(line)
        if ex['metadata'].get('kind') not in ('type_definition', 'helper_stub'):
            out.write(line)
EOF

python3 train.py \
    --train dataset/train_cmds_cascaded.jsonl \
    --val   dataset/val_cmds.jsonl \
    --out   models/commands/ \
    --max-seq 8192
```

---

## Training Script Template

```python
#!/usr/bin/env python3
"""train.py — fine-tune with SFTTrainer + LoRA"""

import argparse
import json
from datasets import Dataset
from transformers import AutoTokenizer, AutoModelForCausalLM, BitsAndBytesConfig
from peft import LoraConfig, get_peft_model
from trl import SFTTrainer, SFTConfig

MODEL_ID = "Qwen/Qwen2.5-Coder-7B-Instruct"

def load_jsonl(path):
    with open(path) as f:
        return [json.loads(l) for l in f if l.strip()]

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--train",   required=True)
    parser.add_argument("--val",     required=True)
    parser.add_argument("--out",     required=True)
    parser.add_argument("--max-seq", type=int, default=4096)
    args = parser.parse_args()

    tokenizer = AutoTokenizer.from_pretrained(MODEL_ID)
    tokenizer.pad_token = tokenizer.eos_token

    def fmt(ex):
        return {"text": tokenizer.apply_chat_template(
            ex["messages"], tokenize=False, add_generation_prompt=False)}

    train_ds = Dataset.from_list(load_jsonl(args.train)).map(fmt)
    val_ds   = Dataset.from_list(load_jsonl(args.val)).map(fmt)

    bnb_config = BitsAndBytesConfig(
        load_in_4bit=True,
        bnb_4bit_quant_type="nf4",
        bnb_4bit_compute_dtype="bfloat16",
    )
    model = AutoModelForCausalLM.from_pretrained(
        MODEL_ID, quantization_config=bnb_config, device_map="auto"
    )
    model = get_peft_model(model, LoraConfig(
        r=16, lora_alpha=32, lora_dropout=0.05,
        target_modules=["q_proj","v_proj","k_proj","o_proj",
                        "gate_proj","up_proj","down_proj"],
        bias="none", task_type="CAUSAL_LM",
    ))
    model.print_trainable_parameters()

    trainer = SFTTrainer(
        model=model,
        args=SFTConfig(
            output_dir=args.out,
            max_seq_length=args.max_seq,
            num_train_epochs=10,
            per_device_train_batch_size=4,
            gradient_accumulation_steps=4,
            learning_rate=2e-4,
            lr_scheduler_type="cosine",
            warmup_ratio=0.03,
            bf16=True,
            evaluation_strategy="epoch",
            save_strategy="epoch",
            load_best_model_at_end=True,
            metric_for_best_model="eval_loss",
            logging_steps=10,
            report_to="tensorboard",
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
```

---

## Evaluation

### Merge adapter for standalone inference

```bash
python3 -c "
from peft import PeftModel
from transformers import AutoModelForCausalLM, AutoTokenizer
model = AutoModelForCausalLM.from_pretrained('Qwen/Qwen2.5-Coder-7B-Instruct')
model = PeftModel.from_pretrained(model, 'models/commands/_best')
model = model.merge_and_unload()
model.save_pretrained('models/commands_merged')
AutoTokenizer.from_pretrained('Qwen/Qwen2.5-Coder-7B-Instruct').save_pretrained('models/commands_merged')
"
```

### End-to-end test on alp14

```bash
python3 pipeline.py \
    --txt      ccaspec/alp14.txt \
    --target   alp14 \
    --l1-rs    boilerplate/layer1.rs \
    --l2-model models/layer2_best \
    --l3-model models/layer3_best \
    --cmd-model models/commands_merged \
    --out      alp14_generated.rs

# Optional: verify with Verus
python3 pipeline.py ... --verify
```

Compare against gold:
```bash
diff alp14_gold.rs alp14_generated.rs | head -80
```

### Metrics

Primary: **exact-match Verus compilation** — does the output compile under Verus?

Secondary:
- **BLEU / CodeBLEU** against gold `_spec` bodies
- **Predicate recall**: fraction of `&&`-clauses in gold recovered in prediction

---

## Reproducing the Dataset from Scratch

```bash
# 1. Convert PDFs → txt (requires poppler-utils)
for v in eac5 rel0 alp11 alp12 alp13 alp14; do
    pdftotext -layout ccaspec/DEN0137_*-${v}_*.pdf ccaspec/${v}.txt
done

# 2. Generate gold Verus output via SCOPE
cd scope
for v in eac5 rel0 alp11 alp12 alp13 alp14; do
    python3 scope --target $v --mode reason > ../${v}_raw.rs
done
cd ..

# 3. Apply patches (eac5 and rel0 have manual corrections)
patch eac5_raw.rs scope/patch/eac5.patch -o eac5_gold.rs
patch rel0_raw.rs scope/patch/rel0.patch -o rel0_gold.rs
for v in alp11 alp12 alp13 alp14; do cp ${v}_raw.rs ${v}_gold.rs; done

# 4. Extract sections and split specs
python3 extract_sections.py
python3 split_specs.py

# 5. Build dataset
python3 build_dataset.py

# 6. Verify oracle reconstruction (all should PASS)
for v in eac5 rel0 alp11 alp12 alp13 alp14; do
    python3 test_e2e_oracle.py --version $v
done
```
