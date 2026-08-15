#!/usr/bin/env python3
"""Toolchain smoke test: compile gold specs through the real eval code path.

Gate G1. If this fails, nothing downstream can be measured, so run it before
submitting any training job.

    VERUS_BIN=/work/tools/verus/verus-x86-linux/verus \
    python scripts/verus_smoke.py --specs-dir training-dataset/specs/alp14 -n 5

Gold specs are SCOPE output and should compile. A failure here is an
environment problem, not a model problem — which is exactly what this
separates.
"""

import argparse
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))

from verify_generated_verus import check_text, find_verus_bin, read_preamble  # noqa: E402


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--specs-dir", default=str(ROOT / "training-dataset/specs/alp14"))
    ap.add_argument("--verus", default=None)
    ap.add_argument("-n", type=int, default=5, help="how many gold specs to try")
    ap.add_argument("--timeout", type=int, default=600)
    args = ap.parse_args()

    verus = find_verus_bin(args.verus)
    if not verus:
        sys.exit("verus not found — set VERUS_BIN or pass --verus")
    print(f"verus: {verus}")

    specs = Path(args.specs_dir)
    # read_preamble, not read_text: it rewrites `struct` to `pub struct`, without
    # which field accesses in a pub open spec fn fail to typecheck in a
    # standalone crate. Skipping it looks like a spec bug and is not one.
    preamble = read_preamble(specs / "preamble.rs")
    print(f"preamble: {len(preamble.splitlines())} lines")

    # macOS tar emits AppleDouble sidecars (._foo.rs) that survive kubectl cp and
    # sort ahead of real files. Use COPYFILE_DISABLE=1 when tarring; skip them here.
    cands = sorted(p for p in specs.glob("*_spec.rs")
                   if not p.name.startswith("._"))[: args.n]
    if not cands:
        sys.exit(f"no *_spec.rs under {specs}")

    npass = 0
    for p in cands:
        cmd = p.stem.replace("_spec", "")
        r = check_text(verus, preamble, cmd, p.read_text(), args.timeout)
        ok = r.status == "pass"
        npass += ok
        detail = "" if ok else f"  reason={r.reason}  {(r.output_head or '')[:150]}"
        print(f"  [{'PASS' if ok else 'FAIL'}] {cmd}{detail}")

    print(f"\n{npass}/{len(cands)} gold specs compile")
    # Gold is SCOPE output; anything less than all of them means the toolchain is
    # wrong, not the specs.
    sys.exit(0 if npass == len(cands) else 1)


if __name__ == "__main__":
    main()
