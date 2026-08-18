"""Locate the constraint a `weaker` spec dropped, by ablating gold one clause at a time.

Textual diffing cannot do this: every run "misses one clause" but no two miss the
same one, which is what reworded parenthesisation looks like rather than an
omission. Z3 can. If gold minus clause i becomes equivalent to the candidate,
clause i is exactly what the candidate failed to say.
"""
import json, re, sys, pathlib
sys.path.insert(0, "scripts"); sys.path.insert(0, "prompt_engineering")
import importlib.util
sp = importlib.util.spec_from_file_location("se", "scripts/semantic_equiv.py")
se = importlib.util.module_from_spec(sp); sp.loader.exec_module(se)
from verify_generated_verus import find_verus_bin, read_preamble
from concurrent.futures import ThreadPoolExecutor

verus = find_verus_bin(None)
pre = read_preamble(pathlib.Path("training-dataset/specs/alp14/preamble.rs"))
CMD, EVAL = sys.argv[1], sys.argv[2]

def top_clauses(body):
    out, depth, cur, i = [], 0, "", 0
    while i < len(body):
        ch = body[i]
        if ch in "([": depth += 1
        elif ch in ")]": depth -= 1
        if depth == 0 and body[i:i+2] == "&&":
            out.append(cur); cur = ""; i += 2; continue
        cur += ch; i += 1
    out.append(cur)
    return [c for c in out if c.strip()]

gold_src = pathlib.Path(f"training-dataset/specs/alp14/{CMD.lower()}_spec.rs").read_text()
gold = se.parse_spec(gold_src)
cand_src = next(r["generated"] for r in json.loads(pathlib.Path(EVAL).read_text())["results"]
                if r["command"] == CMD)
cand = se.parse_spec(cand_src)
body = se.alpha_rename(cand["body"], cand["params"], gold["params"])
cl = top_clauses(gold["body"])
print(f"{CMD}: gold has {len(cl)} top-level clauses")

def test(i):
    reduced = " && ".join(c for j, c in enumerate(cl) if j != i)
    g2 = dict(gold, body=reduced)
    f, _ = se.run_verus(verus, se.make_obligation(pre, g2, body, "cand_implies_gold"), 300)
    b, _ = se.run_verus(verus, se.make_obligation(pre, g2, body, "gold_implies_cand"), 300)
    return i, f, b

with ThreadPoolExecutor(max_workers=12) as p:
    for i, f, b in sorted(p.map(test, range(len(cl)))):
        if f == "proved" and b == "proved":
            print(f"  >>> dropping clause {i} makes gold EQUIVALENT to the model:")
            print(f"      {re.sub(r'[ \t]+',' ', cl[i]).strip()[:300]}")
        elif f == "proved":
            print(f"  clause {i}: removing it makes the model no longer weaker "
                  f"(still not equivalent) — {re.sub(r'[ \t]+',' ', cl[i]).strip()[:110]}")
