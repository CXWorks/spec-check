#!/usr/bin/env python3
"""Recover failure-condition ordering from the spec's *diagram*, not just its text.

Each `Failure condition ordering` section states a few `[a] < [b, c]` relations and
then draws a diagram placing every condition in horizontal tiers, each tier labelled
with the error code it yields. The diagram implies a total order between tiers; the
textual relations frequently state only part of it.

That gap is the defect behind `rmm_bugs.rs` bug 4: for RMI_PDEV_STOP the diagram puts
`pdev_align` (tier 2) above `pdev_state` (tier 3), but no textual relation orders them,
so the specification never resolves which error a caller sees when both hold.

`pdftotext -layout` flattens the tiers into indistinguishable lines. `-bbox-layout`
keeps word coordinates, so tiers are recoverable by clustering on yMin.

Usage:
    python3 diagram_ordering.py --pdf <rmm.pdf> --command RMI_PDEV_STOP
    python3 diagram_ordering.py --pdf <rmm.pdf> --all --json-out gaps.json
"""
import argparse
import json
import re
import subprocess
import tempfile
from pathlib import Path

WORD = re.compile(r'<word xMin="([\d.]+)" yMin="([\d.]+)" xMax="([\d.]+)" yMax="([\d.]+)">([^<]*)</word>')
ORDER_LINE    = re.compile(r'\[([^\]]+)\]\s*<\s*\[([^\]]+)\]', re.S)
COND_ID = re.compile(r'^[a-z][a-z0-9_]*$')
TIER_TOL = 6.0          # points; words within this y-distance are one tier


def page_text(pdf, lo=1, hi=2000):
    return subprocess.run(['pdftotext', '-layout', '-f', str(lo), '-l', str(hi), str(pdf), '-'],
                          capture_output=True, text=True).stdout.split('\f')


def find_pages(pages, command):
    """Pages holding this command's `Failure condition ordering` section."""
    sec = None
    for i, p in enumerate(pages, 1):
        m = re.search(rf'(B\d+\.\d+\.\d+)\s+{re.escape(command)} command', p)
        if m:
            sec = m.group(1)
        if sec and f'{sec}.2.1' in p and 'Failure condition ordering' in p:
            return i, sec, p
    return None, None, None


def words_on_page(pdf, page):
    with tempfile.NamedTemporaryFile(suffix='.xml') as f:
        subprocess.run(['pdftotext', '-bbox-layout', '-f', str(page), '-l', str(page),
                        str(pdf), f.name], capture_output=True)
        t = Path(f.name).read_text(errors='replace')
    return [(float(m.group(2)), float(m.group(1)), m.group(5)) for m in WORD.finditer(t)]


def diagram_band(words, sec):
    """y-range of the ordering diagram: below the `<sec>.2.1` heading, above the next.

    Condition ids also appear in the failure-conditions table on the same page (one
    per row), which would otherwise be clustered as a stack of single-element tiers.
    """
    top = bot = None
    for y, x, w in sorted(words):
        if w.startswith(f'{sec}.2.1'):
            top = y
        elif top is not None and re.match(rf'{re.escape(sec)}\.[3-9]', w):
            bot = y
            break
    return top, (bot if bot is not None else 1e9)


def tiers_from_diagram(words, ids, band=(None, 1e9)):
    """Cluster condition-id words into tiers by y, returning tiers top to bottom."""
    lo, hi = band
    if lo is not None:
        words = [(y, x, w) for y, x, w in words if lo < y < hi]
    sel = sorted((y, x, w) for y, x, w in words if w in ids)
    tiers, cur, cur_y = [], [], None
    for y, x, w in sel:
        if cur_y is None or abs(y - cur_y) <= TIER_TOL:
            cur.append(w); cur_y = y if cur_y is None else cur_y
        else:
            tiers.append(sorted(set(cur))); cur, cur_y = [w], y
    if cur:
        tiers.append(sorted(set(cur)))
    return tiers


def textual_closure(section_text):
    edges = set()
    for m in ORDER_LINE.finditer(section_text):
        lo = [x.strip() for x in m.group(1).split(',')]
        hi = [x.strip() for x in m.group(2).split(',')]
        edges |= {(a, b) for a in lo for b in hi}
    changed = True
    while changed:
        changed = False
        for a, b in list(edges):
            for c, d in list(edges):
                if b == c and (a, d) not in edges:
                    edges.add((a, d)); changed = True
    return edges


def analyse(pdf, command, pages, ids):
    page, sec, ptext = find_pages(pages, command)
    if not page:
        return None
    ws = words_on_page(pdf, page)
    tiers = tiers_from_diagram(ws, ids, diagram_band(ws, sec))
    if len(tiers) < 2:
        return {"command": command, "page": page, "tiers": tiers, "missing": [],
                "note": "no multi-tier diagram found"}
    text_edges = textual_closure(ptext)
    missing = []
    for i, upper in enumerate(tiers):
        for lower in tiers[i + 1:]:
            for a in upper:
                for b in lower:
                    if (a, b) not in text_edges:
                        missing.append([a, b])
    return {"command": command, "page": page, "tiers": tiers,
            "textual_edges": len(text_edges), "missing": missing}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--pdf', type=Path, required=True)
    ap.add_argument('--command', action='append')
    ap.add_argument('--all', action='store_true')
    ap.add_argument('--sections', type=Path,
                    default=Path(__file__).resolve().parents[3] / 'training-dataset/sections/alp14')
    ap.add_argument('--json-out', type=Path)
    args = ap.parse_args()

    pages = page_text(args.pdf)
    cmds = args.command or []
    if args.all:
        cmds = sorted(p.name[:-12] for p in args.sections.glob('*_command.txt'))

    out = []
    for c in cmds:
        ids = set()
        f = args.sections / f'{c}_command.txt'
        if f.exists():
            for m in re.finditer(r'^\s{2,}([a-z][a-z0-9_]*)\s+pre:', f.read_text(), re.M):
                ids.add(m.group(1))
        if not ids:
            continue
        r = analyse(args.pdf, c, pages, ids)
        if r and r['missing']:
            out.append(r)
            print(f"{c:34s} p{r['page']:<4d} tiers={[len(t) for t in r['tiers']]} "
                  f"text_edges={r['textual_edges']:3d} MISSING={len(r['missing'])}")
            for a, b in r['missing'][:4]:
                print(f"      {a} < {b}  (implied by diagram, absent from text)")

    print(f"\ncommands with a text/diagram ordering gap: {len(out)}")
    if args.json_out:
        args.json_out.write_text(json.dumps(out, indent=2) + '\n')
        print(f"Wrote {args.json_out}")


if __name__ == '__main__':
    main()
