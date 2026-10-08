#!/usr/bin/env python3
"""The gold testbed in one table. Run from the repo root.

    python3 results/diag_format/testbed/scripts/testbed_analyze.py

49 held-out alp14 commands, conditions rewritten as prose, every writer given the
full alp14 preamble and gold's signature. 正确 = Z3 proves the spec equivalent to
gold, excluding commands whose gold is itself vacuous; 宽松 = weaker than gold
(admits behaviour gold forbids). 一致 (when scores/sweep-*.json exist) = compiles
and Z3 finds it neither unsatisfiable nor vacuous, the gold-free measure used on
the unseen documents.
"""
import collections
import json
from pathlib import Path

T = Path("results/diag_format/testbed")
ARMS = [
    ("A", "Claude 直接写"),
    ("B", "Claude 写表 → 原始 9B"),
    ("C", "Claude 写表 → 微调 9B"),
    ("D", "原始 9B 写表 → 原始 9B"),
    ("E", "微调 9B 写表 → 微调 9B"),
    ("F", "原始 9B 直接写"),
    ("G", "微调 9B 直接写"),
    ("REF", "微调 9B 读 RMM 原文"),
]


def body(s):
    i = s.find("{")
    return s[i + 1: s.rfind("}")] if i >= 0 else ""


cols = ["编译", "正确", "宽松", "严格", "互不含", "签名错", "比不了", "空话", "一致", "unsat"]
print(f"{'':26s}" + "".join(f"{c:>7s}" for c in cols))
for a, label in ARMS:
    ev = json.loads((T / "scores" / f"eval-{a}.json").read_text())["results"]
    eq = json.loads((T / "scores" / f"equiv-{a}.json").read_text())
    v = collections.Counter(r["verdict"] for r in eq)
    correct = sum(r["verdict"] == "equivalent" and not r.get("gold_vacuous") for r in eq)
    vac = sum(body(f.read_text()).strip() == "true" for f in (T / "gen" / a).glob("*.rs"))
    sw = T / "scores" / f"sweep-{a}.json"
    cons = unsat = "-"
    if sw.exists():
        rows = json.loads(sw.read_text())
        rows = rows.get("rows", rows) if isinstance(rows, dict) else rows
        sv = collections.Counter(r.get("verdict") for r in rows)
        cons, unsat = sv.get("consistent", 0), sv.get("unsat", 0)
    cells = [sum(r["pass"] for r in ev), correct, v.get("weaker", 0), v.get("stronger", 0),
             v.get("incomparable", 0), v.get("signature_mismatch", 0), v.get("compile_error", 0),
             vac, cons, unsat]
    print(f"{a + ' ' + label:26s}" + "".join(f"{c:>7}" for c in cells))

print("\n条件表格式检查（通过 / 总数）:")
for name in ("alp14_ir_claude", "alp14_ir_base", "alp14_ir_ft"):
    res = {}
    for f in sorted((T / "tables").glob(f"{name}.check*.json")):
        res.update(json.loads(f.read_text()))
    print(f"  {name:16s} {sum(not r['problems'] for r in res.values())}/{len(res)}")
