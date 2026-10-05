#!/usr/bin/env python3
"""Second round of the condition-format diagnostic: structure and vocabulary.

    python3 scripts/restyle_section.py --base <restyle_conditions.py output> [--jobs 8]

Round one (restyle_conditions.py) rewrote only the condition tables into prose and
the fine-tuned 9B did NOT collapse: 2/49 vacuous against 73-95% on the six unseen
specifications. Two differences from those documents survived the rewrite, and
this script removes each one separately and then both:

    alp14_full        D: the whole section in PSCI's layout -- no Interface/Context/
                         Output tables, no Failure/Success/Footprint headings, no
                         helper or RMM type names anywhere. Written by Claude from
                         the round-one prose version, with a real PSCI section as
                         the style reference.
    alp14_prose_ren   C: round-one prose with every RMM domain term renamed to a
                         fictional domain, command name included. Layout untouched.
    alp14_full_ren    E: D with the same renaming. The closest thing to an unseen
                         document that still has gold.

The renaming map comes from one Claude call and is then applied deterministically
at the level of alphanumeric runs (CamelCase humps and underscore-separated parts
mapped individually, case preserved), so C differs from round-one prose in
vocabulary and in nothing else. The map is saved next to the output.
"""

import argparse
import json
import re
import shutil
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))
from confab_probe import call_claude  # noqa: E402
from restyle_conditions import CALL, CODE, COND, SYSTEM  # noqa: E402

DATA = ROOT / "training-dataset"
RUN = re.compile(r'(?<![A-Za-z0-9])[A-Za-z][A-Za-z0-9]*')
HUMP = re.compile(r'[A-Z]+[0-9]*(?![a-z])|[A-Z][a-z0-9]*|[a-z0-9]+')
IOROW = re.compile(r'(?m)^\s+([a-z][a-z0-9_]*)\s+X\d{1,2}\s+\d+:\d+')
FORBIDDEN = re.compile(r'(?mi)^\s*(?:[A-Z]\d+(?:\.\d+)*\s+)?'
                       r'(Interface|Context|Failure conditions|Success conditions|'
                       r'Footprint|Input values|Output values)\s*$')

FULL = """\
Below is one command from the Arm Realm Management Monitor (RMM) specification.
Rewrite the WHOLE section in the layout and style of the Arm PSCI specification.
This is how PSCI describes a function:

----- PSCI example -----
{example}
----- end of example -----

Rules:
- First line: the section number and the command name exactly as in the original
  heading, e.g. `B4.3.38 RMI_RTT_CREATE`.
- Then prose subsections in PSCI's manner (for example: Intended use, Parameters,
  Return values, Implementation responsibilities), with bullet points as PSCI uses.
- Parameters: one bullet per input value with its exact name, the register it is
  passed in, and what it means. Likewise for each output value.
- State every failure condition as "<CODE> is returned if ...". State the success
  conditions, the ordering between failure conditions, and what state the command
  may modify, as plain sentences.
- Do NOT use any of: RMM's tables (Input values, Context, Output values); the
  headings Interface, Context, Failure conditions, Success conditions, Footprint;
  helper function names (RealmAt, RttWalk, GranuleAt, ...); RMM type names
  (RmmRealm, RmiCommandReturnCode, UInt64, Address, ...); pseudo-code operators;
  condition IDs.
- Keep by name: the command name, every input and output value name, every return
  or error code, and state or enumeration values.
- Keep every piece of information. Every condition, ordering constraint, success
  condition and modifiable-state item must still be stated. Add nothing that is
  not in the original.

Section:
"""

MAP = """\
Below are the distinct words and identifier parts that occur in 49 command
descriptions from one hardware interface specification (Arm CCA, Realm Management
Monitor). I want to disguise the domain so a reader cannot tell it is Arm CCA / RMM,
while the text stays readable and every statement keeps its meaning.

Return a JSON object that maps each DOMAIN-SPECIFIC item to a replacement taken
from one coherent fictional domain (for example, a storage appliance controller).
- Map only domain-specific terms: names of entities, structures, states, interface
  and architecture names, and acronyms (RMI, RSI, RMM, RTT, IPA, PA, REC, RD,
  RIPAS, PDEV, VDEV, ...). Arm-specific names count as domain-specific.
- Leave generic English and generic computing words unmapped (error, input,
  success, address, level, state, size, table, entry, index, register, value,
  command, memory, page, ...).
- Keys: copied exactly as given (lowercase). Values: lowercase, one alphanumeric
  word, no spaces or underscores, not itself in the list, all distinct.
- Return only the JSON object.

Items:
"""


def runs(text):
    out = set()
    for r in RUN.findall(text):
        out.add(r.lower())
        if not (r.isupper() or r.islower()):
            out |= {h.lower() for h in HUMP.findall(r)}
    return out


def recase(src, word):
    if src.isupper() and len(src) > 1:
        return word.upper()
    if src[0].isupper():
        return word[:1].upper() + word[1:]
    return word


def rename(text, m):
    def one(mo):
        r = mo.group(0)
        if r.lower() in m:
            return recase(r, m[r.lower()])
        if r.isupper() or r.islower():
            return r
        return "".join(recase(h, m[h.lower()]) if h.lower() in m else h
                       for h in HUMP.findall(r))
    return RUN.sub(one, text)


def leftover(text, m):
    """Mapped terms still present after renaming; must be empty."""
    return sorted(runs(text) & set(m))


def check_full(orig, new, cmd):
    bad = []
    if cmd not in new:
        bad.append("command name lost")
    if COND.search(new):
        bad.append("pre:/post: present")
    f = FORBIDDEN.findall(new)
    if f:
        bad.append(f"RMM headings present: {sorted(set(f))[:3]}")
    names = set(IOROW.findall(orig)) - {"fid"}
    lost = sorted(n for n in names if not re.search(rf'\b{re.escape(n)}\b', new))
    if lost:
        bad.append(f"value names lost: {lost[:5]}")
    have = set(CODE.findall(new))
    codes = {c for c in CODE.findall(orig) if not any(h.startswith(c) for h in have)}
    if codes:
        bad.append(f"return codes lost: {sorted(codes)[:4]}")
    leaked = set(CALL.findall(new)) & set(CALL.findall(orig))
    if leaked:
        bad.append(f"helper names leaked: {sorted(leaked)[:4]}")
    return bad


def full_one(cmd, text, example, model, effort):
    why = []
    for _ in range(2):
        try:
            new = call_claude(SYSTEM, FULL.format(example=example) + text, model, effort).strip()
        except Exception as e:  # noqa: BLE001
            why = [f"call failed: {e}"]
            continue
        new = re.sub(r'^```\w*\n|\n```$', '', new)
        why = check_full(text, new, cmd)
        if not why:
            return cmd, new + "\n", None
    return cmd, None, why


def write_version(out, v, items, gold_dir):
    """items: list of (new command name, section text, original command name)."""
    (out / "sections" / v).mkdir(parents=True, exist_ok=True)
    (out / "specs" / v).mkdir(parents=True, exist_ok=True)
    shutil.copy(gold_dir / "preamble.rs", out / "specs" / v / "preamble.rs")
    for new_cmd, text, cmd in items:
        (out / "sections" / v / f"{new_cmd}_command.txt").write_text(text)
        shutil.copy(gold_dir / f"{cmd.lower()}_spec.rs",
                    out / "specs" / v / f"{new_cmd.lower()}_spec.rs")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", required=True, help="output dir of restyle_conditions.py")
    ap.add_argument("--model", default="claude-opus-5-5")
    ap.add_argument("--effort", default="medium")
    ap.add_argument("--jobs", type=int, default=8)
    args = ap.parse_args()

    base = Path(args.base)
    kept = json.loads((base / "restyle_report.json").read_text())["kept"]
    prose = {c: (base / "sections" / "alp14_prose" / f"{c}_command.txt").read_text() for c in kept}
    example = "\n".join((DATA / "sections" / "psci_13" / "CPU_ON_command.txt")
                        .read_text().splitlines()[:45])
    gold_dir = DATA / "specs" / "alp14"
    report = {"model": args.model, "effort": args.effort}

    # D: full PSCI layout.
    with ThreadPoolExecutor(args.jobs) as ex:
        res = list(ex.map(lambda c: full_one(c, prose[c], example, args.model, args.effort), kept))
    full = {c: t for c, t, _ in res if t}
    report["full_rejected"] = {c: w for c, _, w in res if w}
    for c, w in report["full_rejected"].items():
        print(f"[reject] {c} full: {w}", flush=True)
    keep = [c for c in kept if c in full]

    # Renaming map, from the vocabulary of both texts and the command names.
    vocab = set()
    for c in keep:
        vocab |= runs(prose[c]) | runs(full[c]) | runs(c)
    vocab = sorted(w for w in vocab if len(w) > 1 and not w.isdigit())
    raw = call_claude(SYSTEM, MAP + "\n".join(vocab), args.model, "high")
    raw = raw[raw.find("{"): raw.rfind("}") + 1]
    m = {k.lower(): v.lower() for k, v in json.loads(raw).items()
         if k.lower() in vocab and re.fullmatch(r'[a-z][a-z0-9]*', v.lower())}
    clash = (set(m.values()) & set(vocab)) | {v for v in m.values() if list(m.values()).count(v) > 1}
    for k in [k for k, v in m.items() if v in clash]:
        m.pop(k)
    report["rename_map_size"] = len(m)
    report["rename_dropped_for_clash"] = sorted(clash)
    (base / "rename_map.json").write_text(json.dumps(m, indent=1, sort_keys=True))

    items = {"alp14_full": [], "alp14_prose_ren": [], "alp14_full_ren": []}
    names, residue = {}, {}
    for c in keep:
        nc = rename(c, m)
        names[c] = nc
        items["alp14_full"].append((c, full[c], c))
        for v, src in (("alp14_prose_ren", prose[c]), ("alp14_full_ren", full[c])):
            t = rename(src, m)
            left = leftover(t, m)
            if left:
                residue[f"{c}/{v}"] = left
            items[v].append((nc, t, c))
    if len(set(names.values())) != len(names):
        sys.exit("renamed command names collide")
    for v, it in items.items():
        write_version(base, v, it, gold_dir)

    report.update(kept=keep, renamed_commands=names, rename_residue=residue)
    (base / "restyle_section_report.json").write_text(
        json.dumps(report, indent=1, ensure_ascii=False))
    print(f"[restyle-section] D kept {len(keep)}/{len(kept)}; map {len(m)} terms; "
          f"residue in {len(residue)} files -> {base}", flush=True)


if __name__ == "__main__":
    main()
