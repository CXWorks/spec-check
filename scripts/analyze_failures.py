#!/usr/bin/env python3
"""Re-check the specs an eval already generated, and report why they fail.

    python scripts/analyze_failures.py eval/*.json --out analysis.json

Needs Verus and the alp14 preamble, but NOT the network: it re-checks the
`generated` text already stored in each eval JSON rather than re-running the
model. That makes it the way to redo failure analysis after the classifier
changes, and the way to work while the checkpoint repo is unreachable.

It exists because the original eval stored only a reason string, and that string
came from a classifier that reported most arity and type errors as
`parse_error`. Both are fixed now, but the runs already scored under the old code
would otherwise have to be regenerated to be understood.

Reports three things the summary numbers do not:

  taxonomy   what actually breaks, under the corrected classifier
  overlap    which commands fail in EVERY run — the ones a model change would
             have to address, as opposed to those that flip with the seed
  vs gold    whether gold compiles for that command, since 7 of 40 do not and a
             model cannot be faulted for those
"""

import argparse
import collections
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))

GOLD_FAILS = {  # gold itself does not compile for these, under 0.2026.04.12.f1166c4
    "RMI_PSMMU_MSI_CONFIG", "RMI_REALM_CREATE", "RMI_VDEV_GET_STATE",
    "RSI_MEASUREMENT_READ", "RSI_MEM_SET_PERM_INDEX", "RSI_REALM_CONFIG",
    "RSI_VDEV_GET_INFO",
}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("files", nargs="+", help="eval JSONs")
    ap.add_argument("--out", default=None)
    ap.add_argument("--specs-dir", default=str(ROOT / "training-dataset/specs/alp14"))
    ap.add_argument("--verus-timeout", type=int, default=600)
    ap.add_argument("--jobs", type=int, default=8)
    args = ap.parse_args()

    from concurrent.futures import ThreadPoolExecutor
    from verify_generated_verus import check_text, find_verus_bin, read_preamble

    verus = find_verus_bin(None)
    if not verus:
        sys.exit("verus not found — set VERUS_BIN")
    preamble = read_preamble(Path(args.specs_dir) / "preamble.rs")

    runs = {}
    for f in args.files:
        name = Path(f).stem.replace("-final", "")
        runs[name] = {r["command"]: r for r in json.loads(Path(f).read_text())["results"]}

    # One work item per (run, command); Verus is a subprocess so threads suffice.
    work = [(n, c, r["generated"]) for n, rs in runs.items() for c, r in rs.items()]
    print(f"[analyze] re-checking {len(work)} generations from {len(runs)} runs",
          flush=True)
    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        checked = list(pool.map(
            lambda t: check_text(verus, preamble, t[1], t[2], args.verus_timeout), work))

    out = collections.defaultdict(dict)
    for (n, c, gen), chk in zip(work, checked):
        out[n][c] = {
            "pass": chk.status == "pass",
            "reason_old": runs[n][c]["reason"],
            "reason": chk.reason,
            "output_head": chk.output_head,
            "gold_compiles": c not in GOLD_FAILS,
            "chars": len(gen),
        }

    print("\n=== taxonomy (corrected classifier; old label in brackets when it moved)")
    for n, rs in out.items():
        tax = collections.Counter(v["reason"] if not v["pass"] else "PASS"
                                  for v in rs.values())
        moved = collections.Counter(
            f"{v['reason_old']} -> {v['reason']}" for v in rs.values()
            if not v["pass"] and v["reason_old"] != v["reason"])
        print(f"\n  {n}: " + "  ".join(f"{k}={v}" for k, v in tax.most_common()))
        for k, v in moved.most_common():
            print(f"      reclassified {v:2d}x  {k}")

    cmds = sorted(set.intersection(*[set(r) for r in out.values()]))
    print(f"\n=== overlap across {len(out)} runs ({len(cmds)} commands)")
    never = [c for c in cmds if not any(out[n][c]["pass"] for n in out)]
    always = [c for c in cmds if all(out[n][c]["pass"] for n in out)]
    flips = [c for c in cmds if c not in never and c not in always]
    print(f"  always pass : {len(always)}")
    print(f"  never pass  : {len(never)}  ({len([c for c in never if c in GOLD_FAILS])} "
          f"of them gold cannot compile either)")
    print(f"  seed/config dependent : {len(flips)}  <- this is the noise floor")

    print("\n=== never-passing commands where gold DOES compile")
    hard = [c for c in never if c not in GOLD_FAILS]
    for c in hard:
        rs = collections.Counter(out[n][c]["reason"] for n in out)
        print(f"  {c:32s} {dict(rs)}")

    if args.out:
        Path(args.out).write_text(json.dumps(
            {"per_run": out, "never": never, "always": always, "flips": flips,
             "hard_gold_compiles": hard}, indent=2))
        print(f"\n[analyze] wrote {args.out}")


if __name__ == "__main__":
    main()
