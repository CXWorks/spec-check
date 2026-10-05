#!/usr/bin/env python3
"""Compile-check the diagnostic's generated specs against the alp14 preamble and
write one eval JSON per version, in the shape semantic_equiv.py reads. Runs in a
scratch cwd because verus drops lib<stem>.rlib next to the process cwd."""
import json, os, sys, tempfile
from pathlib import Path
from concurrent.futures import ThreadPoolExecutor

ROOT = Path.home() / "spec-check"
sys.path.insert(0, str(ROOT / "prompt_engineering"))
from verify_generated_verus import check_text, find_verus_bin, read_preamble

VERSIONS = sys.argv[1:] or ["alp14_ctl", "alp14_layout", "alp14_prose"]
GEN, OUT = Path.home() / "diag" / "gen", Path.home() / "diag"
verus = find_verus_bin(None)
assert verus, "set VERUS_BIN"
pre = read_preamble(ROOT / "training-dataset" / "specs" / "alp14" / "preamble.rs")
os.chdir(tempfile.mkdtemp(prefix="scorediag-"))

for v in VERSIONS:
    files = sorted((GEN / v).glob("*.rs"))
    def one(f):
        cmd = f.stem.upper()
        src = f.read_text()
        r = check_text(verus, pre, cmd, src, 120)
        return {"command": cmd, "pass": r.status == "pass",
                "reason": r.reason, "generated": src}
    with ThreadPoolExecutor(max_workers=24) as ex:
        res = list(ex.map(one, files))
    n = sum(r["pass"] for r in res)
    (OUT / f"eval-{v}.json").write_text(json.dumps(
        {"summary": {"arm": v, "n": len(res), "pass": n}, "results": res}, indent=1))
    print(f"{v:13s} compile {n}/{len(res)}", flush=True)
