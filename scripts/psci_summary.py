#!/usr/bin/env python3
"""Pull PSCI gen artifacts from the checkpoint repo and put the arms in one table.

    python3 scripts/psci_summary.py sft3-2-final sft3-0-final
    python3 scripts/psci_summary.py --local results/psci/sft3-2-final

Each artifact is a `gen/<name>.tgz` holding `<version>/<command>.rs` plus the
`sweep-<version>.json` written beside them by the entrypoint. The sweep is the
measurement -- PSCI has no gold, so there is no agreement axis at all -- and its
own self-test is what says whether a finding count means anything.

Columns:
  compiles   spec fns Verus accepted, over spec fns found
  unsat      no input satisfies the spec: it forbids everything
  vacuous    every input satisfies it: it forbids nothing
  findings   unsat + vacuous
  detector   whether the sweep passed its own fixtures in that run

An arm whose detector column is not `sound` has an uninterpretable findings
count and is printed with a warning rather than dropped, because dropping it
silently is how a broken run becomes a clean-looking table.
"""

import argparse
import io
import json
import os
import sys
import tarfile
import urllib.request
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def hf_token():
    tok = os.environ.get("HF_TOKEN")
    if tok:
        return tok
    env = ROOT / ".env"          # gitignored; never leaves this machine
    if env.exists():
        for line in env.read_text().splitlines():
            if line.strip().startswith("HF_TOKEN="):
                return line.split("=", 1)[1].strip().strip('"').strip("'")
    sys.exit("no HF_TOKEN in env or .env")


def fetch(name, dest):
    dest = Path(dest)
    if dest.exists() and any(dest.rglob("*.rs")):
        print(f"  {name}: cached at {dest}")
        return dest
    repo = os.environ.get("HF_CKPT_REPO", "jisenli/spec-check-ckpt")
    url = f"https://huggingface.co/{repo}/resolve/main/gen/{name}.tgz"
    req = urllib.request.Request(url, headers={"Authorization": f"Bearer {hf_token()}"})
    dest.mkdir(parents=True, exist_ok=True)
    with tarfile.open(fileobj=io.BytesIO(urllib.request.urlopen(req).read())) as t:
        t.extractall(dest)
    print(f"  {name}: fetched -> {dest}")
    return dest


def find_sweeps(root):
    return sorted(Path(root).rglob("sweep-*.json"))


def summarise(path):
    doc = json.loads(Path(path).read_text())
    rows = doc.get("rows", [])
    c = Counter(r["verdict"] for r in rows)
    total = len([r for r in rows if r.get("fn")])
    compiles = total - c.get("compile_error", 0) - c.get("timeout", 0)
    st = doc.get("self_test") or []
    sound = doc.get("detector_sound")
    st_txt = ("sound" if sound else
              f"UNSOUND {sum(1 for r in st if r.get('pass'))}/{len(st)}" if st
              else "not run")
    return {
        "version": Path(path).name[len("sweep-"):-len(".json")],
        "spec_fns": total,
        "compiles": compiles,
        "unsat": c.get("unsat", 0),
        "vacuous": c.get("vacuous", 0),
        "consistent": c.get("consistent", 0),
        "compile_error": c.get("compile_error", 0),
        "timeout": c.get("timeout", 0),
        "detector": st_txt,
        "findings": [r for r in rows if r["verdict"] in ("unsat", "vacuous")],
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("names", nargs="*", help="artifact names, e.g. sft3-2-final")
    ap.add_argument("--local", nargs="*", default=[],
                    help="already-extracted directories, skipping the download")
    ap.add_argument("--cache", default="results/psci")
    args = ap.parse_args()

    roots = [(Path(d).name, Path(d)) for d in args.local]
    for n in args.names:
        roots.append((n, fetch(n, Path(args.cache) / n)))
    if not roots:
        sys.exit("give at least one artifact name or --local dir")

    print(f"\n{'arm':22s} {'ver':14s} {'fns':>4s} {'compiles':>9s} "
          f"{'unsat':>6s} {'vacuous':>8s} {'findings':>9s}  detector")
    print("-" * 96)
    allrows = []
    for name, root in roots:
        sweeps = find_sweeps(root)
        if not sweeps:
            print(f"{name:22s} (no sweep-*.json in artifact -- was SWEEP=1 set?)")
            continue
        for sp in sweeps:
            s = summarise(sp)
            s["arm"] = name
            allrows.append(s)
            print(f"{name:22s} {s['version']:14s} {s['spec_fns']:4d} "
                  f"{s['compiles']:9d} {s['unsat']:6d} {s['vacuous']:8d} "
                  f"{len(s['findings']):9d}  {s['detector']}")

    bad = [s for s in allrows if not s["detector"].startswith("sound")]
    if bad:
        print("\nWARNING: the detector did not pass its own fixtures in "
              f"{len(bad)} arm(s). Those findings counts are not interpretable.")

    for s in allrows:
        if not s["findings"]:
            continue
        print(f"\n--- {s['arm']} / {s['version']}: {len(s['findings'])} findings ---")
        for f in s["findings"]:
            print(f"  {f['verdict']:8s} {f['file']}::{f['fn']}"
                  + ("  [body is literally true]" if f.get("trivial") else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())
