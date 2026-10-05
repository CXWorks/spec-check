#!/usr/bin/env python3
"""Writer x input x prompt, on the six unseen documents. Run from the repo root.

    python3 results/diag_format/scripts/compare_analyze.py

Restricted to the 310 commands whose RMM-layout rewrite passed its checks
(mirror_report.json, mirror4_report.json), so every row covers the same commands --
except the strict intermediate form, whose own rewrite dropped three more (307),
and rows run on PSCI and SDEI only (39).
`空洞` is a body that is literally `true`. `一致` is stub_sweep.py's `consistent`:
the spec compiles against declarations written for it (the spec itself unchanged)
and Z3 proves it neither unsatisfiable nor vacuous. A literal-`true` body counts
as vacuous there too. Rows without a sweep show `-`.
"""
import collections
import json
import re
from pathlib import Path

R = Path("results")
D = R / "diag_format"
C = D / "compare"
DOCS = ["psci_13", "sdei", "drtm", "ffa", "scmi", "sbi"]

kept = json.loads((D / "mirror" / "mirror_report.json").read_text())["kept"]
kept.update(json.loads((C / "inputs" / "mirror4_report.json").read_text())["kept"])
kept = {d: {c.upper() for c in v} for d, v in kept.items()}


def key(stem):
    m = re.match(r'^([\d.]+)_(.+)$', stem)          # six-document SCMI naming
    return f"{m.group(2).upper()}__{m.group(1).replace('.', '_')}" if m else stem.upper()


def body(s):
    i = s.find("{")
    return s[i + 1: s.rfind("}")] if i >= 0 else ""


def orig(model, d):
    return (R / "six_docs" / "gen_psci" / f"{model}-nopre" / "psci_13" if d == "psci_13"
            else R / "six_docs" / "gen_docs" / f"{d}-{model}-nopre" / d)


def rmm(model, d):
    if model == "ft" and d in ("psci_13", "sdei"):
        return D / "mirror" / "gen" / "rmm" / f"{d}_rmm"
    return C / "gen" / f"{model}-rmm" / f"{d}_rmm"


strict_kept = json.loads((C / "inputs_strict" / "strict_report.json").read_text())["kept"]
strict_kept = {d: kept[d] & {c.upper() for c in v} for d, v in strict_kept.items()}
only39 = lambda f: (lambda d: f(d) if d in ("psci_13", "sdei") else None)

ARMS = [  # label, dir(doc), sweep set prefix, kept override
    ("微调 9B · 原文 · v3.1", lambda d: orig("ft", d), None, None),
    ("微调 9B · 原文 · 允许新起 helper", lambda d: C / "gen" / "ft-invent" / d, "ft_invent", None),
    ("微调 9B · 原文 · 强调不许 true", only39(lambda d: C / "gen" / "ft-nevertrue" / d), None, None),
    ("微调 9B · 中间表示 · v3.1", lambda d: rmm("ft", d), "ft_rmm", None),
    ("微调 9B · 严格中间表示 · v3.1", lambda d: C / "gen" / "ft-strict" / f"{d}_strict", "ft_strict", strict_kept),
    ("原始 9B · 原文 · v3.1", lambda d: orig("base", d), None, None),
    ("原始 9B · 原文 · 允许新起 helper", lambda d: C / "gen" / "base-invent" / d, "base_invent", None),
    ("原始 9B · 中间表示 · v3.1", lambda d: rmm("base", d), "base_rmm", None),
    ("Claude · 原文 · v3.1", only39(lambda d: C / "gen" / "claude-orig" / d), None, None),
    ("Claude · 原文 · 允许新起 helper", lambda d: C / "gen" / "claude-invent" / d, "claude_invent", None),
    ("Claude · 中间表示 · v3.1", only39(lambda d: C / "gen" / "claude-orig" / f"{d}_rmm"), "claude_rmm", None),
]

sweeps = []
for p in [C / "sweep" / n / "final.json" for n in
          ("psci_sdei_9b", "psci_sdei_claude", "four_docs", "ft_invent", "ft_strict", "base_invent")]:
    if p.exists():
        sweeps += json.loads(p.read_text())
verdict = {}
for r in sweeps:
    parts = r["file"].split("/")
    s, v, f = parts[0], parts[1], parts[-1]
    verdict[(s, v.replace("_rmm", ""), key(Path(f).stem))] = r["verdict"]

print(f"{'':34s}" + "".join(f"{d:>11s}" for d in DOCS) + f"{'合计空洞':>14s}{'一致':>10s}{'unsat':>7s}{'非字面空洞':>8s}")
for label, fdir, sset, kov in ARMS:
    K = kov or kept
    cells, vt, nt, ok, un, swn, nv = [], 0, 0, 0, 0, 0, 0
    for d in DOCS:
        p = fdir(d)
        if p is None or not Path(p).exists():
            cells.append("-"); continue
        fs = {key(f.stem): f for f in Path(p).glob("*.rs")}
        fs = {k: f for k, f in fs.items() if k in K[d]}
        v = sum(body(f.read_text()).strip() == "true" for f in fs.values())
        cells.append(f"{v}/{len(fs)}"); vt += v; nt += len(fs)
        if sset:
            for k in fs:
                vd = verdict.get((sset, d, k))
                if vd is not None:
                    swn += 1; ok += vd == "consistent"; un += vd == "unsat"; nv += vd == "vacuous"
    tail = f"{vt}/{nt} ({100 * vt / nt:.0f}%)" if nt else "-"
    swc = f"{ok}/{swn}" if swn else "-"
    print(f"{label:34s}" + "".join(f"{c:>11s}" for c in cells) + f"{tail:>14s}{swc:>10s}{(un if swn else '-'):>7}{(nv if swn else '-'):>8}")
