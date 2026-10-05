#!/usr/bin/env python3
"""The diagnostic in reverse: make an unseen specification look like RMM.

    python3 scripts/restyle_mirror.py --out <dir> [--docs psci_13 sdei] [--jobs 8]

restyle_conditions.py and restyle_section.py made held-out RMM look like an unseen
document -- conditions in prose, PSCI layout, every domain term renamed -- and the
fine-tuned 9B never collapsed: at most 3/49 vacuous, against 45-87% on the real
unseen documents in the same no-preamble configuration. This goes the other way,
on the real documents, and writes two versions of each:

    <doc>_clean   the same section with PDF extraction noise removed (running
                  headers and footers, orphaned bullets, broken hyphenation),
                  wording and layout otherwise kept. Separates extraction noise
                  from content.
    <doc>_rmm     the same information in RMM's layout: Interface tables, a
                  Failure conditions table of `pre:` / `post:` pseudo-code rows,
                  Success conditions, Footprint. This is option 1 of the plan --
                  doc -> intermediate representation -> code -- with Claude
                  producing the intermediate form and the 9B the code.

There is no gold for these documents, so the measures are the gold-free ones:
vacuity, clause count, repetition, and which return codes the spec uses. Every
reply must keep every return code the original section names (from that
document's own code table, results/returns/returns-<doc>.json) and the command
name; the RMM version must actually contain pre:/post: rows.
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
from restyle_conditions import SYSTEM  # noqa: E402

DATA = ROOT / "training-dataset"

CLEAN = """\
Below is one function's section from an Arm firmware interface specification, as
extracted from the PDF. Remove the extraction noise and nothing else:
- drop running page headers and footers, document IDs, copyright and page-number
  lines, and stray characters;
- re-join words and sentences broken across lines, and attach orphaned bullet
  markers to their text;
- keep the original wording, order, headings and tables. Do not summarise,
  rephrase, reorder, add or drop any content.

Section:
"""

RMM = """\
Below is one function's section from an Arm firmware interface specification.
Rewrite it in the layout of the Arm Realm Management Monitor (RMM) specification,
which describes every command like this:

----- RMM example -----
{example}
----- end of example -----

Rules:
- First line: `<section number> <FUNCTION_NAME> command`, keeping the original
  function name exactly.
- Interface: an Input values table (Name, Register, Bits, Type, Description) and
  an Output values table, from the parameters and return values the section gives.
- Failure conditions: a table with one row per error case, each an ID, a
  `pre:` line and a `post:` line. `pre:` is the condition as a pseudo-code
  predicate; `post:` is `ResultEqual(result, <ERROR_CODE>)` with the document's
  own return code name. Helper functions in the predicates may be named freely,
  but descriptively and consistently (for example `!IsValidMpidr(target_cpu)`).
- Failure condition ordering, if the section implies one; otherwise say there is none.
- Success conditions: `post:` rows for what holds on success, including the
  success return code.
- Footprint: the state the function may modify.
- Use ONLY information in the section. Do not invent conditions, codes or
  behaviour. If the section does not say something, leave it out.

Section:
"""


STRICT_RULES = """
Additional rule for the pre:/post: lines -- this matters more than anything above:
- Every `pre:` and `post:` line must be ONE expression in this restricted
  pseudo-code and nothing else: helper calls `Name(arg, ...)`, field access
  `x.f`, integer literals, named constants, comparisons (== != < <= > >=),
  `!`, `&&`, `||`, `==>`, arithmetic (+ - * / %), parentheses. Quantifiers only
  as `forall|x: Type| expr` or `exists|x: Type| expr`.
- No English words or sentences, no bit slices such as `x[31:16]` (use a helper
  such as `Bits(x, 31, 16)`), no `? :`, no `if`, no assignments.
- A condition that cannot be written this way gets a descriptive helper
  predicate instead, e.g. `post: CoreRestartsAtEntryPoint(target_cpu, entry_point_address)`.
"""

EXPR_ROW = re.compile(r'\b(?:pre|post):\s*(.*)$')


def strict_violations(text):
    """Why the pre:/post: expressions are not in the restricted pseudo-code."""
    exprs, cur = [], None
    for line in text.splitlines():
        m = EXPR_ROW.search(line)
        if m:
            if cur is not None:
                exprs.append(cur)
            cur = m.group(1)
        elif cur is not None and line.strip() and not re.match(r'^\s*[A-Z]?[\d.]+\s+[A-Z]', line) \
                and not re.match(r'^\s*\w+\s{2,}(pre|post|ID)\b', line):
            cur += " " + line.strip()
        else:
            if cur is not None:
                exprs.append(cur)
            cur = None
    if cur is not None:
        exprs.append(cur)
    bad = []
    for e in exprs:
        words = [w for w in re.findall(r'\b([A-Za-z_]\w*)\s+([A-Za-z_]\w*)\b', e)
                 if "as" not in w]
        if words:
            bad.append(f"prose: {' '.join(words[0])!r} in {e[:60]!r}")
        if re.search(r'\[\s*\d+\s*:\s*\d+\s*\]', e):
            bad.append(f"bit slice in {e[:60]!r}")
        if "?" in e:
            bad.append(f"'?' in {e[:60]!r}")
        if re.search(r'\b(forall|exists)\b(?!\s*\|)', e):
            bad.append(f"quantifier not in Verus form in {e[:60]!r}")
    return bad[:3]


def doc_codes(doc, text):
    vocab = json.loads((ROOT / "results" / "returns" / f"returns-{doc}.json").read_text())["vocab"]
    return {c for c in vocab if re.search(rf'\b{c}\b', text)}


def check(style, doc, cmd, orig, new):
    bad = []
    # SCMI's names carry their section number (BASE_DISCOVER_AGENT__3_2_2_9) to
    # tell apart same-named messages in different protocols; the document itself
    # never writes the suffix, so only the name before it is required.
    if re.sub(r'__\d+(?:_\d+)*$', '', cmd) not in new:
        bad.append("function name lost")
    lost = doc_codes(doc, orig) - doc_codes(doc, new)
    if lost:
        bad.append(f"return codes lost: {sorted(lost)}")
    if style == "clean":
        if re.search(r'\b(pre|post):', new):
            bad.append("pseudo-code introduced")
        if len(new) < 0.4 * len(orig):
            bad.append(f"too short: {len(new)} vs {len(orig)} chars")
    else:
        if style == "strict":
            bad += strict_violations(new)
        if not re.search(r'\bpost:', new):
            bad.append("no post: rows")
        for h in ("Input values", "Output values", "Failure conditions", "Success conditions", "Footprint"):
            if h not in new:
                bad.append(f"missing heading: {h}")
    return bad


def one(job, example, model, effort):
    doc, cmd, style, text = job
    prompt = (CLEAN if style == "clean" else
              RMM.format(example=example) + (STRICT_RULES if style == "strict" else "")) + text
    why = []
    for _ in range(2):
        try:
            new = call_claude(SYSTEM, prompt, model, effort).strip()
        except Exception as e:  # noqa: BLE001
            why = [f"call failed: {e}"]
            continue
        new = re.sub(r'^```\w*\n|\n```$', '', new)
        why = check(style, doc, cmd, text, new)
        if not why:
            return doc, cmd, style, new + "\n", None
    return doc, cmd, style, None, why


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", required=True)
    ap.add_argument("--docs", nargs="+", default=["psci_13", "sdei"])
    ap.add_argument("--model", default="claude-opus-5-5")
    ap.add_argument("--effort", default="medium")
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--styles", nargs="+", default=["clean", "rmm"], choices=["clean", "rmm", "strict"])
    args = ap.parse_args()

    out = Path(args.out)
    example = (DATA / "sections" / "alp14" / "RMI_GRANULE_DELEGATE_command.txt").read_text()
    jobs = []
    for doc in args.docs:
        for f in sorted((DATA / "sections" / doc).glob("*_command.txt")):
            if f.name.startswith("._"):
                continue
            cmd = f.name[: -len("_command.txt")]
            for style in args.styles:
                jobs.append((doc, cmd, style, f.read_text()))

    with ThreadPoolExecutor(args.jobs) as ex:
        res = list(ex.map(lambda j: one(j, example, args.model, args.effort), jobs))

    report = {"model": args.model, "effort": args.effort, "rejected": {}, "kept": {}}
    done = {}
    for doc, cmd, style, new, why in res:
        if new is None:
            report["rejected"][f"{doc}/{cmd}/{style}"] = why
            print(f"[reject] {doc} {cmd} {style}: {why}", flush=True)
        else:
            done[(doc, cmd, style)] = new

    for doc in args.docs:
        cmds = sorted({c for d, c, s in done if d == doc})
        keep = [c for c in cmds if all((doc, c, st) in done for st in args.styles)]
        report["kept"][doc] = keep
        for style in args.styles:
            v = f"{doc}_{style}"
            (out / "sections" / v).mkdir(parents=True, exist_ok=True)
            (out / "specs" / v).mkdir(parents=True, exist_ok=True)
            shutil.copy(DATA / "specs" / doc / "preamble.rs", out / "specs" / v / "preamble.rs")
            for c in keep:
                (out / "sections" / v / f"{c}_command.txt").write_text(done[(doc, c, style)])
        print(f"[mirror] {doc}: kept {len(keep)} commands", flush=True)
    (out / "mirror_report.json").write_text(json.dumps(report, indent=1, ensure_ascii=False))


if __name__ == "__main__":
    main()
