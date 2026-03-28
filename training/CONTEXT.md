# GPU Training Context — RMM→Verus Pipeline

This document is for a new LLM agent session on the GPU server.
It summarizes everything that has been built, what the current state is,
and exactly what to do next.

---

## What This Project Is

We are replacing a 2400-line hand-written Python pipeline (called SCOPE) that
converts Arm CCA RMM specification PDFs into Verus formal specification files
(`.rs`). The goal is to train an LLM to do this instead.

The RMM spec is a PDF describing the Realm Management Monitor — a trusted
firmware component for Arm Confidential Compute Architecture. The Verus output
is a formal `.rs` file with `pub open spec fn` functions encoding each command's
preconditions and postconditions.

---

## Directory Layout

```
.
├── dataset/
│   ├── train.jsonl          # 938 examples (eac5, rel0, alp11, alp12)
│   ├── val.jsonl            # 320 examples (alp13)
│   ├── test.jsonl           # 326 examples (alp14)
│   └── train_helpers.jsonl  # 480 L3 examples (train versions only)
│
├── specs/{version}/         # Split gold Verus files
│   ├── preamble.rs          # Everything before command functions
│   ├── types/               # Per-type enum/struct definitions
│   ├── helpers/             # Per-helper pub open spec fn stubs
│   ├── {cmd}_spec.rs        # Per-command spec functions
│   ├── {cmd}_rule.rs        # Per-command proof functions (some commands)
│   └── epilogue.rs          # fn main() {} (needed for Verus compilation)
│
├── sections/{version}/      # Per-section raw PDF text
│   ├── {CMD}_command.txt    # Command section text
│   ├── types/{Type}.txt     # Type section text
│   └── helpers/{Fn}.txt     # B3.x helper function section text
│
├── boilerplate/
│   └── layer1.rs            # Hard-coded type aliases + constants (75 lines)
│
├── {version}_gold.rs        # Complete gold Verus files (ground truth)
│
├── README_training.md       # Full GPU training instructions (read this!)
├── train.py                 # (TO CREATE) training script — template in README
├── extract_sections.py      # Extracts sections from spec PDF text
├── split_specs.py           # Splits gold .rs into per-command files
├── build_dataset.py         # Builds JSONL dataset
├── pipeline.py              # End-to-end pipeline (runs after training)
├── test_e2e_oracle.py       # Oracle assembly test (no GPU needed)
└── substitute_context.py    # Cascaded context substitution (Step 2)
```

Versions: `eac5` (1.0-eac5), `rel0` (1.0-rel0), `alp11`, `alp12`, `alp13`, `alp14` (all 1.1-alpha).

---

## Three-Layer Training Architecture

```
PDF → [L2 model] → type defs (pub enum / struct)
PDF → [L3 model] → helper stubs (pub open spec fn ...;)
PDF + L2+L3 context → [CMD model] → _spec functions
```

**Layer 1** (`boilerplate/layer1.rs`): hardcoded, never trained.

**Layer 2** (type definitions): trains on `kind=type_definition` examples.
Input: raw PDF type section text. Output: Verus enum/struct.

**Layer 3** (helper stubs): trains on `train_helpers.jsonl`.
Input: B3.x helper function spec text (ASL pseudocode + description).
Output: `pub open spec fn Name(s: S, ...) -> RetType;` (single line, ends with `;`).

**Command model**: trains on `train_cascaded.jsonl` (built in Step 2).
Input: PDF command section + preamble context. Output: `pub open spec fn {cmd}_spec(...)`.

**Why cascaded?** At inference time, the command model sees *generated* (imperfect)
L2 output as context. Training on golden L2 context would cause covariate shift.
Training on model-generated L2 context matches the inference distribution.

---

## Dataset Example Formats

### Type definition example (kind=type_definition)
```json
{
  "messages": [
    {"role": "system",    "content": "You are a formal specification assistant ... generate the corresponding Verus/Rust type definition."},
    {"role": "user",      "content": "## Type Specification (from RMM spec PDF)\n\nC5.4 RmiHashAlgorithm type\nenumeration RmiHashAlgorithm {\n    RMI_HASH_SHA_256 = 0,\n    RMI_HASH_SHA_512 = 1,\n}"},
    {"role": "assistant", "content": "pub enum RmiHashAlgorithm {\n  RMI_HASH_SHA_256,\n  RMI_HASH_SHA_512,\n}"}
  ],
  "metadata": {"version": "eac5", "type": "RmiHashAlgorithm", "kind": "type_definition"}
}
```

### Helper stub example (kind=helper_stub)
```json
{
  "messages": [
    {"role": "system",    "content": "You are a formal specification assistant ... generate the Verus uninterpreted spec function stub (a single line ending with ';')."},
    {"role": "user",      "content": "## Helper Function Specification (from RMM spec PDF)\n\nB3.3 AddrIsGranuleAligned function\n             Returns TRUE if address addr is aligned to the size of a Granule.\n             func AddrIsGranuleAligned(\n                 addr : Address) => boolean\n..."},
    {"role": "assistant", "content": "pub open spec fn AddrIsGranuleAligned(s: S, addr: Address) -> bool;"}
  ],
  "metadata": {"version": "eac5", "function": "AddrIsGranuleAligned", "kind": "helper_stub"}
}
```

### Command example (no kind field = command)
```json
{
  "messages": [
    {"role": "system",    "content": "You are a formal specification assistant ... generate the Verus specification function for that command."},
    {"role": "user",      "content": "## Context (shared Verus types and helper function signatures)\n\n```rust\n<last 200 lines of preamble.rs>\n```\n\n## Command Specification (from RMM spec PDF)\n\nB4.3.3 RMI_DATA_DESTROY command\n..."},
    {"role": "assistant", "content": "pub open spec fn rmi_data_destroy_spec(...) -> bool {\n  ...\n}"}
  ],
  "metadata": {"version": "eac5", "command": "RMI_DATA_DESTROY", "source_section": "RMI_DATA_DESTROY_command.txt"}
}
```

---

## Exact Steps to Execute on GPU Server

### Prerequisites

```bash
pip install trl peft transformers datasets accelerate bitsandbytes
```

Copy `train.py` from the template in `README_training.md` (it's the `train.py`
code block under "Training Script Template").

### Step 1 — Train L2 model (type definitions)

```bash
python3 - <<'EOF'
import json
for split, src in [('train','train'), ('val','val')]:
    with open(f'dataset/{src}.jsonl') as f, \
         open(f'dataset/{split}_types.jsonl', 'w') as out:
        for line in f:
            ex = json.loads(line)
            if ex['metadata'].get('kind') == 'type_definition':
                out.write(line)
EOF

python3 train.py \
    --train dataset/train_types.jsonl \
    --val   dataset/val_types.jsonl \
    --out   models/layer2 \
    --max-seq 4096
```

Expected: ~276 train + ~56 val examples. Fast to train (~15 min on A100).

### Step 2 — Generate cascaded type context

After Step 1, run the L2 model on each train version's type sections.
The goal: for each of {eac5, rel0, alp11, alp12}, generate all type definitions
and concatenate them into a single text file.

```python
# inference_l2.py — adapt this to your inference stack
import os, json
from transformers import AutoTokenizer, AutoModelForCausalLM
from peft import PeftModel

model_path = "models/layer2_best"  # or merge first
tokenizer  = AutoTokenizer.from_pretrained(model_path)
model      = AutoModelForCausalLM.from_pretrained(model_path, device_map="auto")

SYSTEM = ("You are a formal specification assistant for Arm CCA RMM. "
          "Given a type specification in ASL pseudocode, generate the Verus/Rust type definition.")

os.makedirs("generated_types", exist_ok=True)

for version in ["eac5", "rel0", "alp11", "alp12"]:
    types_dir = f"sections/{version}/types"
    all_defs  = []
    for fname in sorted(os.listdir(types_dir)):
        section_text = open(f"{types_dir}/{fname}").read().strip()
        messages = [
            {"role": "system",  "content": SYSTEM},
            {"role": "user",    "content": f"## Type Specification (from RMM spec PDF)\n\n{section_text}"},
        ]
        input_ids = tokenizer.apply_chat_template(
            messages, tokenize=True, add_generation_prompt=True, return_tensors="pt"
        ).to(model.device)
        out = model.generate(input_ids, max_new_tokens=512, do_sample=False)
        response = tokenizer.decode(out[0][input_ids.shape[-1]:], skip_special_tokens=True)
        all_defs.append(response.strip())
        print(f"  {version}/{fname[:-4]}: {response[:60]}...")

    with open(f"generated_types/{version}_types.rs", "w") as f:
        f.write("\n\n".join(all_defs) + "\n")
    print(f"[L2] {version}: {len(all_defs)} types → generated_types/{version}_types.rs")
```

Then build the cascaded train set:

```bash
python3 substitute_context.py \
    --input   dataset/train.jsonl \
    --gen-dir generated_types/ \
    --output  dataset/train_cascaded.jsonl
```

Verify: `wc -l dataset/train_cascaded.jsonl` should be 938 (same count as train.jsonl).

### Step 3a — Train L3 model (helper stubs)

Can run in parallel with Step 2 / Step 3b — no dependency on L2.

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
    --out   models/layer3 \
    --max-seq 4096
```

Expected: 480 train + ~165 val examples.

### Step 4 — Train command model (cascaded)

Requires `train_cascaded.jsonl` from Step 2.

```bash
python3 - <<'EOF'
import json
for split, src in [('train_cmds_cascaded','train_cascaded'), ('val_cmds','val')]:
    with open(f'dataset/{src}.jsonl') as f, \
         open(f'dataset/{split}.jsonl', 'w') as out:
        for line in f:
            ex = json.loads(line)
            if ex['metadata'].get('kind') not in ('type_definition', 'helper_stub'):
                out.write(line)
EOF

python3 train.py \
    --train dataset/train_cmds_cascaded.jsonl \
    --val   dataset/val_cmds.jsonl \
    --out   models/commands \
    --max-seq 8192
```

Expected: 476 train + 99 val examples. Longest training run (~45 min on A100).

### Step 5 — Evaluate end-to-end on alp14

```bash
# Merge adapters for standalone inference
for layer in layer2 layer3 commands; do
python3 -c "
from peft import PeftModel
from transformers import AutoModelForCausalLM, AutoTokenizer
m = AutoModelForCausalLM.from_pretrained('Qwen/Qwen2.5-Coder-7B-Instruct')
m = PeftModel.from_pretrained(m, 'models/${layer}_best')
m = m.merge_and_unload()
m.save_pretrained('models/${layer}_merged')
AutoTokenizer.from_pretrained('Qwen/Qwen2.5-Coder-7B-Instruct').save_pretrained('models/${layer}_merged')
"
done

# Run pipeline on alp14 (test set)
python3 pipeline.py \
    --txt       ccaspec/alp14.txt \
    --target    alp14 \
    --l1-rs     boilerplate/layer1.rs \
    --l2-model  models/layer2_merged \
    --l3-model  models/layer3_merged \
    --cmd-model models/commands_merged \
    --out       alp14_generated.rs

diff alp14_gold.rs alp14_generated.rs | head -100
```

Primary success criterion: `alp14_generated.rs` compiles under Verus.

---

## Things You Should Know

1. **alp14 is the test set** — do not train on it. Use it only for final eval.

2. **The `substitute_context.py` `--gen-dir` format**: expects files named
   `{version}_types.rs` (e.g., `eac5_types.rs`). Each file is all type
   definitions for that version concatenated, separated by blank lines.

3. **`pipeline.py --oracle` mode**: uses golden files instead of models,
   useful for sanity-checking the assembly pipeline without GPU. Already
   verified to exactly reconstruct all 6 gold files.

4. **`test_e2e_oracle.py`**: verifies that split + reassemble = gold, for all
   6 versions. Run it any time to confirm the pipeline is intact.

5. **Context window**: L2/L3 examples are small (< 4k tokens). Command examples
   use the last 200 lines of preamble as context (PREAMBLE_TAIL_LINES=200 in
   build_dataset.py) — longest are ~8k tokens.

6. **`metadata.kind` field**:
   - `"type_definition"` → L2 training
   - `"helper_stub"` → L3 training
   - absent → command training

7. **`train_cascaded.jsonl`**: command examples have their preamble context
   replaced with model-generated types. The `metadata.cascaded=True` field
   marks substituted examples. Non-command examples pass through unchanged.

8. **Gold files for eac5/rel0** have manual patches applied (scope had bugs).
   alp11+ use raw SCOPE output.

---

## Quick Sanity Checks (no GPU needed)

```bash
# All 6 versions reconstruct exactly from split files
for v in eac5 rel0 alp11 alp12 alp13 alp14; do
    python3 test_e2e_oracle.py --version $v
done
# Expected: PASS for all 6

# Dataset sizes
wc -l dataset/*.jsonl
# train.jsonl:         938
# val.jsonl:           320
# test.jsonl:          326
# train_helpers.jsonl: 480
```
