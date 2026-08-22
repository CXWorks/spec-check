#!/usr/bin/env python3
"""Check a generated spec against the DOCUMENT, not against gold.

    python3 scripts/provenance.py --version eac5 --gold          # audit gold itself
    python3 scripts/provenance.py --version eac5 --gen-dir DIR    # audit a generation

Every correctness number in this project so far measures agreement with gold, and
`semantic_equiv.py` says so in its own docstring: *gold is a human reading of the
text, not the text*. That ceiling is real and we have already hit it —
`RMI_VSMMU_CREATE`'s `idr` came back `stronger` in five independent runs and the
document's Footprint section says the model was right and gold was short a
clause. A metric anchored on gold cannot tell that case from a model error.

What makes a document-anchored check possible here is that the RMM spec is not
prose. Each command section carries three labelled tables:

    B4.3.5.2 Failure conditions      ID -> pre:/post: pair
    B4.3.5.3 Success conditions      ID -> condition
    B4.3.5.4 Footprint               ID -> a state expression that may change

The IDs (`gran_align`, `gran_state`, ...) are stable strings in the document. So
"which part of the PDF does this clause come from" is an exact question, not a
fuzzy span-matching one, and three things become checkable:

    coverage      a document row with no clause    -> the spec MISSED a stated condition
    groundedness  a clause matching no row         -> the spec INVENTED a constraint
    dangling      a declared output never used     -> the document gap SCOPE hunts for

The third is what `rule_check_8bugs` already tests. The first two are new, and
the first is the one that can audit gold: run this with --gold and any coverage
miss is a clause the document states and the human transcription does not.

**On attribution.** Clauses are matched to rows by identifier overlap, which is a
heuristic, so this tool reports an explicit `unattributed` bucket instead of
forcing every clause into some row. Validate before believing: `--gold` should
ground nearly everything, because gold was written from these very tables. If it
does not, the attributor is wrong, not gold. That check is `--self-test`.
"""

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SECTIONS = ROOT / "training-dataset" / "sections"
SPECS = ROOT / "training-dataset" / "specs"

# Identifiers that carry no attribution signal: they appear in nearly every
# clause of every command, so counting them would let any clause match any row.
STOPWORDS = {
    "pre", "post", "result", "old_s", "new_s", "s", "int", "nat", "bool",
    "true", "false", "is_Ok", "is_Err", "ResultEqual", "UInt", "Equal",
}

# A row's ID is the most reliable anchor of all: the document's `reg_base`,
# `idr`, `gran_state` reappear almost verbatim as the field the clause
# constrains. Prefixes name the operand, not the property, so they are stripped
# before matching -- `gran_state` and `rd_state` both anchor on `state`.
ID_PREFIXES = ("gran_", "rd_", "ipa_", "rec_", "rtt_", "realm_", "level_",
               "vsmmu_", "pdev_", "vdev_", "mec_", "src_", "dst_")


# ---------------------------------------------------------------- document ---

def parse_section(text):
    """Pull the three labelled tables and the declared outputs out of a section.

    Rows are recognised by indentation: a table's ID column sits at the same
    indent as its `ID  Condition` header, and continuation lines are indented
    further. That is a property of the extracted text, so it is checked rather
    than assumed -- a section whose header is not found yields an empty table
    and is reported, never silently treated as "no conditions".
    """
    lines = text.split("\n")
    out = {"outputs": [], "failure": {}, "success": {}, "footprint": {}}

    def table_after(header_re, want):
        """Rows of the first table following a heading matching header_re."""
        for i, l in enumerate(lines):
            if not re.search(header_re, l):
                continue
            # find the `ID  <something>` header line that starts the table
            for j in range(i + 1, min(i + 6, len(lines))):
                m = re.match(r"^(\s*)ID\s+\S", lines[j])
                if not m:
                    continue
                indent = len(m.group(1))
                rows, cur = {}, None
                for k in range(j + 1, len(lines)):
                    l2 = lines[k]
                    if not l2.strip():
                        continue
                    ind = len(l2) - len(l2.lstrip())
                    if ind < indent:          # dedent: the table has ended
                        break
                    if ind == indent:
                        parts = re.split(r"\s{2,}", l2.strip(), maxsplit=1)
                        if not re.match(r"^[A-Za-z_][A-Za-z0-9_]*$", parts[0]):
                            break             # not an ID column -- not our table
                        if parts[0] == "ID":
                            # The table's own header, repeated because the PDF
                            # broke it across a page. Reading it as a row
                            # invented a condition named "ID" in 82 of 98 alp14
                            # commands -- 98 of 313 apparent gold misses, nearly
                            # a third of the finding, and pure instrument error.
                            cur = None
                            continue
                        cur = parts[0]
                        # An ID repeated in one table is two conditions sharing a
                        # name (the spec does this); keep both, joined.
                        rows[cur] = (rows.get(cur, "") + " " +
                                     (parts[1] if len(parts) > 1 else "")).strip()
                    elif cur:
                        rows[cur] = (rows[cur] + " " + l2.strip()).strip()
                want.update(rows)
                return
        # header genuinely absent -- leave `want` empty; the caller reports it

    table_after(r"Failure conditions\s*$", out["failure"])
    table_after(r"Success conditions\s*$", out["success"])
    table_after(r"Footprint\s*$", out["footprint"])

    # An empty table has two very different causes and they must not be pooled.
    # PSCI_CPU_OFF really has no failure conditions, no success conditions and no
    # footprint -- the document says so in prose ("does not have any ..."), and
    # its gold spec is correspondingly vacuous. That is a valid zero. A table the
    # parser simply failed on is a bug. Record which, per table.
    out["stated_none"] = {
        t: bool(re.search(rf"does not have any {kw}", text, re.I))
        for t, kw in (("failure", "failure conditions"),
                      ("success", "success conditions"),
                      ("footprint", "footprint"))
    }

    # Output values: a Name/Register/Bits/Type/Description table, no ID column.
    for i, l in enumerate(lines):
        if not re.search(r"Output values\s*$", l):
            continue
        for j in range(i + 1, min(i + 4, len(lines))):
            m = re.match(r"^(\s*)Name\s+Register\s+Bits", lines[j])
            if not m:
                continue
            indent = len(m.group(1))
            for k in range(j + 1, len(lines)):
                l2 = lines[k]
                if not l2.strip():
                    continue
                ind = len(l2) - len(l2.lstrip())
                if ind < indent:
                    break
                if ind == indent:
                    name = re.split(r"\s{2,}", l2.strip())[0]
                    if re.match(r"^[a-z_][a-z0-9_]*$", name):
                        out["outputs"].append(name)
                    else:
                        break
            break
        break
    return out


# ------------------------------------------------------------------- spec ---

def split_conjuncts(src):
    """Top-level `&&` conjuncts of the function body, parens respected."""
    i = src.find("{")
    j = src.rfind("}")
    if i < 0 or j < i:
        return []
    body = src[i + 1:j]
    parts, depth, cur, k = [], 0, [], 0
    while k < len(body):
        c = body[k]
        if c in "([{":
            depth += 1
        elif c in ")]}":
            depth -= 1
        if depth == 0 and body.startswith("&&", k):
            parts.append("".join(cur).strip())
            cur = []
            k += 2
            continue
        cur.append(c)
        k += 1
    parts.append("".join(cur).strip())
    return [p for p in parts if p]


def norm(w):
    """Fold a Verus identifier onto the name the document uses for the same thing.

    The two sides name the same object differently and systematically: the
    document writes `vsmmu.reg_base`, gold writes
    `VsmmuAt(new_s, vsmmu_ptr).reg_base`. Raw token overlap there is 1 -- the
    field name alone -- so a threshold of 2 called a clause that plainly encodes
    the row a MISS. That single bug produced most of an apparent "gold omits 313
    document rows", including a false hit on `RMI_VSMMU_CREATE:idr`, which is
    the one case where gold's omission was already established by hand. The tool
    was about to be validated by agreeing with a known answer for the wrong
    reason.
    """
    w = re.sub(r"^(Rmi|Rmm|Rsi)", "", w)
    w = re.sub(r"(At|_ptr|_pre)$", "", w)
    return w.lower()


def idents(text):
    return {norm(w) for w in re.findall(r"[A-Za-z_][A-Za-z0-9_]*", text)
            if w not in STOPWORDS} - {""}


def attribute(clause, doc):
    """Best-matching document row for one clause, or None.

    Scored by size of the shared identifier set. A tie or a zero score means the
    clause is not attributable, and saying so is the point -- an unattributed
    clause is either an invented constraint or a convention the document states
    once globally rather than per command (chiefly "if no failure condition
    holds, the command succeeds", and the guarded frame conditions derived from
    Footprint).
    """
    ci = idents(clause)
    if not ci:
        return None, 0
    best, best_n = None, 0
    for table in ("failure", "success", "footprint"):
        for rid, cond in doc[table].items():
            n = len(ci & idents(cond))
            if n > best_n:
                best, best_n = (table, rid), n
    return best, best_n


def audit(command, spec_src, section_text, min_overlap=2):
    doc = parse_section(section_text)
    clauses = split_conjuncts(spec_src)
    rows = [(t, rid) for t in ("failure", "success", "footprint")
            for rid in doc[t]]

    # Coverage and attribution are different questions and must not share one
    # best-match assignment. Asking "which single row does this clause belong to"
    # and then calling every other row uncovered is how the first version of this
    # reported gold as missing 24 of RMI_REALM_CREATE's 47 rows: forty clauses
    # collapsed onto twenty-three rows because each could claim only one. A row
    # is covered if ANY clause encodes it; a clause is grounded if it encodes ANY
    # row. Both directions, independently.
    clause_ids = [idents(c) for c in clauses]
    row_ids = {(t, rid): idents(cond)
               for t in ("failure", "success", "footprint")
               for rid, cond in doc[t].items()}

    covered = {r for r, ri in row_ids.items()
               if any(len(ci & ri) >= min_overlap for ci in clause_ids)}
    unattributed = [c for c, ci in zip(clauses, clause_ids)
                    if not any(len(ci & ri) >= min_overlap
                               for ri in row_ids.values())]

    body = spec_src[spec_src.find("{") + 1:] if "{" in spec_src else ""
    dangling = [o for o in doc["outputs"]
                if not re.search(rf"\b{re.escape(o)}\b", body)]

    # Frame gaps. The Footprint lists exactly what the command may change, so any
    # field the Success conditions write that the Footprint does NOT list must be
    # unchanged when the command fails -- and the spec has to say so. This is the
    # check that finds RMI_VSMMU_CREATE: its Footprint is {state, num_vsmmus},
    # gold frames aidr, reg_base and reg_top, and omits idr. Five independent
    # runs wrote the idr frame and were scored `stronger` for being right.
    #
    # Derived from the document alone, so it audits gold on the same footing as
    # any generation. An earlier textual screen for this over-fired 29 times by
    # ignoring that these clauses are GUARDED; the guard is the definition here,
    # not an obstacle -- a frame condition counts only if it sits under an
    # error/failure guard.
    def field(rid):
        for pre in ID_PREFIXES:
            if rid.startswith(pre) and len(rid) > len(pre):
                return rid[len(pre):]
        return rid

    fp_fields = {field(r) for r in doc["footprint"]}
    written = {field(r) for r in doc["success"]}
    framed = set()
    for c in clauses:
        if not re.search(r"is_Err|!\s*result\.is_Ok|!=\s*RMI_SUCCESS|!=\s*SUCCESS", c):
            continue
        framed |= idents(c)
    # A command whose Footprint is empty changes no state at all -- it is a read.
    # Its "Success conditions" then constrain OUTPUT values, not state fields, and
    # there is nothing for a frame condition to preserve. Scoring those as gaps
    # fired on 26 alp14 commands including RMI_RTT_READ_ENTRY, whose nine
    # "unframed fields" are its nine outputs.
    frame_gaps = ([] if not fp_fields else
                  sorted(f for f in written - fp_fields if norm(f) not in framed))

    return {
        "command": command,
        "n_clauses": len(clauses),
        "n_doc_rows": len(rows),
        "tables_found": {t: len(doc[t]) for t in ("failure", "success", "footprint")},
        "stated_none": all(doc["stated_none"].values()),
        "outputs": doc["outputs"],
        "covered": sorted(f"{t}:{r}" for t, r in covered),
        "missed": sorted(f"{t}:{r}" for t, r in rows if (t, r) not in covered),
        "unattributed": unattributed,
        "dangling_outputs": dangling,
        "frame_gaps": frame_gaps,
        "footprint_fields": sorted(fp_fields),
    }


# ------------------------------------------------------------------ driver ---

def load_section(version, command):
    p = SECTIONS / version / f"{command}_command.txt"
    if not p.exists():
        return None
    return p.read_text(encoding="utf-8", errors="replace")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--version", default="eac5")
    ap.add_argument("--gold", action="store_true", help="audit the gold specs")
    ap.add_argument("--gen-dir", default=None,
                    help="directory of <command>.rs to audit instead of gold")
    ap.add_argument("--min-overlap", type=int, default=2,
                    help="shared identifiers required to attribute a clause")
    ap.add_argument("--self-test", action="store_true",
                    help="run the attributor against gold and report whether it "
                         "is trustworthy before any generation is judged")
    ap.add_argument("--out", default=None)
    ap.add_argument("--verbose", action="store_true")
    args = ap.parse_args()

    if args.self_test:
        args.gold = True

    pairs = []
    if args.gen_dir:
        d = Path(args.gen_dir)
        for f in sorted(d.glob("*.rs")):
            cmd = f.stem.upper()
            sec = load_section(args.version, cmd)
            if sec:
                pairs.append((cmd, f.read_text(encoding="utf-8", errors="replace"), sec))
    else:
        d = SPECS / args.version
        for f in sorted(d.glob("*_spec.rs")):
            cmd = f.stem[:-5].upper()
            sec = load_section(args.version, cmd)
            if sec:
                pairs.append((cmd, f.read_text(encoding="utf-8", errors="replace"), sec))

    if not pairs:
        sys.exit("no (spec, section) pairs found -- refusing to report a score")

    results = [audit(c, s, t, args.min_overlap) for c, s, t in pairs]

    tot_rows = sum(r["n_doc_rows"] for r in results)
    tot_cov = sum(len(r["covered"]) for r in results)
    tot_cl = sum(r["n_clauses"] for r in results)
    tot_un = sum(len(r["unattributed"]) for r in results)
    no_tables = [r["command"] for r in results
                 if r["n_doc_rows"] == 0 and not r["stated_none"]]
    empty_ok = [r["command"] for r in results
                if r["n_doc_rows"] == 0 and r["stated_none"]]

    label = args.gen_dir or f"gold/{args.version}"
    print(f"\n  {label}: {len(results)} commands\n")
    print(f"  document rows covered   {tot_cov}/{tot_rows} "
          f"({100*tot_cov/tot_rows:.1f}%)")
    print(f"  clauses attributed      {tot_cl-tot_un}/{tot_cl} "
          f"({100*(tot_cl-tot_un)/tot_cl:.1f}%)")
    print(f"  commands with dangling outputs  "
          f"{sum(1 for r in results if r['dangling_outputs'])}")
    fg = [r for r in results if r["frame_gaps"]]
    print(f"  frame-gap CANDIDATES            {len(fg)} commands, "
          f"{sum(len(r['frame_gaps']) for r in fg)} fields")
    print("    ^ a screen, not a finding. The one case verified by hand,")
    print("      RMI_VSMMU_CREATE, comes out as exactly ['idr'] -- but a previous")
    print("      screen of this kind produced 29 false positives, and this one")
    print("      still fires on create-commands where a field of the object being")
    print("      created has no meaningful before-state. Read each before quoting")
    print("      it; --verbose prints them.")
    if empty_ok:
        print(f"  no conditions stated by the document  {len(empty_ok)}: "
              f"{', '.join(empty_ok)}")
    if no_tables:
        print(f"  !! PARSE FAILURE on {len(no_tables)}: {', '.join(no_tables[:5])}")

    if args.self_test:
        print("\n  --- self-test ---")
        print("  Gold was written from these tables, so on gold both rates are a")
        print("  measure of the ATTRIBUTOR, not of gold. Low numbers here mean the")
        print("  parser or the matcher is broken and no generation may be judged")
        print("  with it yet.")
        ok = tot_cov / tot_rows >= 0.80 and (tot_cl - tot_un) / tot_cl >= 0.80
        print(f"\n  verdict: {'USABLE' if ok else 'NOT USABLE -- fix before reporting'}")
        if not ok:
            worst = sorted(results, key=lambda r: len(r["missed"]), reverse=True)[:5]
            print("\n  worst-covered commands, look at these first:")
            for r in worst:
                print(f"    {r['command']:32s} missed {len(r['missed'])}/"
                      f"{r['n_doc_rows']}  unattributed {len(r['unattributed'])}"
                      f"/{r['n_clauses']}")

    if args.verbose:
        for r in results:
            if r["missed"] or r["unattributed"]:
                print(f"\n  {r['command']}")
                if r["missed"]:
                    print(f"    missed rows: {', '.join(r['missed'])}")
                for c in r["unattributed"]:
                    print(f"    unattributed: {c[:90]}")

    if args.out:
        Path(args.out).write_text(json.dumps(results, indent=2))
        print(f"\n  wrote {args.out}")


if __name__ == "__main__":
    main()
