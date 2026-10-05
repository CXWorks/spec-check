#!/usr/bin/env python3
"""Both diagnostic rounds in one table. Run from the repo root.
    python3 diag_analyze_all.py <pulled dir> <restyle dir>"""
import json, statistics, sys, collections
from pathlib import Path

D, R = Path(sys.argv[1]), Path(sys.argv[2])
GOLD = Path("training-dataset/specs/alp14")
ren = json.load(open(R / "restyle_section_report.json"))["renamed_commands"]
inv = {v: k for k, v in ren.items()}
kept = sorted(ren)

def body(s):
    i = s.find("{"); return s[i + 1: s.rfind("}")] if i >= 0 else ""

def top_clauses(b):
    out, d, cur, i = [], 0, "", 0
    while i < len(b):
        ch = b[i]
        if ch in "([": d += 1
        elif ch in ")]": d -= 1
        if d == 0 and b[i:i + 2] == "&&":
            out.append(cur); cur = ""; i += 2; continue
        cur += ch; i += 1
    out.append(cur); return [c for c in out if c.strip()]

ARMS = [("alp14_ctl", "对照 原文"), ("alp14_layout", "A 只改排版"), ("alp14_prose", "B 条件改成PSCI文风"),
        ("alp14_full", "D 全文PSCI化"), ("alp14_prose_ren", "C B+改名"), ("alp14_full_ren", "E D+改名")]
g = {c: (GOLD / f"{c.lower()}_spec.rs").read_text() for c in kept}
gv = [c for c in kept if body(g[c]).strip() == "true"]
gm = statistics.median(len(top_clauses(body(g[c]))) for c in kept if c not in gv)
print(f"{'':20s}{'n':>4s}{'空洞':>6s}{'子句中位':>9s}{'能编译':>7s}{'正确':>6s}{'weaker':>8s}   空洞的命令")
print(f"{'人工标准答案':20s}{len(kept):>4}{len(gv):>6}{gm:>9}{'-':>7}{'-':>6}{'-':>8}   {gv}")
vac_sets = {}
for v, label in ARMS:
    srcs = {}
    for f in (D / "gen" / v).glob("*.rs"):
        c = f.stem.upper(); c = inv.get(c, c)
        srcs[c] = f.read_text()
    srcs = {c: s for c, s in srcs.items() if c in kept}
    vac = sorted(c for c, s in srcs.items() if body(s).strip() == "true")
    vac_sets[v] = set(vac)
    cl = [len(top_clauses(body(s))) for c, s in srcs.items() if c not in vac]
    med = statistics.median(cl) if cl else 0
    comp = cor = wk = "-"
    ev, eq = D / f"eval-{v}.json", D / f"equiv-{v}.json"
    if ev.exists() and eq.exists():
        comp = sum(r["pass"] for r in json.load(open(ev))["results"])
        e = json.load(open(eq))
        cor = sum(r["verdict"] == "equivalent" and not r.get("gold_vacuous") for r in e)
        wk = sum(r["verdict"] == "weaker" for r in e)
    print(f"{label:20s}{len(srcs):>4}{len(vac):>6}{med:>9}{comp:>7}{cor:>6}{wk:>8}   {vac[:6]}{' …' if len(vac) > 6 else ''}")
