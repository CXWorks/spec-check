#!/usr/bin/env bash
# Compile + equivalence for the prompt ablation on alp14 held-out 49.
mkdir -p ~/diag/gen/alp14_ctl_invent && cp ~/diag/gen3/ft-invent/alp14_ctl/*.rs ~/diag/gen/alp14_ctl_invent/
cd ~/spec-check
export VERUS_BIN=$HOME/verus/verus-x86-linux/verus RUSTUP_HOME=$HOME/rust/rustup CARGO_HOME=$HOME/rust/cargo PATH=$HOME/rust/cargo/bin:$PATH
.venv/bin/python ~/diag/score_diag.py alp14_ctl_invent
mkdir -p /tmp/equivdiag && cd /tmp/equivdiag
~/spec-check/.venv/bin/python ~/spec-check/scripts/semantic_equiv.py ~/diag/eval-alp14_ctl_invent.json \
  --specs-dir ~/spec-check/training-dataset/specs/alp14 --jobs 16 --timeout 300 --out ~/diag/equiv-alp14_ctl_invent.json
echo SCORE_INVENT_DONE
