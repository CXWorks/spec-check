#!/usr/bin/env python3
"""Verus-feedback repair pass for benchmark items whose generated spec will not compile.

Scoring a generator on this benchmark requires its spec function to type-check. When
it does not, the item is `inconclusive` -- not a detection failure, just unscorable.
This pass feeds the real Verus error back to the same model and asks it to fix its
own output, which is the project's existing repair-loop idea (repair_loop_verus.py)
with the Qwen backend swapped for a CLI model.

Repaired output goes to a SEPARATE directory so the raw generator numbers stay intact;
"model + repair" is a different configuration and is reported as its own row.

Usage:
    python3 repair.py --model codex  --src results/baseline1_general/gpt56sol/eac5
    python3 repair.py --model claude --src results/baseline1_general/claude_opus5/eac5
"""
import argparse
import re
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT / "prompt_engineering"))

import run_bench as B                                          # noqa: E402
from cli_models import MODELS, QuotaExhausted                   # noqa: E402
from prompt_engineering import normalize_verus_with_verusfmt    # noqa: E402

SYSTEM = (
    "You fix Verus specification functions that fail to compile. You are given one "
    "`pub open spec fn`, the Verus error it produces, and the relevant declarations "
    "from the preamble.\n"
    "Rules:\n"
    "- Output ONLY the corrected function item. No markdown fences, no prose.\n"
    "- Fix ONLY what the error reports. Do not add, remove or weaken any logical "
    "condition, and do not change the parameter list.\n"
    "- Enum variants that carry a payload must be applied to an argument "
    "(e.g. `RMI_ERROR_RTT(level as int)`, never a bare `RMI_ERROR_RTT`).\n"
    "- Verus needs explicit `as int` at boundaries where a helper declares `int`."
)


def first_error_block(out, n=14):
    """Slice from the first real error, skipping the preamble's warning noise."""
    lines = out.split("\n")
    for i, l in enumerate(lines):
        if l.startswith("error"):
            return "\n".join(lines[i:i + n])
    return out[-800:]


def preamble_decls(err, preamble):
    """Quote the preamble's declaration of every identifier the error names."""
    names = set(re.findall(r'`([A-Za-z_][A-Za-z0-9_]*)`', err))
    out = []
    for n in names:
        for m in re.finditer(rf'(?m)^.*\b{re.escape(n)}\b.*$', preamble):
            line = m.group(0).strip()
            if any(k in line for k in ("spec fn", "enum", "struct", "const")) and len(line) < 200:
                out.append(line)
                break
    return "\n".join(sorted(set(out))[:12])


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--model", choices=sorted(MODELS), required=True)
    ap.add_argument("--src", type=Path, required=True)
    ap.add_argument("--out", type=Path, default=None)
    ap.add_argument("--version", default="eac5")
    ap.add_argument("--rounds", type=int, default=2)
    ap.add_argument("--only", nargs="*", default=None, help="limit to these commands")
    args = ap.parse_args()

    out_root = args.out or ROOT / "results" / "verus_repair" / args.src.parent.name / args.version
    preamble = B.read_preamble(args.version)
    model = MODELS[args.model](timeout=600, log_path=out_root / "calls.jsonl")
    print(f"[info] {model.name}  src={args.src}  out={out_root}")

    cmds = args.only or sorted(p.name for p in args.src.iterdir() if p.is_dir())
    fixed = failed = skipped = 0

    for cmd in cmds:
        f = args.src / cmd / "generated.formatted.rs"
        if not f.exists():
            continue
        _, fn = B.split_signature_body(f.read_text())
        ok, out = B.run_verus(preamble + "\n\n" + fn + "\n\n} // verus!\n")
        if ok is not None:
            skipped += 1
            continue                                   # already compiles, nothing to repair

        for rnd in range(1, args.rounds + 1):
            err = first_error_block(out)
            user = (f"## Verus error\n```\n{err}\n```\n\n"
                    f"## Relevant preamble declarations\n```rust\n{preamble_decls(err, preamble)}\n```\n\n"
                    f"## Function to fix\n```rust\n{fn}\n```\n\n"
                    "Return the corrected function only.")
            try:
                raw = model.generate([{"role": "system", "content": SYSTEM},
                                      {"role": "user", "content": user}])
            except QuotaExhausted as e:
                print(f"\n!! {e}\n!! Re-run the same command to resume; finished work is kept.")
                sys.exit(2)
            cand = normalize_verus_with_verusfmt(raw)
            _, cand_fn = B.split_signature_body(cand)
            if not cand_fn:
                continue
            ok, out = B.run_verus(preamble + "\n\n" + cand_fn + "\n\n} // verus!\n")
            fn = cand_fn
            if ok is not None:
                break

        d = out_root / cmd
        d.mkdir(parents=True, exist_ok=True)
        (d / "generated.formatted.rs").write_text(fn)
        if ok is not None:
            fixed += 1
            print(f"  [fixed]  {cmd}")
        else:
            failed += 1
            print(f"  [failed] {cmd}: {first_error_block(out, 1)[:80]}")

    print(f"\n{model.name}: fixed {fixed}, still failing {failed}, already compiling {skipped}")


if __name__ == "__main__":
    main()
