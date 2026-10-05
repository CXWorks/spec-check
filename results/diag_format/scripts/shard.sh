#!/usr/bin/env bash
# shard.sh <version> <gpu> <k>: hand the last k commands of <version> to <gpu>,
# and stop the original process once it has written the other 49-k.
v=$1; g=$2; k=$3; head_n=$((49-k))
cd ~/spec-check/training-dataset
mkdir -p sections/${v}_t specs/${v}_t
cp specs/$v/preamble.rs specs/${v}_t/
for f in $(ls sections/$v | sort | tail -n $k); do
  cp sections/$v/$f sections/${v}_t/
  c=${f%_command.txt}; cp specs/$v/${c,,}_spec.rs specs/${v}_t/
done
echo "shard ${v}_t: $(ls sections/${v}_t | wc -l) commands on gpu$g"
bash ~/diag/run_diag.sh $g:${v}_t
until [ "$(ls ~/diag/gen/$v | wc -l)" -ge $head_n ]; do sleep 15; done
pkill -f "versions $v --out-dir" && echo "stopped original $v at $(ls ~/diag/gen/$v | wc -l) files"
