#!/usr/bin/env python3
"""Compare eval runs on the 40 held-out commands, paired rather than marginally.

    # two runs, head to head
    python scripts/compare_runs.py eval/sft2-0-final.json eval/sft2-1-final.json

    # a configuration's seed replicates, pooled
    python scripts/compare_runs.py --group bf16 eval/sft2-0*-final.json \
                                   --group fp16 eval/sft2-1*-final.json

Why paired: every run is scored on the same 40 commands, so command difficulty —
by far the largest noise source — is shared and can be cancelled. Comparing the
two marginal rates throws that away. With 40 commands the marginal comparison
cannot resolve anything below ~22pp; in the first bf16/fp16 comparison 34 of the
40 commands behaved identically under both runs, meaning the marginal test was
diluting a 6-command signal across 40 samples.

Enlarging the eval set is not an option here — the split is by command name, so
test commands come straight out of training. See docs/gpu-and-runs.md
"Statistical power".
"""

import argparse
import glob
import json
import sys
from itertools import combinations
from math import comb
from pathlib import Path


def load(path):
    """{command: passed} for one eval JSON."""
    d = json.loads(Path(path).read_text())
    return {r["command"]: bool(r["pass"]) for r in d["results"]}


def summarise(path):
    """One row per run: the numbers that have to be read together.

    Kept in one table because each of these has, on its own, produced a wrong
    conclusion in this project. A pass rate hid a decode budget that truncated a
    third of one run's outputs; a non-degeneracy score of 39/40 hid a model
    emitting the same conjunct 156 times; and pass@1 alone cannot distinguish a
    model that does not know an answer from one that does not rank it first.
    """
    d = json.loads(Path(path).read_text())
    rs, s = d["results"], d["summary"]
    n = len(rs)
    g = sum(r["pass"] for r in rs)
    anyp = sum(r.get("any_pass", r["pass"]) for r in rs)
    trunc = sum(r.get("n_truncated", 0) > 0 for r in rs)
    rep = sum(bool(r["degeneracy"].get("repetitive")) for r in rs)
    nd = sum(1 for r in rs if not r["degeneracy"]["trivial_body"]
             and not r["degeneracy"]["no_implication"]
             and not r["degeneracy"].get("repetitive"))
    k = (s.get("sampling") or {}).get("samples")
    dist = [r["n_distinct"] for r in rs if r.get("n_distinct")]
    # A field that was never recorded must print as "-", not 0. Runs scored
    # before these checks existed have no `repetitive` or `n_truncated` key, and
    # showing 0 would assert that sft2-3 had no repetitive outputs when it in
    # fact had seven — the same "absent read as measured-zero" mistake that let a
    # 2048-token cap pass for a modelling result. Re-score old runs with
    # scripts/analyze_failures.py instead of reading a default here.
    has_rep = any("repetitive" in r["degeneracy"] for r in rs)
    has_tr = any("n_truncated" in r for r in rs)
    return {
        "run": Path(path).stem, "n": n,
        "pass@1": f"{g}/{n} ({100*g/n:.1f}%)",
        "pass@k": f"{anyp}/{n} ({100*anyp/n:.1f}%)" if k else "-",
        "recovered": sum(1 for r in rs if r.get("any_pass") and not r["pass"]) if k else "-",
        "non_degen": f"{nd}/{n}" if has_rep else f"{nd}/{n}*",
        "repetitive": rep if has_rep else "-",
        "truncated": trunc if has_tr else "-",
        "distinct": f"{sum(dist)/len(dist):.1f}/{k+1}" if dist and k else "-",
    }


def mcnemar_exact(b, c):
    """Two-sided exact p for the discordant pairs.

    Under the null the b+c disagreements split 50/50, so this is a sign test on
    them. Exact rather than the chi-square approximation because b+c is single
    digits here, where the approximation is not trustworthy.
    """
    n = b + c
    if n == 0:
        return 1.0
    k = min(b, c)
    tail = sum(comb(n, i) for i in range(k + 1)) / 2 ** n
    return min(1.0, 2 * tail)


def compare(name_a, runs_a, name_b, runs_b):
    """Paired comparison. Multiple runs per side are pooled by majority vote.

    Majority vote across seeds, not mean rate: it keeps the result per-command,
    which is what the pairing needs. Ties (even seed counts) count as a pass only
    if strictly more than half passed, so a tie resolves against the claim.
    """
    commands = sorted(set(runs_a[0]) & set(runs_b[0]))
    if not commands:
        sys.exit("no commands in common — are these eval files from the same split?")

    def vote(runs, cmd):
        return sum(r.get(cmd, False) for r in runs) * 2 > len(runs)

    both = a_only = b_only = neither = 0
    for cmd in commands:
        pa, pb = vote(runs_a, cmd), vote(runs_b, cmd)
        both += pa and pb
        a_only += pa and not pb
        b_only += pb and not pa
        neither += not pa and not pb

    p = mcnemar_exact(a_only, b_only)
    n = len(commands)
    print(f"\n{name_a} ({len(runs_a)} run{'s' * (len(runs_a) > 1)}) "
          f"vs {name_b} ({len(runs_b)} run{'s' * (len(runs_b) > 1)}) "
          f"on {n} commands\n")
    print(f"  both pass          {both:3d}")
    print(f"  both fail          {neither:3d}   <- {both + neither} commands "
          f"({100 * (both + neither) / n:.0f}%) carry no information")
    print(f"  only {name_a:<12s}  {a_only:3d}")
    print(f"  only {name_b:<12s}  {b_only:3d}")
    print(f"\n  {name_a}: {both + a_only}/{n} ({100 * (both + a_only) / n:.1f}%)   "
          f"{name_b}: {both + b_only}/{n} ({100 * (both + b_only) / n:.1f}%)")
    print(f"  McNemar exact p = {p:.3f} on {a_only + b_only} disagreements  "
          f"-> {'SIGNIFICANT' if p < 0.05 else 'not significant'} at 0.05")
    if p >= 0.05 and a_only != b_only:
        lead = name_a if a_only > b_only else name_b
        print(f"  ({lead} leads, but not by enough to rule out seed noise.)")
    return p


def seed_spread(name, runs, paths):
    """How far apart the replicates are. This is the noise floor, measured
    rather than assumed — a between-config gap smaller than this means nothing."""
    if len(runs) < 2:
        return
    rates = [100 * sum(r.values()) / len(r) for r in runs]
    print(f"\n{name} seed spread: " +
          ", ".join(f"{Path(p).stem}={x:.1f}%" for p, x in zip(paths, rates)))
    print(f"  range {min(rates):.1f}-{max(rates):.1f}%  (spread {max(rates) - min(rates):.1f}pp)")
    for (p1, r1), (p2, r2) in combinations(zip(paths, runs), 2):
        d = sum(1 for c in set(r1) & set(r2) if r1[c] != r2[c])
        print(f"  {Path(p1).stem} vs {Path(p2).stem}: disagree on {d} commands")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="*", help="two eval JSONs, when not using --group")
    ap.add_argument("--group", action="append", nargs="+", metavar=("NAME", "GLOB"),
                    help="--group <name> <glob>... ; repeat for the other side")
    ap.add_argument("--table", action="store_true",
                    help="Summary row per file, no pairing. Accepts any number "
                         "of files, unlike the paired comparison.")
    args = ap.parse_args()

    if args.table:
        rows = [summarise(f) for f in sorted(args.files)]
        cols = list(rows[0])
        w = {c: max(len(c), max(len(str(r[c])) for r in rows)) for c in cols}
        print("  ".join(c.ljust(w[c]) for c in cols))
        print("  ".join("-" * w[c] for c in cols))
        for r in rows:
            print("  ".join(str(r[c]).ljust(w[c]) for c in cols))
        return

    if args.group:
        if len(args.group) != 2:
            sys.exit("--group must be given exactly twice (one per side)")
        sides = []
        for g in args.group:
            name, pats = g[0], g[1:]
            paths = sorted({p for pat in pats for p in glob.glob(pat)})
            if not paths:
                sys.exit(f"group '{name}' matched no files: {pats}")
            sides.append((name, [load(p) for p in paths], paths))
    else:
        if len(args.files) != 2:
            sys.exit("give two eval JSONs, or use --group twice")
        sides = [(Path(f).stem, [load(f)], [f]) for f in args.files]

    for name, runs, paths in sides:
        seed_spread(name, runs, paths)
    compare(sides[0][0], sides[0][1], sides[1][0], sides[1][1])


if __name__ == "__main__":
    main()
