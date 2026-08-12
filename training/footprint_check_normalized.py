#!/usr/bin/env python3
"""Re-check the footprint-check candidates flagged by scope_rule_check_ourcode.py
against a *semantically normalized* form of the spec's declared footprint,
instead of naive substring matching.

Why: our generated code explicitly threads state through every helper call
(`RttWalk(new_s, RealmAt(new_s, rd), ipa, level, tree)`), while the spec's
footprint table is written in terms of bound Context variables with no state
argument at all (`realm: RealmAt(rd)` ... `footprint: realm_state: realm.state`).
A plain substring match between the two styles produces heavy false-positive
noise (see BASELINE1_OUR_CODE_RULE_CHECK.md). This script:

  1. Parses each command's Context table (bound var -> defining expression)
     from the `scope --mode raw` dump (previously discarded by
     scope_rule_check_ourcode.py's parse_raw()).
  2. Parses footprint values and our own generated code's flagged LHS
     expressions into a small AST (function calls / field access / index /
     +- / `as` casts).
  3. Expands footprint values by substituting Context bindings recursively.
  4. Canonicalizes both sides: strips `old_s`/`new_s` state-threading
     arguments, drops `as TYPE` casts, applies a small primitive-name alias
     table (only relevant for eac5/rel0, where our model sometimes uses the
     alp11+ naming convention -- e.g. RealmAt instead of Realm).
  5. Compares canonical ASTs structurally (function name + field/index shape
     must match; scalar leaf arguments like level/tree/index constants are
     treated as wildcards, since footprint declarations don't parameterize
     over them).

Limitations (documented, not solved here): does not know that
`RttEntry(s, Rtt(s, X), Y).addr` is the same location as `X-derived .rtte.addr`
in the gold formalization -- that's an axiom-level equivalence between two
different but intended-equivalent formalizations, not a naming/state issue,
and is out of scope for this normalization pass.
"""
import argparse
import re
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from scope_rule_check_ourcode import (  # noqa: E402
    extract_our_clauses,
    footprint_check as naive_footprint_check,
)

BASE = Path(__file__).resolve().parent.parent

CMD_MARKER = re.compile(r"\n([A-Z][A-Z0-9_]+) command\n-{10,}\n")
LIST_LINE = re.compile(r"^\[.*\]$")

# Only matters for eac5/rel0, where our model sometimes leaks the alp11+
# naming convention into an older spec version.
NAME_ALIASES = {
    "RealmAt": "Realm",
}


# ---------------------------------------------------------------------------
# raw dump parsing (context-aware -- scope_rule_check_ourcode.parse_raw()
# throws the context lines away; this keeps them)
# ---------------------------------------------------------------------------

def parse_raw_with_context(text):
    text = "\n" + text
    matches = list(CMD_MARKER.finditer(text))
    cmds = {}
    for i, m in enumerate(matches):
        name = m.group(1)
        start = m.end()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(text)
        body = text[start:end]
        parts = body.split("-" * 44 + "\n")

        ctx_block_lines = [l for l in parts[0].split("\n") if l.strip()]
        list_lines_idx = [j for j, l in enumerate(ctx_block_lines) if LIST_LINE.match(l.strip())]
        outputs = []
        context = {}
        if list_lines_idx:
            outputs_line = ctx_block_lines[list_lines_idx[-1]]
            try:
                import ast as _ast
                outputs = _ast.literal_eval(outputs_line.strip())
            except Exception:
                outputs = []
            ctx_start = list_lines_idx[0] + 1 if len(list_lines_idx) >= 1 else 0
            ctx_end = list_lines_idx[-1]
            for l in ctx_block_lines[ctx_start:ctx_end]:
                if ":" in l:
                    var, expr = l.split(":", 1)
                    context[var.strip()] = expr.strip()

        footprints = []
        if len(parts) > 3:
            for l in parts[3].split("\n"):
                l = l.strip().rstrip("=").strip()
                if l and ": " in l:
                    fid, fval = l.split(": ", 1)
                    footprints.append((fid.strip(), fval.strip()))

        cmds[name] = {"outputs": outputs, "context": context, "footprints": footprints}
    return cmds


# ---------------------------------------------------------------------------
# tiny expression parser: IDENT | NUM | call(...) | .field | [index] | as TYPE | + -
# ---------------------------------------------------------------------------

TOKEN_RE = re.compile(r"\s*(?:(0x[0-9a-fA-F_]+|\d[\d_]*)|([A-Za-z_][A-Za-z0-9_]*)|([(),.\[\]+\-]))")


def tokenize(s):
    # SCOPE's PDF table extraction doubles array-index brackets (`x[[0]]`
    # instead of `x[0]`) -- collapse before tokenizing.
    s = s.replace("[[", "[").replace("]]", "]")
    toks = []
    i = 0
    while i < len(s):
        m = TOKEN_RE.match(s, i)
        if not m or m.end() == i:
            if s[i].isspace():
                i += 1
                continue
            raise ParseError(f"bad token at {i} in {s!r}: {s[i:i+20]!r}")
        i = m.end()
        num, ident, punct = m.groups()
        if num:
            toks.append(("NUM", num))
        elif ident:
            toks.append(("ID", ident))
        elif punct:
            toks.append(("PUNCT", punct))
    return toks


class ParseError(Exception):
    pass


class Parser:
    def __init__(self, toks):
        self.toks = toks
        self.i = 0

    def peek(self):
        return self.toks[self.i] if self.i < len(self.toks) else None

    def next(self):
        t = self.peek()
        self.i += 1
        return t

    def expect(self, kind, val=None):
        t = self.next()
        if t is None or t[0] != kind or (val is not None and t[1] != val):
            raise ParseError(f"expected {kind} {val}, got {t}")
        return t

    def additive(self):
        left = self.cast_expr()
        while self.peek() in (("PUNCT", "+"), ("PUNCT", "-")):
            op = self.next()[1]
            right = self.cast_expr()
            left = ("binop", op, left, right)
        return left

    def cast_expr(self):
        e = self.postfix()
        while self.peek() == ("ID", "as"):
            self.next()
            t = self.expect("ID")
            e = ("cast", e, t[1])
        return e

    def postfix(self):
        e = self.atom()
        while True:
            t = self.peek()
            if t == ("PUNCT", "."):
                self.next()
                name = self.expect("ID")[1]
                e = ("field", e, name)
            elif t == ("PUNCT", "("):
                self.next()
                args = []
                if self.peek() != ("PUNCT", ")"):
                    args.append(self.additive())
                    while self.peek() == ("PUNCT", ","):
                        self.next()
                        args.append(self.additive())
                self.expect("PUNCT", ")")
                if e[0] != "id":
                    raise ParseError(f"call on non-identifier: {e}")
                e = ("call", e[1], args)
            elif t == ("PUNCT", "["):
                self.next()
                idx = self.additive()
                self.expect("PUNCT", "]")
                e = ("index", e, idx)
            else:
                break
        return e

    def atom(self):
        t = self.next()
        if t is None:
            raise ParseError("unexpected end of input")
        if t[0] == "NUM":
            return ("num", t[1])
        if t[0] == "ID":
            return ("id", t[1])
        if t == ("PUNCT", "("):
            e = self.additive()
            self.expect("PUNCT", ")")
            return e
        if t == ("PUNCT", "-"):
            e = self.cast_expr()
            return ("neg", e)
        raise ParseError(f"unexpected token {t}")


def parse_expr(s):
    toks = tokenize(s)
    p = Parser(toks)
    e = p.additive()
    if p.peek() is not None:
        raise ParseError(f"trailing tokens: {p.toks[p.i:]}")
    return e


# ---------------------------------------------------------------------------
# canonicalization
# ---------------------------------------------------------------------------

def expand(node, context, depth=0):
    if depth > 25:
        return node
    kind = node[0]
    if kind == "id":
        if node[1] in context and node[1] not in ("old_s", "new_s"):
            try:
                sub = parse_expr(context[node[1]])
            except ParseError:
                return node
            return expand(sub, context, depth + 1)
        return node
    if kind == "call":
        return ("call", node[1], [expand(a, context, depth + 1) for a in node[2]])
    if kind == "field":
        return ("field", expand(node[1], context, depth + 1), node[2])
    if kind == "index":
        return ("index", expand(node[1], context, depth + 1), expand(node[2], context, depth + 1))
    if kind == "binop":
        return ("binop", node[1], expand(node[2], context, depth + 1), expand(node[3], context, depth + 1))
    if kind == "cast":
        return ("cast", expand(node[1], context, depth + 1), node[2])
    if kind == "neg":
        return ("neg", expand(node[1], context, depth + 1))
    return node


def erase_state(node):
    kind = node[0]
    if kind == "call":
        name, args = node[1], node[2]
        if args and args[0][0] == "id" and args[0][1] in ("old_s", "new_s"):
            args = args[1:]
        return ("call", name, [erase_state(a) for a in args])
    if kind == "field":
        return ("field", erase_state(node[1]), node[2])
    if kind == "index":
        return ("index", erase_state(node[1]), erase_state(node[2]))
    if kind == "binop":
        return ("binop", node[1], erase_state(node[2]), erase_state(node[3]))
    if kind == "cast":
        return ("cast", erase_state(node[1]), node[2])
    if kind == "neg":
        return ("neg", erase_state(node[1]))
    if kind == "id" and node[1] in ("old_s", "new_s"):
        return ("id", "__state__")
    return node


def apply_aliases(node):
    kind = node[0]
    if kind == "call":
        name = NAME_ALIASES.get(node[1], node[1])
        return ("call", name, [apply_aliases(a) for a in node[2]])
    if kind == "field":
        return ("field", apply_aliases(node[1]), node[2])
    if kind == "index":
        return ("index", apply_aliases(node[1]), apply_aliases(node[2]))
    if kind == "binop":
        return ("binop", node[1], apply_aliases(node[2]), apply_aliases(node[3]))
    if kind in ("cast", "neg"):
        # cast type info isn't semantically meaningful for footprint matching
        return apply_aliases(node[1]) if kind == "cast" else ("neg", apply_aliases(node[1]))
    return node


def canon_footprint(expr_str, context):
    node = parse_expr(expr_str)
    node = expand(node, context)
    node = erase_state(node)
    node = apply_aliases(node)
    return node


def canon_ours(expr_str):
    node = parse_expr(expr_str)
    node = erase_state(node)
    node = apply_aliases(node)
    return node


def is_scalar_leaf(node):
    if node[0] in ("num", "id"):
        return True
    if node[0] in ("cast", "neg"):
        return is_scalar_leaf(node[1])
    return False


def rtte_equiv(a, b):
    """`X.rtte` (our field-access style) and `RttEntryAt(RttAt(X.rtt_addr), idx)`
    (the footprint table's composed style) name the same RTT-entry object --
    this is a real identity in the spec's own vocabulary (SCOPE's footprint
    table and its success/failure conditions use two different but
    equivalent spellings for the same thing), not a naming-drift artifact of
    our model."""
    if a[0] == "field" and a[2] == "rtte" and b[0] == "call" and b[1] in ("RttEntryAt", "RttEntry"):
        if not b[2]:
            return False
        rtt_arg = b[2][0]
        # Two equivalent spellings seen in the spec's own footprint tables:
        #   RttEntryAt(RttAt(W.rtt_addr), idx)  -- RttAt: Address -> RmmRtt object first
        #   RttEntry(W.rtt_addr, idx)           -- RttEntry takes the Address directly
        if rtt_arg[0] == "call" and rtt_arg[1] in ("RttAt", "Rtt") and len(rtt_arg[2]) == 1:
            addr_field = rtt_arg[2][0]
        else:
            addr_field = rtt_arg
        if addr_field[0] == "field" and addr_field[2] == "rtt_addr":
            return struct_equal(a[1], addr_field[1])
    return False


def struct_equal(a, b):
    if rtte_equiv(a, b) or rtte_equiv(b, a):
        return True
    if is_scalar_leaf(a) and is_scalar_leaf(b):
        return True
    if a[0] != b[0]:
        return False
    if a[0] == "call":
        if a[1] != b[1] or len(a[2]) != len(b[2]):
            return False
        return all(struct_equal(x, y) for x, y in zip(a[2], b[2]))
    if a[0] == "field":
        return struct_equal(a[1], b[1]) and a[2] == b[2]
    if a[0] == "index":
        return struct_equal(a[1], b[1])
    if a[0] == "binop":
        return a[1] == b[1] and struct_equal(a[2], b[2]) and struct_equal(a[3], b[3])
    return a == b


# ---------------------------------------------------------------------------
# driver
# ---------------------------------------------------------------------------

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--raw-file", required=True)
    ap.add_argument("--gen-dir", required=True)
    ap.add_argument("--only", default="")
    args = ap.parse_args()

    raw_file = Path(args.raw_file)
    gen_dir = Path(args.gen_dir)

    cmds = parse_raw_with_context(raw_file.read_text())
    if args.only:
        allow = {n.strip().lower() for n in args.only.split(",") if n.strip()}
        cmds = {n: v for n, v in cmds.items() if n.lower() in allow}

    print(f"Checking {len(cmds)} commands from {raw_file} against {gen_dir}\n")

    still_flagged = []
    resolved = []
    unparseable = []
    preserved = []

    for name, info in sorted(cmds.items()):
        dirname = name.lower()
        rs_path = gen_dir / dirname / "generated.formatted.rs"
        if not rs_path.exists():
            continue
        rs_text = rs_path.read_text()
        clauses, body = extract_our_clauses(rs_text)

        naive_flagged = naive_footprint_check(name, clauses, info["outputs"], info["footprints"])
        if not naive_flagged:
            continue

        # pre-compute canonical footprint values for this command
        canon_fp_values = []
        for fid, fval in info["footprints"]:
            try:
                canon_fp_values.append((fid, canon_footprint(fval, info["context"])))
            except ParseError:
                pass

        for clause in naive_flagged:
            if "==" not in clause:
                still_flagged.append((name, clause, "no '==' -- not an equality clause"))
                continue
            # `==` is symmetric -- our model sometimes writes the modification
            # target on the right (`RimExtendData(...) == realm.measurements[0]`
            # instead of the spec's `realm.measurements[0] == RimExtendData(...)`).
            # Try both sides rather than assuming the target is always on the left.
            sides = [s.strip() for s in clause.split("==", 1)]

            # `X(..., new_s, ...) == X(..., old_s, ...)` (same call/field shape,
            # only the state argument differs) asserts *no change* -- it can
            # never be a footprint violation, whether or not X is declared,
            # since it doesn't claim anything was modified. This is the
            # "unchanged on failure" / "this field is untouched" boilerplate
            # our model emits for every non-modified field.
            if len(sides) == 2:
                try:
                    a_raw = parse_expr(sides[0])
                    b_raw = parse_expr(sides[1])
                    if a_raw != b_raw and erase_state(a_raw) == erase_state(b_raw):
                        preserved.append((name, clause))
                        continue
                except ParseError:
                    pass

            match = None
            unparse_err = None
            any_parsed = False
            for side_str in sides:
                try:
                    our_canon = canon_ours(side_str)
                    any_parsed = True
                except ParseError as e:
                    unparse_err = str(e)
                    continue

                # A footprint entry often names a whole record (e.g. `rtte: RttEntryAt(...)`)
                # while our code narrows to one of its fields (`....rtte.state`). That's
                # still within the declared footprint, so peel trailing .field/[index]
                # layers off our LHS and check each peeled level too.
                candidates = [our_canon]
                peeled = our_canon
                while peeled[0] in ("field", "index"):
                    peeled = peeled[1]
                    candidates.append(peeled)

                for cand in candidates:
                    for fid, fp_canon in canon_fp_values:
                        if struct_equal(cand, fp_canon):
                            match = fid
                            break
                    if match:
                        break
                if match:
                    break

            if match is None and not any_parsed:
                unparseable.append((name, clause, unparse_err or "unparseable"))
                continue

            if match:
                resolved.append((name, clause, match))
            else:
                still_flagged.append((name, clause, "no matching footprint entry after normalization"))

    print(f"=== Resolved by normalization: {len(resolved)} (false positives of the naive check) ===")
    for name, clause, fid in resolved:
        print(f"{name}: matches footprint '{fid}'")
        print(f"  {clause}")

    print(f"\n=== Preservation clauses (X_new == X_old, not a modification claim): {len(preserved)} ===")
    for name, clause in preserved:
        print(f"{name}")
        print(f"  {clause}")

    print(f"\n=== Still flagged after normalization: {len(still_flagged)} ===")
    for name, clause, reason in still_flagged:
        print(f"{name}: {reason}")
        print(f"  {clause}")

    if unparseable:
        print(f"\n=== Unparseable (kept flagged, needs manual look): {len(unparseable)} ===")
        for name, clause, err in unparseable:
            print(f"{name}: {err}")
            print(f"  {clause}")


if __name__ == "__main__":
    main()
