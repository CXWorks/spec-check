#!/usr/bin/env bash
# Runs once every round-2 generation process is gone: merge shards, verify
# coverage, then compile + equivalence-check D (the only round-2 arm whose
# vocabulary still matches the alp14 preamble).
until grep -q "^shard" ~/diag/logs/shard_C.log 2>/dev/null; do sleep 30; done
quiet=0
while [ $quiet -lt 2 ]; do
  if pgrep -f "scripts/gen_specs.py" >/dev/null; then quiet=0; else quiet=$((quiet+1)); fi
  sleep 30
done
cd ~/diag/gen
for v in alp14_full_ren alp14_prose_ren; do
  for f in ${v}_t/*.rs; do
    b=$(basename $f)
    if [ -e $v/$b ] && ! cmp -s $f $v/$b; then echo "DIFFERS $v/$b (kept shard copy)"; fi
    cp $f $v/$b
  done
done
for v in alp14_full alp14_prose_ren alp14_full_ren; do echo "COVERAGE $v $(ls $v | wc -l)/49"; done
cd ~/spec-check
export VERUS_BIN=$HOME/verus/verus-x86-linux/verus
export RUSTUP_HOME=$HOME/rust/rustup CARGO_HOME=$HOME/rust/cargo PATH=$HOME/rust/cargo/bin:$PATH
.venv/bin/python ~/diag/score_diag.py alp14_full
mkdir -p /tmp/equivdiag && cd /tmp/equivdiag
~/spec-check/.venv/bin/python ~/spec-check/scripts/semantic_equiv.py ~/diag/eval-alp14_full.json \
  --specs-dir ~/spec-check/training-dataset/specs/alp14 --jobs 24 --timeout 300 \
  --out ~/diag/equiv-alp14_full.json
echo ROUND2_DONE
