#!/usr/bin/env bash
# Waits for both code queues and Claude's direct specs, then compiles and
# equivalence-checks all eight arms against alp14 gold.
until grep -q "^QUEUE_DONE" ~/diag/logs/queue-tb-base-sig2.log 2>/dev/null && grep -q "^QUEUE_DONE" ~/diag/logs/queue-tb-ft-sig2.log 2>/dev/null && [ -f ~/diag/tbs_claude_direct_ready ]; do sleep 30; done
rm -rf ~/diag/gen/tbs2_A && cp -r ~/diag/gen/tbs_A ~/diag/gen/tbs2_A
G=~/diag/gen3
declare -A SRC=([tbs2_B]=$G/tb-base-sig2/alp14_ir_claude [tbs2_C]=$G/tb-ft-sig2/alp14_ir_claude [tbs2_D]=$G/tb-base-sig2/alp14_ir_base [tbs2_E]=$G/tb-ft-sig2/alp14_ir_ft [tbs2_F]=$G/tb-base-sig2/alp14_prose [tbs2_G]=$G/tb-ft-sig2/alp14_prose [tbs2_REF]=$G/tb-ft-sig2/alp14_ctl)
for a in "${!SRC[@]}"; do rm -rf ~/diag/gen/$a; mkdir -p ~/diag/gen/$a; cp ${SRC[$a]}/*.rs ~/diag/gen/$a/; done
for a in tbs2_A tbs2_B tbs2_C tbs2_D tbs2_E tbs2_F tbs2_G tbs2_REF; do echo "COUNT $a $(ls ~/diag/gen/$a | wc -l)"; done
cd ~/spec-check
export VERUS_BIN=$HOME/verus/verus-x86-linux/verus RUSTUP_HOME=$HOME/rust/rustup CARGO_HOME=$HOME/rust/cargo PATH=$HOME/rust/cargo/bin:$PATH
.venv/bin/python ~/diag/score_diag.py tbs2_A tbs2_B tbs2_C tbs2_D tbs2_E tbs2_F tbs2_G tbs2_REF
mkdir -p /tmp/equivtbs2 && cd /tmp/equivtbs2
for a in tbs2_A tbs2_B tbs2_C tbs2_D tbs2_E tbs2_F tbs2_G tbs2_REF; do
  ~/spec-check/.venv/bin/python ~/spec-check/scripts/semantic_equiv.py ~/diag/eval-$a.json \
    --specs-dir ~/spec-check/training-dataset/specs/alp14 --jobs 24 --timeout 300 --out ~/diag/equiv-$a.json > ~/diag/logs/equiv-$a.log 2>&1
  echo "EQUIV_DONE $a"
done
echo TBS2_SCORE_DONE
