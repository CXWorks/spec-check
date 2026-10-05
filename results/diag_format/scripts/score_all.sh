#!/usr/bin/env bash
cd ~/spec-check
export VERUS_BIN=$HOME/verus/verus-x86-linux/verus
export RUSTUP_HOME=$HOME/rust/rustup CARGO_HOME=$HOME/rust/cargo PATH=$HOME/rust/cargo/bin:$PATH
.venv/bin/python ~/diag/score_diag.py
mkdir -p /tmp/equivdiag && cd /tmp/equivdiag
for v in alp14_ctl alp14_layout alp14_prose; do
  ~/spec-check/.venv/bin/python ~/spec-check/scripts/semantic_equiv.py ~/diag/eval-$v.json \
    --specs-dir ~/spec-check/training-dataset/specs/alp14 --jobs 24 --timeout 300 \
    --out ~/diag/equiv-$v.json
  echo "equiv done $v"
done
echo SCORE_DONE
