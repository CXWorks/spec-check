#!/usr/bin/env python3
"""Rewrite only the condition subsections of held-out alp14 commands, gold unchanged.

    python3 scripts/restyle_conditions.py --out <dir> [--jobs 8]

Diagnostic for the transfer failure. On alp14 the fine-tuned 9B is vacuous on 3/98
commands, exactly like gold; on six unseen specifications it is vacuous on 73-95%.
One candidate cause is how conditions are written. RMM states them as pseudo-code
tables

    gran_align   pre:  !AddrIsGranuleAligned(addr)
                 post: ResultEqual(result, RMI_ERROR_INPUT)

while PSCI and the others say the same thing in prose ("INVALID_PARAMETERS is
returned if target_cpu describes an invalid MPIDR"). If the model learned to
transliterate the table, taking the table away should reproduce the collapse on a
document it was trained on -- and, unlike the six documents, gold still exists to
score against.

Three versions are written, each a self-contained `sections/<v>` + `specs/<v>`
pair that `gen_specs.py --versions <v>` loads unchanged:

    alp14_ctl     the original text                       (control)
    alp14_layout  table rows -> sentences, every expression copied verbatim
    alp14_prose   PSCI-style prose: no pseudo-code, no helper names, no IDs

Only the span from the "Failure conditions" heading up to the "Footprint" heading
is sent to Claude, and the reply is spliced back, so everything else in the
section is byte-identical across the three. Each reply is checked mechanically
before it is accepted; a reply that fails is retried, and one that fails twice is
reported and left out rather than kept.
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

DATA = ROOT / "training-dataset"
START = re.compile(r'(?m)^\s*[A-Z]?[\d.]+\s+Failure conditions\s*$')
STOP = re.compile(r'(?m)^\s*[A-Z]?[\d.]+\s+Footprint\s*$')
HEAD = re.compile(r'(?m)^\s*[A-Z]\d+(?:\.\d+)+\s+[A-Z].*$')
COND = re.compile(r'\b(pre|post):')
CODE = re.compile(r'\b(?:RMI|RSI|PSCI)_[A-Z0-9_]+\b')
CALL = re.compile(r'\b([A-Z][a-z0-9]+[A-Z][A-Za-z0-9]*)\s*\(')
IDENT = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')

SYSTEM = ("You rewrite one excerpt of a hardware interface specification. Return only "
          "the rewritten excerpt: no commentary, no preamble, no code fence.")

LAYOUT = """\
Below are the condition subsections of one command from the Arm Realm Management
Monitor specification. Conditions are laid out as a table: an ID column, then
`pre:` and `post:` lines.

Rewrite the table rows as ordinary English sentences, one sentence per row. This is
a LAYOUT change only:
- Copy every `pre:` and `post:` expression EXACTLY, character for character:
  function names, operators, arguments, field accesses.
- Keep each ID as a label at the start of its sentence, in parentheses, so that
  ordering statements that cite IDs still make sense.
- Keep every heading line exactly as it is.
- Remove the `ID  Condition` header rows and the `pre:` / `post:` labels.
- Do not add, drop, merge, split or reorder conditions.

Example
  gran_align     pre:  !AddrIsGranuleAligned(addr)
                 post: ResultEqual(result, RMI_ERROR_INPUT)
becomes
  (gran_align) If !AddrIsGranuleAligned(addr), then ResultEqual(result, RMI_ERROR_INPUT).

Excerpt:
"""

PROSE = """\
Below are the condition subsections of one command from the Arm Realm Management
Monitor specification. Conditions are written as a pseudo-code table: an ID column,
then `pre:` and `post:` lines that call helper functions.

Rewrite them in the style of the Arm PSCI specification, which describes the same
kind of information in plain English, for example:
  "INVALID_PARAMETERS is returned if target_cpu describes an invalid MPIDR."
  "ALREADY_ON is returned if the core is already in an ON state."

Rules:
- No pseudo-code at all: no helper function names (AddrIsGranuleAligned,
  ResultEqual, GranuleAt, ...), no operators (!, ==, !=, &&, ||), no `pre:` or
  `post:`, no condition IDs, no tables. Say what each one means in words.
- Keep, by name: the command's input and output arguments, every return or error
  code (RMI_ERROR_INPUT, ...), and state or enumeration values (DELEGATED,
  REALM, ...). Use a field name only where words alone would be ambiguous.
- Every condition must still be there, with the same meaning. Do not add, drop
  or merge any. State ordering constraints in words.
- Keep every heading line exactly as it is.

Excerpt:
"""


def span(text):
    a, b = START.search(text), STOP.search(text)
    if not a or not b or b.start() <= a.start():
        return None
    return a.start(), b.start()


def cond_lines(excerpt):
    """`pre:`/`post:` expressions, continuation lines joined on."""
    out, cur = [], None
    for line in excerpt.splitlines():
        # Anything may precede the label: long IDs wrap inside their PDF column,
        # so a row can read `pdev_1_gran_sta te    pre: ...`.
        m = re.match(r'^.*?\b(pre|post):\s*(.*)$', line)
        if m:
            if cur is not None:
                out.append(cur)
            cur = m.group(2)
        elif cur is not None and line.strip() and not HEAD.match(line) \
                and not line.strip().startswith("ID"):
            cur += " " + line.strip()
        else:
            if cur is not None:
                out.append(cur)
            cur = None
    if cur is not None:
        out.append(cur)
    return out


def check(style, orig, new):
    """Reasons to reject a rewrite; empty means accept."""
    bad = []
    if COND.search(new):
        bad.append("still has pre:/post: rows")
    if re.search(r'(?m)^\s*ID\s{2,}Condition', new):
        bad.append("still has the table header")
    flat = " ".join(new.split())
    for h in HEAD.findall(orig):
        if " ".join(h.split()) not in flat:
            bad.append(f"heading lost: {h.strip()[:50]}")
    # Prefix match: the ordering diagrams are PDF graphics whose labels come out
    # truncated (RMI_ERROR_RT for RMI_ERROR_RTT, 24 times in alp14), and a
    # rewrite that restores the full name has not lost anything.
    have = set(CODE.findall(new))
    lost = {c for c in CODE.findall(orig) if not any(h.startswith(c) for h in have)}
    if lost:
        bad.append(f"return codes lost: {sorted(lost)[:4]}")
    if style == "layout":
        want = set()
        for e in cond_lines(orig):
            want |= set(IDENT.findall(e))
        missing = want - set(IDENT.findall(new))
        if missing:
            bad.append(f"expression identifiers lost: {sorted(missing)[:6]}")
    else:
        leaked = set(CALL.findall(new)) & set(CALL.findall(orig))
        if leaked:
            bad.append(f"helper names leaked: {sorted(leaked)[:4]}")
    return bad


def restore_lines(orig, new):
    """Put back the exact original bytes of every line the rewrite did not change.

    Claude tends to drop the PDF's leading indentation on lines it was told to
    keep (headings, "does not have any failure conditions"). That is harmless
    to a reader but it is a second difference between the arms, on top of the
    one being measured, so it is undone here.
    """
    norm = {" ".join(l.split()): l for l in orig.splitlines() if l.strip()}
    return "\n".join(norm.get(" ".join(l.split()), l) for l in new.splitlines())


def restyle(job, model, effort):
    cmd, style, excerpt = job
    prompt = (LAYOUT if style == "layout" else PROSE) + excerpt
    why = []
    for attempt in range(2):
        try:
            new = call_claude(SYSTEM, prompt, model, effort).strip("\n")
        except Exception as e:  # noqa: BLE001 -- one failed call must not end the run
            why = [f"call failed: {e}"]
            continue
        new = re.sub(r'^```\w*\n|\n```$', '', new)
        why = check(style, excerpt, new)
        if not why:
            return cmd, style, restore_lines(excerpt, new) + "\n", None
    return cmd, style, None, why


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True, help="root that receives sections/ and specs/")
    ap.add_argument("--model", default="claude-opus-5-5")
    ap.add_argument("--effort", default="medium")
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--limit", type=int, default=None)
    ap.add_argument("--commands", nargs="+", default=None, help="subset, for trial runs")
    args = ap.parse_args()

    held = sorted(json.loads((DATA / "dataset_bench" / "splits.json").read_text())["command_test"])
    if args.commands:
        held = [c for c in held if c in args.commands]
    if args.limit:
        held = held[: args.limit]
    src_sec, src_spec = DATA / "sections" / "alp14", DATA / "specs" / "alp14"
    out = Path(args.out)

    texts, jobs = {}, []
    for c in held:
        t = (src_sec / f"{c}_command.txt").read_text()
        s = span(t)
        if s is None:
            sys.exit(f"{c}: no Failure conditions ... Footprint span")
        texts[c] = (t, s)
        for style in ("layout", "prose"):
            jobs.append((c, style, t[s[0]:s[1]]))

    for v in ("alp14_ctl", "alp14_layout", "alp14_prose"):
        (out / "sections" / v).mkdir(parents=True, exist_ok=True)
        (out / "specs" / v).mkdir(parents=True, exist_ok=True)
        shutil.copy(src_spec / "preamble.rs", out / "specs" / v / "preamble.rs")

    report = {"model": args.model, "effort": args.effort, "commands": len(held),
              "rejected": {}}
    with ThreadPoolExecutor(args.jobs) as ex:
        results = list(ex.map(lambda j: restyle(j, args.model, args.effort), jobs))

    done = {}
    for cmd, style, new, why in results:
        if new is None:
            report["rejected"][f"{cmd}/{style}"] = why
            print(f"[reject] {cmd} {style}: {why}", flush=True)
        else:
            done[(cmd, style)] = new

    # A command enters the comparison only if BOTH rewrites were accepted, so the
    # three arms always cover the same commands.
    keep = [c for c in held if (c, "layout") in done and (c, "prose") in done]
    for c in keep:
        t, (a, b) = texts[c]
        gold = src_spec / f"{c.lower()}_spec.rs"
        for v, body in (("alp14_ctl", t[a:b]),
                        ("alp14_layout", done[(c, "layout")]),
                        ("alp14_prose", done[(c, "prose")])):
            (out / "sections" / v / f"{c}_command.txt").write_text(t[:a] + body + t[b:])
            shutil.copy(gold, out / "specs" / v / gold.name)

    report["kept"] = keep
    (out / "restyle_report.json").write_text(json.dumps(report, indent=1, ensure_ascii=False))
    print(f"[restyle] kept {len(keep)}/{len(held)} commands; "
          f"rejected {len(report['rejected'])} rewrites -> {out}", flush=True)


if __name__ == "__main__":
    main()
