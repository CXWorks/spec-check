#!/usr/bin/env python3
"""Make specs written against an invented vocabulary checkable by Verus and Z3.

    # 1. local, needs the claude CLI: one stub preamble per spec
    python3 scripts/stub_sweep.py stub   --specs DIR --stubs DIR [--jobs 8]
    # 2. where Verus is: compile, then the unsat / vacuous obligations
    python3 scripts/stub_sweep.py sweep  --specs DIR --stubs DIR --out R.json
    # 3. local again: rewrite the stub (never the spec) from the compiler error
    python3 scripts/stub_sweep.py repair --specs DIR --stubs DIR --results R.json

A spec generated from an intermediate form calls helpers the intermediate form
made up (IsValidMpidr, CoreAt) and that no preamble declares, so it cannot be
compiled as it stands and nothing about it can be proved. This writes, for each
spec, the declarations it needs -- types, enum variants, constants, and bodyless
`pub open spec fn` helpers, the same style as the alp14 preamble -- and then runs
psci_sweep.py's two obligations against it:

    unsat    requires SPEC ensures false   verifies -> no input satisfies it
    vacuous  ensures SPEC                  verifies -> every input satisfies it

What "compiles" means here: there EXISTS a set of declarations under which the
spec type-checks. The spec text is never changed; a stub repair may only change
declarations. A spec that is syntactically not Verus (an English quantifier, say)
stays a compile error whatever the stub says. A body that is literally `true` is
recorded as vacuous without spending a call.
"""
import argparse
import json
import re
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))

STUB_SYSTEM = ("You write Verus declarations. Output only Verus source code: no prose, "
               "no code fence.")

STUB = """\
Below is a Verus specification function. Write a complete Verus preamble that
declares everything it needs to type-check:

    use vstd::prelude::*;
    verus! {{
    ... declarations ...
    }} // verus!

- Declare every type, enum (with every variant the function uses), constant and
  function it references that is not a Verus or vstd built-in.
- Functions: bodyless `pub open spec fn Name(args) -> Type;` with argument and
  return types consistent with EVERY use in the function.
- Types the parameters use (e.g. `pub type UInt64 = u64;`), the state struct
  `S` with every field the function accesses, and result types. If the function
  calls methods on a type (`.is_Ok()`), choose a declaration that has them.
- Give constants of the same type pairwise DISTINCT values: two return codes
  must never be equal, or a contradiction between them would go unnoticed.
- Do NOT include the function itself. Do NOT give any helper a body.

Function:
{spec}
"""

REPAIR = """\
This Verus preamble is meant to declare everything the function below needs, but
compiling the two together fails. Fix the PREAMBLE so they compile. You may add,
remove or change declarations only; the function must stay exactly as it is, and
no helper may get a body. If the error is in the function's own syntax and no
declaration can fix it, return the preamble unchanged.

Function:
{spec}

Preamble:
{pre}

Compiler error:
{err}

Output the full corrected preamble only.
"""


def body_of(src):
    i = src.find("{")
    return src[i + 1: src.rfind("}")] if i >= 0 else ""


def specs_in(d):
    return sorted(Path(d).rglob("*.rs"))


def stub_path(stubs, spec_root, f):
    return Path(stubs) / f.relative_to(spec_root).with_suffix(".preamble.rs")


def claude(system, user, effort="medium"):
    from confab_probe import call_claude
    out = call_claude(system, user, "claude-opus-5-5", effort)
    return re.sub(r'^```\w*\n|\n```\s*$', '', out.strip()) + "\n"


def cmd_stub(a):
    todo = [f for f in specs_in(a.specs)
            if body_of(f.read_text()).strip() != "true"
            and not stub_path(a.stubs, Path(a.specs), f).exists()]

    def one(f):
        p = stub_path(a.stubs, Path(a.specs), f)
        p.parent.mkdir(parents=True, exist_ok=True)
        try:
            p.write_text(claude(STUB_SYSTEM, STUB.format(spec=f.read_text())))
            return "ok"
        except Exception as e:  # noqa: BLE001
            return f"failed {f}: {e}"

    with ThreadPoolExecutor(a.jobs) as ex:
        res = list(ex.map(one, todo))
    print(f"[stub] {sum(r == 'ok' for r in res)}/{len(todo)} written", flush=True)
    for r in res:
        if r != "ok":
            print("[stub]", r, flush=True)


def error_head(out, n=16):
    lines = out.split("\n")
    for i, l in enumerate(lines):
        if l.startswith("error"):
            return "\n".join(lines[i:i + n])
    return out[-1200:]


def parse_balanced(src):
    """`pub open spec fn NAME(<params>) -> bool {...}` by bracket matching.

    psci_sweep's signature regex refuses `;` and braces inside the parameter
    list, so a legal array parameter `[UInt32; 6]` made 53 fine-tuned specs read
    as "no spec function". Here the parameter list is whatever sits inside the
    balanced parentheses; if it is not valid Verus, compiling says so.
    """
    from psci_sweep import extract_fn_body, is_trivial, split_params
    fns = []
    for m in re.finditer(r'pub\s+open\s+spec\s+fn\s+(\w+)\s*\(', src):
        i, depth = m.end() - 1, 0
        for j in range(i, len(src)):
            depth += {"(": 1, ")": -1}.get(src[j], 0)
            if depth == 0:
                break
        else:
            continue
        rest = re.match(r'\s*->\s*bool\s*\{', src[j + 1:])
        if not rest:
            continue
        params = src[i:j + 1]
        body = extract_fn_body(src, j + 1 + rest.end() - 1)
        fns.append({"name": m.group(1), "params_str": params,
                    "params": split_params(params[1:-1]), "body": body,
                    "trivial": is_trivial(body)})
    return fns


def cmd_sweep(a):
    from psci_sweep import build_case, classify, parse_spec_fns, run_verus
    from verify_generated_verus import find_verus_bin
    verus = find_verus_bin(None)
    assert verus, "set VERUS_BIN"
    root = Path(a.specs)

    def one(f):
        rel = str(f.relative_to(root))
        src = f.read_text(errors="replace")
        if body_of(src).strip() == "true":
            return {"file": rel, "verdict": "vacuous_literal"}
        sp = stub_path(a.stubs, root, f)
        if not sp.exists():
            return {"file": rel, "verdict": "no_stub"}
        found = parse_spec_fns(src) or parse_balanced(src)
        fns = [x for x in found if x["name"].endswith("_spec")] or found
        if not fns:
            return {"file": rel, "verdict": "no_spec_fn"}
        fn, pre, row = fns[0], sp.read_text(), {"file": rel}
        with tempfile.TemporaryDirectory() as td:
            for kind in ("unsat", "vacuous"):
                p = Path(td) / f"{re.sub(r'[^A-Za-z0-9_]', '_', f.stem)}_{kind}.rs"
                p.write_text(build_case(pre, fn, kind))
                rc, out = run_verus(verus, p, a.timeout)
                row[kind], detail = classify(rc, out)
                if row[kind] == "compile_error" and "error" not in row:
                    row["error"] = error_head(out)
        if row["unsat"] == "proved":
            row["verdict"] = "unsat"
        elif row["vacuous"] == "proved":
            row["verdict"] = "vacuous"
        elif "compile_error" in (row["unsat"], row["vacuous"]):
            row["verdict"] = "compile_error"
        elif "timeout" in (row["unsat"], row["vacuous"]):
            row["verdict"] = "timeout"
        elif "toolchain_error" in (row["unsat"], row["vacuous"]):
            row["verdict"] = "toolchain_error"
        else:
            row["verdict"] = "consistent"
        return row

    with ThreadPoolExecutor(a.jobs) as ex:
        rows = list(ex.map(one, specs_in(a.specs)))
    Path(a.out).write_text(json.dumps(rows, indent=1))
    from collections import Counter
    print(f"[sweep] {a.specs}: {dict(Counter(r['verdict'] for r in rows))}", flush=True)


def cmd_repair(a):
    rows = json.loads(Path(a.results).read_text())
    root = Path(a.specs)
    bad = [r for r in rows if r["verdict"] == "compile_error" and r.get("error")]

    def one(r):
        f = root / r["file"]
        sp = stub_path(a.stubs, root, f)
        new = claude(STUB_SYSTEM, REPAIR.format(spec=f.read_text(), pre=sp.read_text(),
                                                err=r["error"]))
        sp.write_text(new)
        return r["file"]

    with ThreadPoolExecutor(a.jobs) as ex:
        done = list(ex.map(one, bad))
    print(f"[repair] rewrote {len(done)} stubs", flush=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=["stub", "sweep", "repair"])
    ap.add_argument("--specs", required=True)
    ap.add_argument("--stubs", required=True)
    ap.add_argument("--out")
    ap.add_argument("--results")
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--timeout", type=int, default=180)
    a = ap.parse_args()
    {"stub": cmd_stub, "sweep": cmd_sweep, "repair": cmd_repair}[a.cmd](a)


if __name__ == "__main__":
    main()
