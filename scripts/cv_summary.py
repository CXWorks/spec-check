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

Note what the rate measures. `pass` in these files is check_text's status: Verus
accepted the generated function with zero errors. It is a COMPILE rate. Whether
the spec says what gold says is a different axis, measured by semantic_equiv,
and it is roughly half as high (20.4% / 22.4% against 36.7% / 44.9%). Nothing in
this repo labelled the distinction on the number itself, which is an easy way to
read a syntax result as a correctness result.
"""

import argparse
import collections
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
            core_all[r["command"]] = r
        p = sum(r["pass"] for r in core)
        rows.append((name, p, len(core), 100 * p / len(core) if core else 0,
                     d["summary"]["pass"], d["summary"]["n"]))

    if missing:
        print(f"  not yet available: {', '.join(missing)}\n")
    if not rows:
        sys.exit("nothing to summarise")

    print("  rate = Verus accepts the generated spec (zero errors). This is a")
    print("  COMPILE rate, not a correctness rate -- `pass` in these files is")
    print("  check_text status, and agreement with gold is measured separately by")
    print("  semantic_equiv (20.4% / 22.4% where this reports 36.7% / 44.9%).")
    print()
    print(f"  {'fold':10s} {'core':>12s}  {'rate':>7s}   {'(all held out)':>16s}")
    for name, p, n, rate, ap_, an in rows:
        print(f"  {name:10s} {p:5d}/{n:<6d} {rate:6.1f}%   {ap_:5d}/{an:<5d}")

    rates = [r[3] for r in rows]
    print(f"\n  core commands scored: {len(core_all)}")
    if len(rates) > 1:
        print(f"  fold rates: mean {statistics.mean(rates):.1f}%  "
              f"range {max(rates)-min(rates):.1f}pp  sd {statistics.stdev(rates):.1f}pp")
    if len(rows) == args.folds:
        tot = sum(bool(v["pass"]) for v in core_all.values())
        n = len(core_all)
        p_hat = tot / n
        print(f"  pooled over all folds: {tot}/{n} = {100*p_hat:.1f}%"
              f"  (SE {100*(p_hat*(1-p_hat)/n)**0.5:.1f}pp)")

        # Fold rates scatter even if every command is equally hard, purely from
        # splitting ~16 Bernoulli draws into blocks. Comparing the observed
        # scatter against that floor is the difference between "some command
        # subsets are harder" and "this is what random partitioning looks like" --
        # and reporting the first without checking is how four other conclusions
        # tonight went wrong.
        exp_sd = statistics.mean(
            [(p_hat * (1 - p_hat) / r[2]) ** 0.5 for r in rows]) * 100
        obs_sd = statistics.stdev(rates)
        print(f"\n  fold sd observed  {obs_sd:.1f}pp")
        print(f"  fold sd expected  {exp_sd:.1f}pp  from binomial sampling alone"
              f" (n~{round(statistics.mean([r[2] for r in rows]))} per fold)")
        if obs_sd <= exp_sd:
            print("  -> observed <= expected: the folds are consistent with every\n"
                  "     command being equally hard. No evidence of a subset effect;\n"
                  "     the spread is the price of a small test set, not a property\n"
                  "     of which commands are in it.")
        else:
            print(f"  -> observed exceeds expected by {obs_sd-exp_sd:.1f}pp: some of the\n"
                  "     spread is the command subset rather than sampling.")
        print("\n  Either way the pooled SE above, not the 1.2pp seed sd, is the error\n"
              "  bar a single-split comparison deserves. Seeds and command choice are\n"
              "  separate sources and the smaller one was being quoted alone.")

        # Every core command is scored exactly once across the folds, by a model
        # that never trained on it, so these two breakdowns are the cleanest
        # description of the failure this repo has. They are descriptive: the
        # per-family n is 7-48, so read the counts, not a ranking.
        print(f"\n  why the {n - tot} failures fail:")
        for reason, k in collections.Counter(
                v["reason"] for v in core_all.values()).most_common():
            if reason != "ok":
                print(f"    {k:3d}  {reason}")
        print("\n  by command family (compile rate, n in brackets):")

        def family(c):
            return "RMI_RTT_*" if c.startswith("RMI_RTT") else c.split("_")[0] + "_*"

        fam = collections.defaultdict(lambda: [0, 0])
        for c, v in core_all.items():
            fam[family(c)][0] += bool(v["pass"])
            fam[family(c)][1] += 1
        for f, (p_, n_) in sorted(fam.items(), key=lambda kv: -kv[1][1]):
            se = 100 * ((p_ / n_) * (1 - p_ / n_) / n_) ** 0.5
            print(f"    {f:12s} {p_:2d}/{n_:<3d} {100*p_/n_:5.1f}%  (SE {se:.1f}pp)")


if __name__ == "__main__":
    main()
