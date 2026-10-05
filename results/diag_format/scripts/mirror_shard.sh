#!/usr/bin/env bash
# Spread the slow RMM-layout arm over GPUs as they free up. The original
# process (gpu2) runs psci_13_rmm then sdei_rmm; it is stopped once it has
# written the first 12 PSCI files, and the rest is covered by shards:
#   gpu5 now           sdei_rmm_a  (first 10 SDEI)
#   gpu1 after clean   sdei_rmm_b  (last 9 SDEI)
#   gpu0 after ctl     psci_13_rmm_t (last 8 PSCI)
cd ~/spec-check/training-dataset
export HF_HUB_OFFLINE=1 TRANSFORMERS_OFFLINE=1
AD=$HOME/.cache/huggingface/hub/models--jisenli--spec-check-ckpt/snapshots/7b59ac04600bb54d60a4fc30e1e0c0dc25675992
COMMON="--base Qwen/Qwen3.5-9B --adapter $AD --subfolder sft3-2/final --prompt-variant v3.1 --no-gold"
mk() { # mk <src version> <new version> <python slice>
  python3 - "$1" "$2" "$3" <<'PY'
import os, shutil, sys
src, dst, sl = sys.argv[1:]
names = sorted(os.listdir(f"sections/{src}"))
pick = eval(f"names[{sl}]")
os.makedirs(f"sections/{dst}", exist_ok=True); os.makedirs(f"specs/{dst}", exist_ok=True)
shutil.copy(f"specs/{src}/preamble.rs", f"specs/{dst}/preamble.rs")
for n in pick: shutil.copy(f"sections/{src}/{n}", f"sections/{dst}/{n}")
print(dst, len(pick), pick[0], "..", pick[-1])
PY
}
run() { # run <gpu> <version>
  (cd ~/spec-check && CUDA_VISIBLE_DEVICES=$1 setsid nohup .venv/bin/python scripts/gen_specs.py $COMMON \
     --versions $2 --out-dir ~/diag/gen2/rmm_s > ~/diag/logs/mirror-$2.log 2>&1 < /dev/null &)
  echo "gpu$1 $2 launched"
}
mk sdei_rmm sdei_rmm_a ":10"; mk sdei_rmm sdei_rmm_b "10:"; mk psci_13_rmm psci_13_rmm_t "12:"
run 5 sdei_rmm_a
( while pgrep -f "versions psci_13_clean sdei_clean" >/dev/null; do sleep 20; done; run 1 sdei_rmm_b ) &
( while pgrep -f "versions psci_13 sdei --out-dir" >/dev/null; do sleep 20; done; run 0 psci_13_rmm_t ) &
until [ "$(ls ~/diag/gen2/rmm/psci_13_rmm 2>/dev/null | wc -l)" -ge 12 ]; do sleep 15; done
pkill -f "versions psci_13_rmm sdei_rmm --out-dir" && echo "stopped original at $(ls ~/diag/gen2/rmm/psci_13_rmm | wc -l) PSCI files"
wait
