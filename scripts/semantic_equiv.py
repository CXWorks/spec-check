#!/usr/bin/env python3
"""Ask Z3 whether a generated spec means the same thing as gold.

    python scripts/semantic_equiv.py eval-new/sft2-2-final-bok8.json --out equiv.json

Compiling is not evidence of faithfulness — that is why nothing in this project
may be trained on compile-success — but "faithful to the PDF" is not directly
checkable either. What *is* checkable is agreement with gold, which was written
from the same PDF. That is a weaker claim than faithfulness and has to be read as
one: gold is a human reading of the text, not the text.

For each command it emits one of

  equivalent    both implications proved. Same meaning on every input.
  stronger      cand ==> gold only. The spec forbids things gold allows.
                Over-constrained: it can reject legal behaviour.
  weaker        gold ==> cand only. The spec ALLOWS things gold forbids.
                **This is the failure mode that compile-success cannot see** —
                the limit is `{ true }`, which compiles and permits everything.
  incomparable  neither direction proved. Disagrees in both directions.
  signature_mismatch / timeout / error  — reported, never silently bucketed as
                a disagreement.

The construction: preamble, gold renamed to `gold_spec`, candidate renamed to
`cand_spec` with its parameters alpha-renamed to gold's, then a `proof fn` whose
`ensures` is the implication under test. Verus unfolds both `open spec fn` bodies
and hands the obligation to Z3.
"""

import argparse
import json
import re
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))


def parse_spec(src):
    """(params, body) for the first `pub open spec fn ... -> bool { ... }`."""
    from verify_generated_verus import extract_fn_block, split_params
    name, params_str, text = extract_fn_block(src)
    if not name or not params_str or not text:
        return None
    body = text[text.find("{") + 1: text.rfind("}")]
    return {"name": name, "params": split_params(params_str),
            "params_str": params_str, "body": body}


def alpha_rename(body, src_params, dst_params):
    """Rename the candidate's parameters to gold's, positionally.

    Whole-word only: `rd` must not rewrite inside `rd_ptr`. Applied via a single
    simultaneous pass with placeholders, because sequential renames can collide
    when the two parameter lists are permutations of each other.
    """
    tmp = {}
    for i, (s, _) in enumerate(src_params):
        ph = f"__P{i}__"
        tmp[ph] = dst_params[i][0]
        body = re.sub(rf"\b{re.escape(s)}\b", ph, body)
    for ph, d in tmp.items():
        body = body.replace(ph, d)
    return body


def make_obligation(preamble, gold, cand_body, kind):
    args = ", ".join(n for n, _ in gold["params"])
    concl = {
        "equiv": "cand_spec({a}) == gold_spec({a})",
        "cand_implies_gold": "cand_spec({a}) ==> gold_spec({a})",
        "gold_implies_cand": "gold_spec({a}) ==> cand_spec({a})",
    }[kind].format(a=args)
    return (
        f"{preamble}\n\n"
        f"pub open spec fn gold_spec{gold['params_str']} -> bool {{{gold['body']}}}\n\n"
        f"pub open spec fn cand_spec{gold['params_str']} -> bool {{{cand_body}}}\n\n"
        f"proof fn check_{kind}{gold['params_str']}\n"
        f"    ensures {concl}\n"
        f"{{}}\n\n"
        "} // verus!\n"
    )


def run_verus(verus, src, timeout):
    """("proved" | "disproved" | "compile_error" | "timeout", output).

    The three failure kinds must stay separate. A file that does not build is not
    evidence that two specs disagree, but both come back as "not verified" — so
    collapsing them would report `incomparable` for a candidate whose parameters
    simply do not match, which is a harness result dressed up as a finding.
    """
    with tempfile.NamedTemporaryFile(mode="w", suffix=".rs", delete=False) as tf:
        tf.write(src)
        path = Path(tf.name)
    try:
        p = subprocess.run([str(verus), "--crate-type", "lib", str(path)],
                           capture_output=True, text=True, timeout=timeout)
        out = (p.stdout or "") + "\n" + (p.stderr or "")
        m = re.search(r"(\d+) verified, (\d+) errors", out)
        if m and int(m.group(2)) == 0:
            return "proved", out
        # Z3 reached the obligation and refuted it. Tested FIRST because Verus
        # also prints "aborting due to N previous errors" for a verification
        # failure — keying on that string alone mislabels 18 of 21 real
        # disagreements as harness breakage, which is precisely the conflation
        # this function's docstring exists to prevent.
        if "postcondition not satisfied" in out:
            return "disproved", out
        # rustc diagnostics carry an error code; verification failures do not
        if re.search(r"error\[E\d+\]", out):
            return "compile_error", out
        if m:                       # reached Z3, errors reported, none of them ours
            return "disproved", out
        return "compile_error", out
    except subprocess.TimeoutExpired:
        return "timeout", "timeout"
    finally:
        try:
            path.unlink(missing_ok=True)
        except Exception:
            pass


def classify(verus, preamble, gold_src, cand_src, timeout):
    gold, cand = parse_spec(gold_src), parse_spec(cand_src)
    if not gold:
        return {"verdict": "error", "detail": "gold unparseable"}
    if not cand:
        return {"verdict": "error", "detail": "candidate has no pub open spec fn"}
    if len(gold["params"]) != len(cand["params"]):
        return {"verdict": "signature_mismatch",
                "detail": f"{len(cand['params'])} params vs gold {len(gold['params'])}"}
    gt = [t for _, t in gold["params"]]
    ct = [t for _, t in cand["params"]]
    if gt != ct:
        # Types differ, so the two are not even asking about the same objects.
        # Reported rather than forced into a comparison that would be meaningless.
        return {"verdict": "signature_mismatch",
                "detail": f"types {ct} vs gold {gt}"}

    body = alpha_rename(cand["body"], cand["params"], gold["params"])
    fwd, fout = run_verus(verus, make_obligation(preamble, gold, body,
                                                 "cand_implies_gold"), timeout)
    bwd, bout = run_verus(verus, make_obligation(preamble, gold, body,
                                                 "gold_implies_cand"), timeout)
    base = {"cand_implies_gold": fwd, "gold_implies_cand": bwd}
    if "compile_error" in (fwd, bwd):
        err = next(l for o in (fout, bout) for l in o.splitlines()
                   if l.startswith("error")) if "error" in (fout + bout) else ""
        return {**base, "verdict": "compile_error", "detail": err[:160]}
    if "timeout" in (fwd, bwd):
        return {**base, "verdict": "timeout"}
    v = {("proved", "proved"): "equivalent", ("proved", "disproved"): "stronger",
         ("disproved", "proved"): "weaker",
         ("disproved", "disproved"): "incomparable"}[(fwd, bwd)]
    # A gold that is itself `true` makes "equivalent" vacuous: psci_cpu_off_spec
    # is literally `{ true }`, so anything at all is equivalent to it. Flagged so
    # such commands cannot be counted as faithfulness evidence.
    return {**base, "verdict": v, "gold_vacuous": gold["body"].strip() in ("true", "true,")}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("evals", nargs="+", help="eval JSONs with a `generated` field")
    ap.add_argument("--specs-dir", default=str(ROOT / "training-dataset/specs/alp14"))
    ap.add_argument("--out", default=None)
    ap.add_argument("--timeout", type=int, default=300)
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--only-passing", action="store_true",
                    help="Skip specs that do not compile. They cannot be a "
                         "faithfulness result, only a compile failure.")
    args = ap.parse_args()

    from verify_generated_verus import find_verus_bin, read_preamble
    verus = find_verus_bin(None)
    if not verus:
        sys.exit("verus not found — set VERUS_BIN")
    preamble = read_preamble(Path(args.specs_dir) / "preamble.rs")
    specs = Path(args.specs_dir)

    work = []
    for f in args.evals:
        run = Path(f).stem
        for r in json.loads(Path(f).read_text())["results"]:
            if args.only_passing and not r["pass"]:
                continue
            g = specs / f"{r['command'].lower()}_spec.rs"
            if not g.exists():
                continue
            work.append((run, r["command"], g.read_text(errors="ignore"), r["generated"]))
    if not work:
        sys.exit("nothing to check")
    print(f"[equiv] {len(work)} spec pairs, {args.jobs} parallel, "
          f"{args.timeout}s timeout each (2 Verus runs per pair)", flush=True)

    def one(t):
        run, cmd, g, c = t
        res = classify(verus, preamble, g, c, args.timeout)
        res.update(run=run, command=cmd)
        print(f"[equiv] {run} {cmd}: {res['verdict']}", flush=True)
        return res

    with ThreadPoolExecutor(max_workers=args.jobs) as pool:
        out = list(pool.map(one, work))

    import collections
    print("\n=== verdicts by run")
    for run in sorted({r["run"] for r in out}):
        c = collections.Counter(r["verdict"] for r in out if r["run"] == run)
        n = sum(c.values())
        print(f"  {run}: " + "  ".join(f"{k}={v}" for k, v in c.most_common()))
        weak = c["weaker"] + c["incomparable"]
        if n:
            print(f"     -> {weak}/{n} admit behaviour gold forbids "
                  f"({100*weak/n:.0f}%); equivalent {c['equivalent']}/{n}")
    if args.out:
        Path(args.out).write_text(json.dumps(out, indent=2))
        print(f"\n[equiv] wrote {args.out}")


if __name__ == "__main__":
    main()
