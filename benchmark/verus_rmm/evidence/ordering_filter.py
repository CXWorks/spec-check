import re, json, sys
from pathlib import Path
SEC = Path('/mnt/sdc/xiang/spec-check/training-dataset/sections/alp14')

ID_LINE  = re.compile(r'^\s{2,}([a-z][a-z0-9_]*)\s+pre:\s*(.*)$')
POST     = re.compile(r'^\s+post:\s*(?:ResultEqual\(\s*result\s*,\s*([A-Z][A-Z0-9_]*)|result\s*==\s*([A-Z][A-Z0-9_]*))')
ORDER    = re.compile(r'^\s*\[([^\]]+)\]\s*<\s*\[([^\]]+)\]\s*$')

def parse(cmd):
    p = SEC / f'{cmd}_command.txt'
    if not p.exists(): return None
    txt = p.read_text()
    fail = txt[txt.find('Failure conditions'):txt.find('Success conditions')] if 'Failure conditions' in txt else ''
    conds, cur = {}, None
    for line in fail.split('\n'):
        m = ID_LINE.match(line)
        if m: cur = m.group(1); conds[cur] = {'pre': m.group(2).strip(), 'code': None}; continue
        m = POST.match(line)
        if m and cur: conds[cur]['code'] = conds[cur]['code'] or (m.group(1) or m.group(2))
    edges = []
    for line in fail.split('\n'):
        m = ORDER.match(line)
        if m:
            lo = [x.strip() for x in m.group(1).split(',')]
            hi = [x.strip() for x in m.group(2).split(',')]
            edges += [(a, b) for a in lo for b in hi]
    # transitive closure (both directions count as "ordered")
    ordered = set(edges)
    changed = True
    while changed:
        changed = False
        for a, b in list(ordered):
            for c, d in list(ordered):
                if b == c and (a, d) not in ordered:
                    ordered.add((a, d)); changed = True
    return conds, ordered

def uncovered(cmd):
    r = parse(cmd)
    if not r: return None
    conds, ordered = r
    ids = [k for k, v in conds.items() if v['code']]
    out = []
    for i in range(len(ids)):
        for j in range(i+1, len(ids)):
            a, b = ids[i], ids[j]
            if conds[a]['code'] == conds[b]['code']: continue
            if (a, b) in ordered or (b, a) in ordered: continue
            out.append((a, conds[a]['code'], b, conds[b]['code']))
    return conds, ordered, out

if __name__ == '__main__':
    for cmd in sys.argv[1:]:
        r = uncovered(cmd)
        if not r: print(f"{cmd}: no section"); continue
        conds, ordered, unc = r
        print(f"{cmd}: {len(conds)} conditions, {len(ordered)} ordering pairs (closed), {len(unc)} UNCOVERED differing-code pairs")
        for a, ca, b, cb in unc[:6]:
            print(f"     {a} ({ca})  vs  {b} ({cb})")
