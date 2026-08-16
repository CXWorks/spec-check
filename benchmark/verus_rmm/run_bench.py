#!/usr/bin/env python3
"""Score a generator's alp14 Verus specs against the machine-checked bug benchmark.

Each item pairs a command with a proof obligation. Two item shapes:

  kind = "obligation"     assert(P) under `requires <spec fn>(...)`.
                          The bug is present iff Verus REJECTS the assert -- the
                          command's own conditions fail to entail P.
  kind = "contradiction"  `ensures false` under `requires <spec fn>(...)` plus a
                          witness. The bug is present iff Verus ACCEPTS it -- the
                          spec is unsatisfiable at that witness.

An item is DETECTED when the generator's spec exhibits the labelled behaviour,
MISSED when it does not, and INCONCLUSIVE when the file will not compile or a
parameter the obligation needs is absent from the generated signature. Inconclusive
is never folded into missed.

Usage:
    python3 run_bench.py --gen-dir training-dataset/specs/alp14 --gen-pattern '{cmd}_spec.rs'
    python3 run_bench.py --gen-dir results/.../alp14 --label claude
"""
import argparse
import json
import os
import re
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
def preamble_path(version):
    return ROOT / "training-dataset" / "specs" / version / "preamble.rs"
VERUS = ROOT / "training" / "verus-x86-linux" / "verus"
LD = os.path.expanduser("~/.rustup/toolchains/1.97.1-x86_64-unknown-linux-gnu/lib")


def read_preamble(version="eac5"):
    """Preamble minus its closing `} // verus!`, with structs made pub.

    Field accesses inside `pub open spec fn` do not compile in a standalone crate
    unless the datatype is visible (STATUS.md lesson 5).
    """
    t = preamble_path(version).read_text()
    t = re.sub(r'(?m)^struct\b', 'pub struct', t)
    t = re.sub(r'\n\}\s*//\s*verus!\s*$', '', t.rstrip())
    return t


def split_signature_body(rs):
    """Return (params_str, body) for the first `pub open spec fn` item."""
    start = rs.find("pub open spec fn")
    if start < 0:
        return None, None
    lp = rs.find("(", start)
    depth, i = 0, lp
    while i < len(rs):
        if rs[i] == "(":
            depth += 1
        elif rs[i] == ")":
            depth -= 1
            if depth == 0:
                break
        i += 1
    params = rs[lp + 1:i]
    brace = rs.find("{", i)
    d, j = 0, brace
    while j < len(rs):
        if rs[j] == "{":
            d += 1
        elif rs[j] == "}":
            d -= 1
            if d == 0:
                break
        j += 1
    return params, rs[start:j + 1]


def param_names(params_str):
    out = []
    for p in re.split(r',(?![^<(]*[>)])', params_str):
        p = p.strip()
        if p and ":" in p:
            out.append(p.split(":")[0].strip())
    return out


def run_verus(text):
    with tempfile.NamedTemporaryFile("w", suffix=".rs", delete=False) as f:
        f.write(text)
        path = f.name
    try:
        r = subprocess.run([str(VERUS), "--crate-type=lib", path],
                           capture_output=True, text=True, timeout=900,
                           env=dict(os.environ, LD_LIBRARY_PATH=LD))
        out = r.stdout + r.stderr
        m = re.search(r'(\d+) verified, (\d+) errors', out)
        if not m:
            return None, out            # did not compile
        return int(m.group(2)) == 0, out  # True == proof accepted
    finally:
        os.unlink(path)


def score_item(item, gen_dir, gen_pattern, preamble):
    cmd = item["spec_fn"].replace("_spec", "")
    f = gen_dir / gen_pattern.format(cmd=cmd)
    if not f.exists():
        return "inconclusive", "no generated file"

    params, fn_src = split_signature_body(f.read_text())
    if fn_src is None:
        return "inconclusive", "no spec fn found"
    names = param_names(params)

    # every identifier the obligation mentions must exist in the signature
    needed = set()
    for txt in item["requires"] + item["lets"] + [item["assertion"]]:
        needed |= set(re.findall(r'\b[a-z_][a-z0-9_]*\b', txt))
    missing = [n for n in ("old_s", "new_s") if n not in names]
    for n in names:
        needed.discard(n)
    if missing:
        return "inconclusive", f"signature missing {missing}"

    call = f"{item['spec_fn']}({', '.join(names)})"
    reqs = [call] + [r for r in item["requires"] if not r.startswith(item["spec_fn"])]
    body = "\n".join(f"  {l}" for l in item["lets"])
    assertion = "assert(false);" if item["assertion"] == "false" else f"assert({item['assertion']});"
    ens = "\n  ensures false" if item["kind"] == "contradiction" else ""
    proof = (f"pub proof fn bench_item({params})\n  requires\n    "
             + ",\n    ".join(reqs) + ",\n" + (ens.strip() and ens + "\n" or "")
             + "{\n" + body + "\n  " + (assertion if item["kind"] == "obligation" else "") + "\n}\n")

    text = preamble + "\n\n" + fn_src + "\n\n" + proof + "\n} // verus!\n"
    accepted, out = run_verus(text)
    if accepted is None:
        first = next((l for l in out.split("\n") if l.startswith("error")), "compile error")
        return "inconclusive", first[:120]

    want = item["bug_present_when"]
    present = accepted if want == "proof_succeeds" else (not accepted)
    return ("detected" if present else "missed"), ("proof accepted" if accepted else "proof rejected")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", default="eac5")
    ap.add_argument("--ground-truth", type=Path, default=None)
    ap.add_argument("--gen-dir", type=Path, required=True)
    ap.add_argument("--gen-pattern", default="{cmd}/generated.formatted.rs")
    ap.add_argument("--label", default=None)
    ap.add_argument("--json-out", type=Path, default=None)
    args = ap.parse_args()

    gt = json.loads((args.ground_truth or
                     HERE / f"ground_truth_{args.version}.json").read_text())
    preamble = read_preamble(args.version)
    label = args.label or args.gen_dir.name

    rows, tally = [], {}
    for item in gt["items"]:
        outcome, why = score_item(item, args.gen_dir, args.gen_pattern, preamble)
        rows.append({**{k: item[k] for k in ("id", "label", "kind")},
                     "outcome": outcome, "detail": why})
        tally[(item["label"], outcome)] = tally.get((item["label"], outcome), 0) + 1
        print(f"  {item['label']:14s} {outcome:12s} {item['id']:52s} {why}")

    print(f"\n=== {label} ===")
    for lab in ("TP", "TP-BORDERLINE", "FP"):
        n = sum(v for (l, _), v in tally.items() if l == lab)
        if not n:
            continue
        det = tally.get((lab, "detected"), 0)
        inc = tally.get((lab, "inconclusive"), 0)
        verb = "recall" if lab.startswith("TP") else "false alarms"
        print(f"  {lab:14s} {verb}: {det}/{n}" + (f"   inconclusive: {inc}" if inc else ""))

    if args.json_out:
        args.json_out.write_text(json.dumps({"label": label, "rows": rows}, indent=2) + "\n")
        print(f"\nWrote {args.json_out}")


if __name__ == "__main__":
    main()
