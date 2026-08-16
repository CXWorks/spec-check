#!/usr/bin/env bash
# Poll HF until private-repo reads work again, then un-suspend the eval Jobs.
#
#   scripts/resume_when_quota_clears.sh [job ...]
#
# The account's private storage quota was exceeded, and HF answers every read of
# every private repo with 403 until its background GC reclaims the LFS blobs of
# deleted files. Deleting files and squashing history do not trigger that GC —
# `usedStorage` stayed at 103.97 GB with 19.32 GB actually on the branch — so the
# only thing to do is wait, and the only cost worth avoiding is waiting longer
# than necessary because nobody was watching.
#
# Probes an actual download rather than the `usedStorage` field: that field is
# what the enforcement is derived from, but it is cached, and what we care about
# is whether a job can now fetch its data.
set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."
export KUBECONFIG="${KUBECONFIG:-$HOME/.kube/boogiebonjour}"
JOBS=("$@")
[ ${#JOBS[@]} -eq 0 ] && JOBS=(de2-rl-test-bok-0 de2-rl-test-bok-1 de2-rl-test-bok-9b de2-rl-test-seed-4b)
INTERVAL="${INTERVAL:-300}"

probe() {
  python3 - <<'PY'
import os, sys, requests
for line in open('.env'):
    line = line.strip()
    if line and not line.startswith('#') and '=' in line:
        k, v = line.split('=', 1)
        os.environ.setdefault(k, v.strip().strip('"'))
h = {'Authorization': 'Bearer ' + os.environ['HF_TOKEN']}
try:
    used = requests.get(
        'https://huggingface.co/api/models/jisenli/spec-check-ckpt?expand[]=usedStorage',
        headers=h, timeout=60).json().get('usedStorage', 0) / 1e9
    r = requests.head(
        'https://huggingface.co/datasets/jisenli/spec-check-data/resolve/main/dataset_clean/train.jsonl',
        headers=h, timeout=30, allow_redirects=False)
    print(f'usedStorage={used:.2f}GB http={r.status_code}', flush=True)
    sys.exit(0 if r.status_code in (200, 302) else 1)
except Exception as e:                       # a network blip is not "still blocked"
    print(f'probe error: {type(e).__name__}', flush=True)
    sys.exit(1)
PY
}

echo "[resume] waiting for HF quota; probing every ${INTERVAL}s"
until probe; do sleep "$INTERVAL"; done

echo "[resume] reads restored — un-suspending ${#JOBS[@]} job(s)"
for j in "${JOBS[@]}"; do
  kubectl patch job "$j" -n default --type=merge -p '{"spec":{"suspend":false}}' \
    2>&1 | sed 's/^/  /' || echo "  $j: patch failed (already gone?)"
done
echo "[resume] done"
