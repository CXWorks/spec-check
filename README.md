# spec-check

**Detecting Inconsistencies in ARM CCA's Formally Verified Specification** (ASPLOS 2026)

This repository contains three complementary approaches for converting ARM Confidential Compute Architecture (CCA) Realm Management Monitor (RMM) specification PDFs into [Verus](https://github.com/verus-lang/verus) formal verification code:

1. **SCOPE** — a hand-written 2,400-line Python pipeline (submodule)
2. **Fine-tuned LLM pipeline** — a Qwen3-4B model trained to replace SCOPE's heuristics
3. **Prompt Engineering + Claude** — zero-shot / few-shot prompting with Claude API (no fine-tuning)

The generated Verus code encodes each RMM command's preconditions and postconditions as `pub open spec fn` functions, enabling formal inconsistency detection across specification versions.

---

## Repository Structure

```
spec-check/
├── scope/                    # SCOPE tool (git submodule)
├── dataset_loader.py         # Load raw PDF sections + gold specs (prompt eng path)
├── prompt_engineering.py     # Prompt variants + Claude API + A/B testing
├── training/                 # LLM fine-tuning scripts and docs
│   ├── train.py              # Unsloth+Qwen3-4B SFTTrainer
│   ├── inference_l2.py       # L2 model inference → cascaded context
│   ├── pipeline.py           # End-to-end inference pipeline
│   ├── build_dataset.py      # Build JSONL datasets from PDF sections + gold specs
│   ├── extract_sections.py   # Extract per-command/type/helper sections from PDF text
│   ├── split_specs.py        # Split gold .rs files into per-command components
│   ├── substitute_context.py # Replace golden L2 context with model-generated context
│   ├── test_e2e_oracle.py    # Validate pipeline assembly without GPU
│   ├── boilerplate/layer1.rs # Hardcoded type aliases and constants (never trained)
│   ├── README_training.md    # Full GPU training guide
│   ├── CONTEXT.md            # Architecture overview and GPU server briefing
│   └── STATUS.md             # Training progress and lessons learned
└── training-dataset/         # Prepared dataset and gold specifications
    ├── dataset/              # JSONL training/val/test splits (1,584 examples)
    ├── gold/                 # Complete gold Verus files for all 6 versions
    ├── sections/{version}/   # Per-command/type/helper raw PDF text
    └── specs/{version}/      # Gold Verus split into per-command components
```

---

## Supported Specifications

| Target | Version    | Split    |
|--------|------------|----------|
| eac5   | 1.0-eac5   | Train    |
| rel0   | 1.0-rel0   | Train    |
| alp11  | 1.1-alp11  | Train    |
| alp12  | 1.1-alp12  | Train    |
| alp13  | 1.1-alp13  | Val      |
| alp14  | 1.1-alp14  | Test     |

---

## Approach 1: SCOPE

SCOPE (*spectroscope*) is the baseline hand-written pipeline. It parses RMM specification PDFs and generates Verus `.rs` files with three processing modes:

| Mode    | Description                                      |
|---------|--------------------------------------------------|
| `reason`| Full conversion to Verus verification code       |
| `rule`  | Rule-based analysis and validation (no codegen)  |
| `raw`   | Document parsing only                            |

See [`scope/README.md`](scope/README.md) and [`scope/USAGE.md`](scope/USAGE.md) for installation and usage.

---

## Approach 2: LLM Fine-Tuning Pipeline

The training pipeline trains a **Qwen3-4B** model (via Unsloth QLoRA) using a three-layer cascaded architecture:

```
PDF → [L2 model] → type definitions  (pub enum / struct)
PDF → [L3 model] → helper stubs      (pub open spec fn ...;)
PDF + L2+L3 context → [CMD model]  → spec functions
```

**Layer 1** (`boilerplate/layer1.rs`): hardcoded type aliases and constants; never trained.

**Layer 2** (type definitions): fine-tuned on 179 `kind=type_definition` examples. Input: raw PDF type section. Output: Verus `pub enum` / `struct`.

**Layer 3** (helper stubs): fine-tuned on 480 `kind=helper_stub` examples. Input: B3.x helper function spec text. Output: single-line `pub open spec fn` stub.

**Command model**: fine-tuned on cascaded command examples (model-generated L2 context, not gold) to avoid covariate shift at inference time. Input: PDF command section + preamble context. Output: `pub open spec fn {cmd}_spec(...)`.

### Dataset

| Split | Versions           | Commands | Types | Helpers | Total |
|-------|--------------------|----------|-------|---------|-------|
| Train | eac5, rel0, alp11, alp12 | 379 | 163 | 496 | 938 |
| Val   | alp13              | 99       | 56    | 165     | 320   |
| Test  | alp14              | 98       | 57    | 171     | 326   |

All examples fit within an 8,192-token context window.

### Training Configuration

```
Base model:  unsloth/Qwen3-4B (4-bit QLoRA)
LoRA:        r=16, alpha=32, dropout=0.05
LR:          2e-4, cosine scheduler, 3% warmup
Epochs:      10
Precision:   FP16
max_seq:     4096 (L2/L3) | 6144 (CMD)
```

### Quick Start (GPU server)

```bash
pip install unsloth trl peft transformers datasets accelerate bitsandbytes xformers

# Step 1 — Train L2 (type definitions)
python3 train.py --train dataset/train_types.jsonl --val dataset/val_types.jsonl \
    --out models/layer2 --max-seq 4096

# Step 2 — Generate cascaded context and build train_cascaded.jsonl
python3 inference_l2.py       # → generated_types/{version}_types.rs
python3 substitute_context.py --input dataset/train.jsonl \
    --gen-dir generated_types/ --output dataset/train_cascaded.jsonl

# Step 3 — Train L3 (helper stubs, runs in parallel with Step 2)
python3 train.py --train dataset/train_helpers.jsonl --val dataset/val_helpers.jsonl \
    --out models/layer3 --max-seq 4096

# Step 4 — Train CMD model (cascaded)
python3 train.py --train dataset/train_cmds_cascaded.jsonl --val dataset/val_cmds.jsonl \
    --out models/commands --max-seq 8192

# Step 5 — End-to-end evaluation on alp14
python3 pipeline.py --txt ccaspec/alp14.txt --target alp14 \
    --l2-model models/layer2_best --l3-model models/layer3_best \
    --cmd-model models/commands_best --out alp14_generated.rs
```

See [`training/README_training.md`](training/README_training.md) for the full guide, and [`training/STATUS.md`](training/STATUS.md) for current training status.

### Sanity Check (no GPU needed)

```bash
# Verify split + reassemble = gold for all 6 versions
for v in eac5 rel0 alp11 alp12 alp13 alp14; do
    python3 test_e2e_oracle.py --version $v
done
# Expected: PASS for all 6
```

### Success Criteria

- **Primary**: `alp14_generated.rs` compiles under Verus without errors
- **Secondary**: BLEU/CodeBLEU vs `alp14_gold.rs`, predicate recall (fraction of `&&`-clauses recovered)

---

## Approach 3: Prompt Engineering + Claude API

An alternative to fine-tuning: use a large commercial model (Claude) with carefully designed prompts to generate Verus spec functions directly — no training required.

### Data Flow

```
sections/{version}/{CMD}_command.txt     ← Raw input (PDF-extracted command text)
specs/{version}/preamble.rs (tail 200L)  ← Context (Verus type/function signatures)
            ↓
    [Prompt Template + Claude API]
            ↓
pub open spec fn {cmd}_spec(...) -> bool  ← Generated output
            ↓
specs/{version}/{cmd}_spec.rs            ← Gold label (for evaluation)
```

This approach sends the raw command text + preamble context to Claude in a single call.

### Prompt Variants

Five prompt strategies are implemented for A/B testing:

| Variant | Strategy | Description |
|---------|----------|-------------|
| V0 | Baseline | System role + structured input/output template |
| V1 | Minimal | Spec + context only, minimal instructions |
| V2 | Few-shot | Includes an example input/output pair in system prompt |
| V3 | Structured | Step-by-step generation instructions (extract params → conditions → combine) |
| V4 | Best Practices | Explicit Verus idiom rules (implications, conjunctions, state pattern) |

### Quick Start

```bash
pip install anthropic

# Set API key
export ANTHROPIC_API_KEY="sk-..."

# Run A/B test on test split (alp14, 98 commands)
python3 prompt_engineering.py
```

### Key Files

| File | Purpose |
|------|---------|
| `dataset_loader.py` | Loads raw PDF sections + preamble context + gold specs from disk |
| `prompt_engineering.py` | 5 prompt variants, Claude Haiku API integration, A/B testing framework |

### Evaluation

Results are compared against the same `alp14` gold specs used by the fine-tuning pipeline, enabling direct comparison between approaches.

---

## License

SCOPE is licensed under Apache-2.0. See [`scope/LICENSE`](scope/LICENSE).
