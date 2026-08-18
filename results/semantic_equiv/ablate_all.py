"""Run the clause ablation over every `weaker` verdict and classify what was dropped.

Two hand-picked examples both turned out to be frame conditions. This turns that
into a rate: for each weaker spec, find the clause whose removal stops it being
weaker, then ask whether that clause is a frame condition — an equality relating
old_s and new_s, i.e. "this part of the state does not change".
"""
import json, re, sys, pathlib, collections
sys.path.insert(0, "prompt_engineering")
import importlib.util
sp = importlib.util.spec_from_file_location("se", "scripts/semantic_equiv.py")
se = importlib.util.module_from_spec(sp); sp.loader.exec_module(se)
from verify_generated_verus import find_verus_bin, read_preamble
from concurrent.futures import ThreadPoolExecutor

verus = find_verus_bin(None)
pre = read_preamble(pathlib.Path("training-dataset/specs/alp14/preamble.rs"))

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

def is_frame(c):
    """An equality that relates new_s back to old_s: a does-not-change clause."""
    return bool(re.search(r"new_s", c) and re.search(r"old_s", c) and "==" in c)

targets = []
for f in sys.argv[1:]:
    for r in json.loads(pathlib.Path(f).read_text()):
        if r["verdict"] == "weaker":
            targets.append((r["run"], r["command"]))
print(f"{len(targets)} weaker verdicts to localise")

evals = {p.stem: p for p in pathlib.Path("eval-new").glob("*.json")}
def locate(t):
    run, cmd = t
    ev = evals.get(run)
    if not ev:
        return (run, cmd, None, "no eval file")
    gsrc = pathlib.Path(f"training-dataset/specs/alp14/{cmd.lower()}_spec.rs")
    if not gsrc.exists():
        return (run, cmd, None, "no gold")
    gold = se.parse_spec(gsrc.read_text())
    cand_src = next((r["generated"] for r in json.loads(ev.read_text())["results"]
                     if r["command"] == cmd), None)
    cand = se.parse_spec(cand_src) if cand_src else None
    if not cand or len(cand["params"]) != len(gold["params"]):
        return (run, cmd, None, "unparseable")
    body = se.alpha_rename(cand["body"], cand["params"], gold["params"])
    cl = top_clauses(gold["body"])
    for i, c in enumerate(cl):
        g2 = dict(gold, body=" && ".join(x for j, x in enumerate(cl) if j != i))
        f, _ = se.run_verus(verus, se.make_obligation(pre, g2, body, "cand_implies_gold"), 240)
        if f == "proved":
            return (run, cmd, c, "frame" if is_frame(c) else "other")
    return (run, cmd, None, "no single clause")

with ThreadPoolExecutor(max_workers=10) as p:
    res = list(p.map(locate, targets))

c = collections.Counter(k for _, _, _, k in res)
print("\n=== what the weaker specs dropped:", dict(c))
for run, cmd, cl, kind in res:
    tag = {"frame": "FRAME COND", "other": "other"}.get(kind, kind)
    print(f"  {cmd:30s} {run.split('-final')[0]:14s} {tag}")
    if cl and kind == "other":
        print(f"      {re.sub(r'[ \t]+',' ', cl).strip()[:120]}")
