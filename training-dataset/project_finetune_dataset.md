---
name: RMM→Verus fine-tuning dataset pipeline
description: Graduated 3-layer training pipeline (L2 types, L3 helper stubs, CMD) for LLM to replace SCOPE's heuristic PDF→Verus pipeline
type: project
---

Pipeline in /mnt/sdc/xiang/spec-gen/ targeting all 6 RMM versions (eac5, rel0, alp11, alp12, alp13, alp14).

## Current state (2026-03-28)
- All data preparation complete, oracle test passing for all 6 versions
- `/mnt/sdc/xiang/spec-gen-release/` — clean folder ready to push to GitHub (28MB uncompressed)
- `/mnt/sdc/xiang/spec-gen-release.tar.gz` — 1.3MB tarball of release folder
- `/mnt/sdc/xiang/spec-gen/rmm_finetune_data.tar.gz` — 1.3MB tarball for GPU server transfer
- GPU training not yet started

## Release folder layout (spec-gen-release/)
```
boilerplate/layer1.rs      # hardcoded type aliases + constants
dataset/                   # train(938), val(320), test(326), train_helpers(480)
gold/                      # {version}_gold.rs for all 6 versions
sections/{version}/        # command/type/helper PDF section text
specs/{version}/           # split gold Verus (types/, helpers/, *_spec.rs, epilogue.rs)
*.py                       # all pipeline scripts
README_training.md         # full GPU training instructions + train.py template
CONTEXT.md                 # briefing doc for new agent session on GPU server
```

## Architecture (3 trained layers)
- **L1**: hardcoded (`boilerplate/layer1.rs`)
- **L2**: PDF type section → `pub enum` / `struct` (train on `kind=type_definition` examples)
- **L3**: PDF B3.x helper fn section → `pub open spec fn ...;` stub (train on `train_helpers.jsonl`)
- **CMD**: PDF command section + generated L2+L3 context → `_spec` function (train on `train_cascaded.jsonl`)

## Training order (cascaded — must follow this sequence)
1. Train L2 on type examples → run on train PDFs → `generated_types/{version}_types.rs`
2. `substitute_context.py --gen-dir generated_types/` → `train_cascaded.jsonl`
3. Train L3 on `train_helpers.jsonl` (parallel with step 2)
4. Train CMD on `train_cascaded.jsonl` command examples

## Key scripts
- `extract_sections.py`, `split_specs.py`, `build_dataset.py` — data pipeline
- `pipeline.py` — end-to-end PDF→.rs (oracle mode verified; real mode needs trained models)
- `test_e2e_oracle.py` — reconstruction test, PASS for all 6 versions
- `substitute_context.py` — replaces golden L2 context with model-generated context
- `CONTEXT.md` — full briefing for new agent session on GPU server

## Split
- train: eac5, rel0, alp11, alp12 | val: alp13 | test: alp14 (never train on alp14)

## Why cascaded
At inference time command model sees generated (imperfect) L2 types as context.
Training on golden L2 context causes covariate shift — cascaded training closes that gap.
