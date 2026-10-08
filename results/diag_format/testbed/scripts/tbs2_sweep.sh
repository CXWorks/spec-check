#!/usr/bin/env bash
# Gold-free check on each testbed arm, against the real alp14 preamble.
cd ~/spec-check
export VERUS_BIN=$HOME/verus/verus-x86-linux/verus RUSTUP_HOME=$HOME/rust/rustup CARGO_HOME=$HOME/rust/cargo PATH=$HOME/rust/cargo/bin:$PATH
for a in tbs2_A tbs2_B tbs2_C tbs2_D tbs2_E tbs2_F tbs2_G tbs2_REF; do
  ( mkdir -p /tmp/sw_$a && cd /tmp/sw_$a && ~/spec-check/.venv/bin/python ~/spec-check/scripts/psci_sweep.py --gen-dir ~/diag/gen/$a \
      --preamble ~/spec-check/training-dataset/specs/alp14/preamble.rs --out ~/diag/sweep-$a.json --jobs 12 --skip-self-test > ~/diag/logs/sweep-$a.log 2>&1; echo "SWDONE $a" ) &
done
wait
echo TBS2_SWEEP_DONE
