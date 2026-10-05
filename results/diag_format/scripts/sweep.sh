#!/usr/bin/env bash
# sweep.sh <name>: compile + unsat/vacuous every spec under ~/diag/sweep/specs
# against its stub, write ~/diag/sweep/<name>.json, then print SWEEP_DONE.
cd ~/spec-check
export VERUS_BIN=$HOME/verus/verus-x86-linux/verus RUSTUP_HOME=$HOME/rust/rustup CARGO_HOME=$HOME/rust/cargo PATH=$HOME/rust/cargo/bin:$PATH
.venv/bin/python scripts/stub_sweep.py sweep --specs ~/diag/${2:-sweep}/specs --stubs ~/diag/${2:-sweep}/stubs --out ~/diag/${2:-sweep}/$1.json --jobs 24
echo "SWEEP_DONE $1"
