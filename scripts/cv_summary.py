#!/usr/bin/env python3
"""Summarise a k-fold cross-validation run.

    python3 scripts/cv_summary.py --folds 5

Answers the one question seed replicates cannot: would a different sample of
commands rank things the same way? Three seeds per configuration bound the
retraining noise, but they all share one 49-command test set, so none of them
tests whether that set is representative. Five folds evaluate every non-benchmark
alp14 command with a model that trained on none of it.

**Scored on core commands only.** Each fold's held-out set also contains the 17
benchmark commands, force-held-out in all five so the benchmarks stay measurable.
Including them would double-count 85 evaluations and mix in a subset that is
unusually hard, so they are dropped here and the folds are then disjoint --
5 folds x ~16 core commands = 81, each command scored exactly once.

The fold rates are NOT comparable to the 36.7% from `dataset_bench`: different
denominators, and each fold trains on 87-90 command names against that split's 72.
The comparison that means something is fold-to-fold. Tight clustering says a
single 49-command sample is representative and the existing conclusions carry.
A wide spread says any single-sample comparison -- including the 9B-over-4B gap --
needs an error bar wider than seed noise alone provides.
"""

import argparse
import json
import os
import statistics
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def hf_token():
    tok = os.environ.get("HF_TOKEN")
    if tok:
        return tok
    for line in (ROOT / ".env").read_text().splitlines():
        line = line.strip()
        if line.startswith("HF_TOKEN="):
            return line.split("=", 1)[1].strip().strip('"').strip("'")
    sys.exit("no HF_TOKEN")


def fetch(path, tok):
    u = f"https://huggingface.co/jisenli/spec-check-ckpt/resolve/main/{path}"
    req = urllib.request.Request(u, headers={"Authorization": f"Bearer {tok}"})
    return json.loads(urllib.request.urlopen(req).read())


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--folds", type=int, default=5)
    ap.add_argument("--prefix", default="cv3-f")
    ap.add_argument("--benchmark-commands",
                    default=str(ROOT / "training-dataset" / "benchmark_commands.json"))
    args = ap.parse_args()

    tok = hf_token()
    forced = set(json.loads(Path(args.benchmark_commands).read_text())["commands"])

    rows, core_all, missing = [], {}, []
    for k in range(1, args.folds + 1):
        name = f"{args.prefix}{k}"
        try:
            d = fetch(f"eval/{name}-final.json", tok)
        except Exception:
            missing.append(name)
            continue
        res = d["results"]
        core = [r for r in res if r["command"] not in forced]
        for r in core:
            # Disjointness is the design; assert it rather than trust it.
            if r["command"] in core_all:
                sys.exit(f"command {r['command']} scored in two folds -- "
                         "folds are not disjoint, the split is wrong")
            core_all[r["command"]] = r["pass"]
        p = sum(r["pass"] for r in core)
        rows.append((name, p, len(core), 100 * p / len(core) if core else 0,
                     d["summary"]["pass"], d["summary"]["n"]))

    if missing:
        print(f"  not yet available: {', '.join(missing)}\n")
    if not rows:
        sys.exit("nothing to summarise")

    print(f"  {'fold':10s} {'core':>12s}  {'rate':>7s}   {'(all held out)':>16s}")
    for name, p, n, rate, ap_, an in rows:
        print(f"  {name:10s} {p:5d}/{n:<6d} {rate:6.1f}%   {ap_:5d}/{an:<5d}")

    rates = [r[3] for r in rows]
    print(f"\n  core commands scored: {len(core_all)}")
    if len(rates) > 1:
        print(f"  fold rates: mean {statistics.mean(rates):.1f}%  "
              f"range {max(rates)-min(rates):.1f}pp  sd {statistics.stdev(rates):.1f}pp")
    if len(rows) == args.folds:
        tot = sum(v for v in core_all.values())
        print(f"  pooled over all folds: {tot}/{len(core_all)} = "
              f"{100*tot/len(core_all):.1f}%")
        print("\n  Read the fold sd against the seed sd measured on the same model\n"
              "  (4B: 1.2pp). Larger means command choice moves the number more than\n"
              "  retraining does, and every single-split comparison inherits that.")


if __name__ == "__main__":
    main()
