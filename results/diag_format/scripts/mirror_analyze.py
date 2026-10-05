#!/usr/bin/env python3
"""Summarise the mirror experiment. Run from the repo root.
    python3 mirror_analyze.py <pulled gen2 dir> <mirror dir>"""
import json, re, statistics, sys, collections
from pathlib import Path

G, M = Path(sys.argv[1]), Path(sys.argv[2])
kept = json.load(open(M / "mirror_report.json"))["kept"]
PREV = {"psci_13": Path("results/six_docs/gen_psci/ft-nopre/psci_13"),
        "sdei": Path("results/six_docs/gen_docs/sdei-ft-nopre/sdei")}

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

def repeated(s):
    c = collections.Counter(x.strip() for x in s.splitlines() if len(x.strip()) > 20)
    return bool(c) and c.most_common(1)[0][1] >= 4

print(f"{'':16s}{'n':>4s}{'空洞':>8s}{'子句中位':>9s}{'有复读':>7s}{'用到文档返回码':>12s}{'覆盖本函数返回码':>14s}")
for doc in ("psci_13", "sdei"):
    ret = json.load(open(f"results/returns/returns-{doc}.json"))
    vocab, own = ret["vocab"], ret["commands"]
    for tag, v in (("ctl", doc), ("clean", f"{doc}_clean"), ("rmm", f"{doc}_rmm")):
        srcs = {f.stem.upper(): f.read_text() for f in (G / tag / v).glob("*.rs")}
        srcs = {c: s for c, s in srcs.items() if c in kept[doc]}
        vac = [c for c, s in srcs.items() if body(s).strip() == "true"]
        cl = [len(top_clauses(body(s))) for c, s in srcs.items() if c not in vac]
        uses = sum(1 for s in srcs.values() if any(re.search(rf'\b\w*{w}\b', s) for w in vocab if w != "SUCCESS"))
        cov_num = cov_den = 0
        for c, s in srcs.items():
            o = own.get(c, {})
            if o.get("weak_source", True):
                continue
            want = [w for w in o["codes"] if w != "SUCCESS"]
            cov_den += len(want); cov_num += sum(1 for w in want if re.search(rf'\b\w*{w}\b', s))
        print(f"{doc + ' ' + tag:16s}{len(srcs):>4}{f'{len(vac)} ({100*len(vac)/len(srcs):.0f}%)':>10}"
              f"{(statistics.median(cl) if cl else 0):>8}{sum(map(repeated, srcs.values())):>7}"
              f"{uses:>10}/{len(srcs):<3}{cov_num:>10}/{cov_den}")
        if tag == "ctl":
            same = sum(1 for c, s in srcs.items()
                       if (PREV[doc] / f"{c.lower()}.rs").exists() and (PREV[doc] / f"{c.lower()}.rs").read_text() == s)
            pv = [c for c in srcs if (PREV[doc] / f"{c.lower()}.rs").exists()
                  and body((PREV[doc] / f"{c.lower()}.rs").read_text()).strip() == "true"]
            print(f"{'':16s}  (与六份文档那次的输出逐字节相同 {same}/{len(srcs)}；那次空洞 {len(pv)}/{len(srcs)})")
