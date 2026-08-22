#!/usr/bin/env python3
"""Localise a spec disagreement to a row of the document.

    python3 scripts/clause_equiv.py --version eac5 --gen-dir DIR \
        --commands RMI_RTT_FOLD --verus-bin ...

`semantic_equiv.py` compares two whole specs and returns one verdict per command.
When that verdict is `incomparable` it says the two disagree somewhere, and
nothing about where. For the eight `incomparable` commands in the scaffolded
9B run, `provenance.py` then showed that gold and the model cover *exactly the
same document rows* — so the disagreement is inside a shared row, below the
resolution of both tools.

This closes that gap. For each row of the command's Failure/Success/Footprint
tables it collects the clauses each side wrote for that row, conjoins them, and
asks Z3 the same four-way question `semantic_equiv` asks of whole specs:

    agree            each side's clauses for this row mean the same thing
    model_stronger   the model forbids more here -- CANDIDATE gold omission
    gold_stronger    gold forbids more here -- candidate model omission
    differ           neither implication holds

The point is `model_stronger`. That is the shape of the one case where gold is
known to be wrong (`RMI_VSMMU_CREATE`'s missing `idr` frame condition), and at
whole-spec resolution it is indistinguishable from the model over-constraining
somewhere unrelated. Per row, it names the row of the PDF to go read.

**A row verdict is not a document verdict.** Z3 compares the two clause sets to
each other, not either of them to the English in the table. `agree` means the two
formalisations coincide, including when both are wrong the same way. Only a human
reading the row can promote a `model_stronger` into "gold is missing this".
"""

import argparse
import json
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "prompt_engineering"))

from provenance import parse_section, split_conjuncts, idents, load_section  # noqa: E402
from semantic_equiv import parse_spec, alpha_rename, run_verus  # noqa: E402


def clauses_for_row(clauses, clause_ids, row_ids, min_overlap):
    """Every clause that encodes this row. Many-to-many on purpose: a row can
    need several clauses (`idr[0..6]` is one row and seven conjuncts) and one
    clause can serve several rows."""
    return [c for c, ci in zip(clauses, clause_ids)
            if len(ci & row_ids) >= min_overlap]


def obligation(preamble, params_str, gold_body, cand_body, kind):
    """Same construction as semantic_equiv, but over one row's conjunction.

    Reusing that module's proof shape rather than writing a second one is
    deliberate: its `aborting due to N previous errors` handling took a
    correction to get right, and a parallel implementation would have to
    rediscover it.
    """
    concl = {
        "equiv": "cand_row(*) == gold_row(*)",
        "cand_implies_gold": "cand_row(*) ==> gold_row(*)",
        "gold_implies_cand": "gold_row(*) ==> cand_row(*)",
    }[kind]
    args = ", ".join(p.strip().split(":")[0].strip()
                     for p in params_str.strip("()").split(",") if ":" in p)
    concl = concl.replace("*", args)
    return (
        f"{preamble}\n\n"
        f"pub open spec fn gold_row{params_str} -> bool {{ {gold_body} }}\n\n"
        f"pub open spec fn cand_row{params_str} -> bool {{ {cand_body} }}\n\n"
        f"proof fn check{params_str}\n    ensures {concl}\n{{}}\n\n"
        "} // verus!\n"
    )


def verdict_for_row(verus, preamble, params_str, gold_cl, cand_cl, timeout):
    if not gold_cl and not cand_cl:
        return "neither_encodes", None
    if not cand_cl:
        return "model_missing", None
    if not gold_cl:
        return "gold_missing", None
    g = " && ".join(f"({c})" for c in gold_cl)
    c = " && ".join(f"({x})" for x in cand_cl)
    fwd, _ = run_verus(verus, obligation(preamble, params_str, g, c,
                                         "cand_implies_gold"), timeout)
    bwd, _ = run_verus(verus, obligation(preamble, params_str, g, c,
                                         "gold_implies_cand"), timeout)
    if "compile_error" in (fwd, bwd) or "timeout" in (fwd, bwd):
        # Never let a build failure masquerade as a disagreement.
        return f"unchecked({fwd}/{bwd})", None
    if fwd == "proved" and bwd == "proved":
        return "agree", None
    if fwd == "proved":
        return "model_stronger", None
    if bwd == "proved":
        return "gold_stronger", None
    return "differ", None


def read_preamble_text(version):
    from verify_generated_verus import read_preamble
    return read_preamble(ROOT / "training-dataset" / "specs" / version / "preamble.rs")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", default="eac5")
    ap.add_argument("--gen-dir", required=True)
    ap.add_argument("--commands", nargs="*", default=None)
    ap.add_argument("--verus-bin", default=None)
    ap.add_argument("--min-overlap", type=int, default=2)
    ap.add_argument("--timeout", type=int, default=180)
    ap.add_argument("--jobs", type=int, default=4)
    ap.add_argument("--self-test", action="store_true",
                    help="compare gold against itself; every row must `agree` or "
                         "the harness is broken and no result may be reported")
    ap.add_argument("--out", default=None)
    args = ap.parse_args()

    verus = Path(args.verus_bin or "verus")
    specs = ROOT / "training-dataset" / "specs" / args.version
    preamble = read_preamble_text(args.version)
    gen = Path(args.gen_dir)

    cmds = args.commands or sorted(
        f.stem.upper() for f in gen.glob("*.rs"))

    tasks, meta = [], []
    for cmd in cmds:
        gold_p = specs / f"{cmd.lower()}_spec.rs"
        cand_p = (gold_p if args.self_test else gen / f"{cmd.lower()}.rs")
        sec = load_section(args.version, cmd)
        if not (gold_p.exists() and cand_p.exists() and sec):
            continue
        gold = parse_spec(gold_p.read_text(encoding="utf-8", errors="replace"))
        cand = parse_spec(cand_p.read_text(encoding="utf-8", errors="replace"))
        if not gold or not cand:
            continue
        if len(gold["params"]) != len(cand["params"]):
            meta.append({"command": cmd, "rows": [],
                         "note": "signature_mismatch -- not comparable"})
            continue
        cand_body = alpha_rename(cand["body"], cand["params"], gold["params"])

        doc = parse_section(sec)
        gcl = split_conjuncts("{" + gold["body"] + "}")
        ccl = split_conjuncts("{" + cand_body + "}")
        gids, cids = [idents(x) for x in gcl], [idents(x) for x in ccl]

        rows = []
        for table in ("failure", "success", "footprint"):
            for rid, cond in doc[table].items():
                ri = idents(cond)
                gc = clauses_for_row(gcl, gids, ri, args.min_overlap)
                cc = clauses_for_row(ccl, cids, ri, args.min_overlap)
                rows.append({"row": f"{table}:{rid}", "gold_n": len(gc),
                             "model_n": len(cc)})
                tasks.append((cmd, f"{table}:{rid}", gold["params_str"], gc, cc))
        meta.append({"command": cmd, "rows": rows})

    if not tasks:
        sys.exit("nothing to compare -- refusing to report")

    print(f"  {len(tasks)} document rows across {len(meta)} commands, "
          f"{args.jobs} jobs\n", flush=True)

    def run(t):
        cmd, row, ps, gc, cc = t
        v, _ = verdict_for_row(verus, preamble, ps, gc, cc, args.timeout)
        print(f"  {cmd:28s} {row:28s} {v}", flush=True)
        return {"command": cmd, "row": row, "verdict": v,
                "gold_clauses": len(gc), "model_clauses": len(cc)}

    with ThreadPoolExecutor(max_workers=args.jobs) as ex:
        results = list(ex.map(run, tasks))

    import collections
    c = collections.Counter(r["verdict"] for r in results)
    print("\n=== row verdicts")
    for k, v in c.most_common():
        print(f"  {v:4d}  {k}")

    if args.self_test:
        bad = [r for r in results
               if r["verdict"] not in ("agree", "neither_encodes")
               and not r["verdict"].startswith("unchecked")]
        print(f"\n  --- self-test: gold against itself ---")
        print(f"  rows that should agree and do not: {len(bad)}")
        for r in bad[:10]:
            print(f"    {r['command']} {r['row']} -> {r['verdict']}")
        print(f"\n  verdict: {'USABLE' if not bad else 'NOT USABLE -- fix first'}")

    strong = [r for r in results if r["verdict"] == "model_stronger"]
    if strong and not args.self_test:
        print(f"\n  {len(strong)} rows where the model forbids more than gold.")
        print("  These are CANDIDATES for gold being incomplete, in the shape of")
        print("  the one confirmed case. Each needs the document row read before")
        print("  it counts as anything:")
        for r in strong:
            print(f"    {r['command']:28s} {r['row']}")

    if args.out:
        Path(args.out).write_text(json.dumps(
            {"rows": results, "commands": meta}, indent=2))
        print(f"\n  wrote {args.out}")


if __name__ == "__main__":
    main()
