#!/usr/bin/env python3
"""
cleanup_psci.py

Post-process psci_generated.rs to fix model output artifacts and produce
a syntactically valid psci_generated_clean.rs.

Artifacts handled:
  1. <think>...</think> CoT blocks — used as segment delimiters
  2. "assistant" / "user" role tokens
  3. JSON/BSON "value" wrappers (literal multiline or \\n-escaped)
  4. Extra inner  verus! { ... }  wrappers around functions
  5. Repeated consecutive identical && clause lines
  6. Deeply nested ==> chains (> 5 occurrences on one line → collapse to true)
  7. Truncated function bodies (missing closing })
  8. Verbatim spec PDF text instead of Verus code
  9. Layer1's own  } // verus!  closing

Strategy: split the file on <think>/<think> tags into segments, extract
functions from each segment independently (avoids brace-counting consuming
content from adjacent segments).

Output: psci_generated_clean.rs
"""

import re
import os

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

LAYER1_PATH    = os.path.join(BASE_DIR, "boilerplate", "layer1_psci.rs")
GENERATED_PATH = os.path.join(BASE_DIR, "psci_generated.rs")
OUTPUT_PATH    = os.path.join(BASE_DIR, "psci_generated_clean.rs")

COMMANDS = sorted([
    "AFFINITY_INFO",
    "CPU_DEFAULT_SUSPEND",
    "CPU_FREEZE",
    "CPU_OFF",
    "CPU_ON",
    "CPU_SUSPEND",
    "MEM_PROTECT_CHECK_RANGE",
    "MEM_PROTECT",
    "MIGRATE_INFO_TYPE",
    "MIGRATE_INFO_UP_CPU",
    "MIGRATE",
    "NODE_HW_STATE",
    "PSCI_FEATURES",
    "PSCI_SET_SUSPEND_MODE",
    "PSCI_STAT_COUNT",
    "PSCI_STAT_RESIDENCY",
    "PSCI_VERSION",
    "SYSTEM_OFF2",
    "SYSTEM_OFF",
    "SYSTEM_RESET2",
    "SYSTEM_RESET",
    "SYSTEM_SUSPEND",
])


# ---------------------------------------------------------------------------
# Layer 1 preamble
# ---------------------------------------------------------------------------

def load_layer1() -> str:
    with open(LAYER1_PATH) as f:
        text = f.read()
    # Remove trailing `} // verus!` — commands will live inside the same block
    text = re.sub(r'\n\}\s*//\s*verus!\s*$', '', text)
    return text.rstrip()


# ---------------------------------------------------------------------------
# Signature regex that handles  Result<(), T>  in parameter lists
# ---------------------------------------------------------------------------
#
# [^{;()]*  — characters outside parens
# (?:\([^)]*\))*  — optional inner () groups (e.g. Result<()>)
# Combined: parameter list that allows one level of nested parens.

_SIG_RE = re.compile(
    r'pub\s+open\s+spec\s+fn\s+(\w+)\s*'
    r'\((?:[^(){};]|\([^)]*\))*\)'   # parameter list, one level nesting
    r'\s*->\s*\w+\s*\{',
    re.DOTALL,
)


# ---------------------------------------------------------------------------
# Body extraction
# ---------------------------------------------------------------------------

def extract_fn_body(text: str, brace_start: int) -> tuple[str, bool]:
    """
    Brace-count extract starting at the `{` at brace_start.
    Returns (body_text, truncated).
    """
    depth = 0
    i = brace_start
    while i < len(text):
        c = text[i]
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                return text[brace_start: i + 1], False
        i += 1
    return text[brace_start:], True   # truncated


# ---------------------------------------------------------------------------
# Body cleaning
# ---------------------------------------------------------------------------

def _paren_balance(text: str) -> int:
    """Return #open_parens - #close_parens in text (ignoring comments/strings)."""
    count = 0
    in_line_comment = False
    for ch in text:
        if ch == '\n':
            in_line_comment = False
            continue
        if in_line_comment:
            continue
        if ch == '/' and count >= 0:   # simplistic: not tracking prev char
            pass
        if ch == '(':
            count += 1
        elif ch == ')':
            count -= 1
    return count


def clean_body(body: str, truncated: bool) -> str:
    """
    1. Deduplicate consecutive identical non-empty lines.
    2. Replace lines with > 5  ==>  occurrences with  true // [SIMPLIFIED].
    3. If truncated:
         - Drop the last incomplete line (unclosed token / truncated mid-word).
         - Add  // [TRUNCATED]\\n    true\\n}.
    4. Fix unmatched parentheses in the last code line.
    """
    lines = body.split('\n')

    # Global dedup: keep first occurrence of each non-empty line content.
    # Model artifacts repeat the same clauses many times (not just consecutively),
    # so we deduplicate across the entire body.
    seen: set[str] = set()
    deduped: list[str] = []
    for ln in lines:
        s = ln.strip()
        if s:
            if s in seen:
                continue
            seen.add(s)
        deduped.append(ln)

    # Flatten deeply nested ==>
    out: list[str] = []
    for ln in deduped:
        if ln.count('==>') > 5:
            indent = len(ln) - len(ln.lstrip())
            out.append(' ' * indent + 'true // [DEEP_NESTING_SIMPLIFIED]')
        else:
            out.append(ln)

    if truncated:
        # Keep only lines that look like Verus code; drop prose/spec text.
        # Strategy: scan forward from the start, keep the last position where
        # we saw a Verus code line, then truncate there.
        _verus_ops = re.compile(
            r'(==>|&&|\|\||::|\.is_Ok\(\)|\.is_Err\(\)|== PSCI_|== true|== false'
            r'|\bCpuIs|\bAddrIs|\bPSCI_|\bCallerIs|\bTrustedOs)',
        )
        last_code_idx = -1
        for idx2, ln in enumerate(out):
            s = ln.strip()
            if not s:
                continue
            if s in ('{', '}'):
                last_code_idx = idx2
            elif s.startswith('//') and 'TRUNCATED' not in s and 'SIMPLIFIED' not in s:
                last_code_idx = idx2
            elif bool(_verus_ops.search(s)):
                # Only count as "complete" if the line ends with closing paren
                # (truncated identifiers end with \w, not )
                if s.endswith(')'):
                    last_code_idx = idx2
            # else: looks like prose or incomplete token — skip

        if last_code_idx >= 0:
            out = out[:last_code_idx + 1]
        else:
            out = []

        body_out = '\n'.join(out).rstrip()
        if not body_out.endswith('}'):
            inner = body_out.lstrip('{').strip()
            if inner:
                body_out += '\n    && true // [TRUNCATED]\n}'
            else:
                body_out = '{\n    true // [TRUNCATED]\n}'
    else:
        body_out = '\n'.join(out)

    # Fix unmatched parens: scan the body between { and } and balance
    inner = body_out[1:-1] if (body_out.startswith('{') and body_out.endswith('}')) else body_out
    balance = 0
    for ch in inner:
        if ch == '(':
            balance += 1
        elif ch == ')':
            balance -= 1

    if balance < 0:
        # Extra closing parens: remove from the last line that has them
        result_lines = body_out.split('\n')
        excess = -balance
        for i in range(len(result_lines) - 1, -1, -1):
            ln = result_lines[i]
            stripped = ln.rstrip()
            removed = 0
            while excess > 0 and stripped.endswith(')') and \
                    not stripped.endswith('})'):
                stripped = stripped[:-1].rstrip()
                excess -= 1
                removed += 1
            if removed:
                result_lines[i] = stripped
                if excess == 0:
                    break
        body_out = '\n'.join(result_lines)
    elif balance > 0:
        # Missing closing parens: add before the final }
        closing = ')' * balance
        body_out = body_out[:-1].rstrip() + '\n    ' + closing + '\n}'

    return body_out


# ---------------------------------------------------------------------------
# BSON / JSON "value" extraction
# ---------------------------------------------------------------------------

def extract_bson_value(segment: str) -> str | None:
    """
    If `segment` contains a BSON / JSON block with a "value" key whose
    value starts with  pub open spec fn , return that value string.

    Handles two cases:
      (a) \\n-escaped:  "value": "pub open spec fn ...\\nline2\\n..."
      (b) Literal multiline:  "value": "pub open spec fn ...\nline2\n..."
    """
    # Try \\n-escaped first
    m = re.search(r'"value":\s*"((?:[^"\\]|\\.)*)"\s*\}', segment, re.DOTALL)
    if m:
        raw = m.group(1)
        raw = raw.replace('\\n', '\n').replace('\\"', '"').replace('\\\\', '\\')
        if 'pub open spec fn' in raw:
            return raw

    # Try literal multiline (model forgot to escape)
    # Pattern: "value": "pub open spec fn <content until the closing `}"` on its own line
    m2 = re.search(
        r'"value":\s*"(pub open spec fn.*?)\n\s*\"\s*\}',
        segment,
        re.DOTALL,
    )
    if m2:
        return m2.group(1).rstrip('"').rstrip()

    return None


# ---------------------------------------------------------------------------
# Segment-level function extraction
# ---------------------------------------------------------------------------

def looks_like_code(text: str) -> bool:
    pat = re.compile(
        r'\b(CpuIs|AddrIs|result|PSCI_|CallerIs|TrustedOs|==>|&&|PSCI_SUCCESS)\b'
    )
    return bool(pat.search(text[:600]))


def segment_clean(seg: str) -> str:
    """Light cleanup of a segment before function extraction."""
    # Remove role tokens
    seg = re.sub(r'^(assistant|user)\s*$', '', seg, flags=re.MULTILINE)
    # Remove inner verus! { wrappers (but not the one in layer1)
    seg = re.sub(r'^\s*verus!\s*\{', '', seg, flags=re.MULTILINE)
    # Remove ## prompt headers that leaked in
    seg = re.sub(r'^##\s.*$', '', seg, flags=re.MULTILINE)
    return seg


def extract_from_segment(seg: str) -> dict[str, str]:
    """Return {fn_name: full_fn_text} for functions found in this segment."""
    results: dict[str, str] = {}

    # --- Case 1: BSON/JSON wrapper ---
    if '"value"' in seg and 'pub open spec fn' in seg:
        value_text = extract_bson_value(seg)
        if value_text:
            # Recurse on the extracted value text
            inner = extract_from_segment(value_text)
            results.update(inner)
            if inner:
                return results

    # --- Case 2: Literal function in segment text ---
    seg2 = segment_clean(seg)

    # Handle \\n-escaped sequences (from JSON strings in segment)
    seg2 = seg2.replace('\\n', '\n').replace('\\"', '"')

    for m in _SIG_RE.finditer(seg2):
        fn_name = m.group(0).split('fn')[1].split('(')[0].strip()
        brace_pos = m.end() - 1  # position of `{`
        body, truncated = extract_fn_body(seg2, brace_pos)

        # Limit: if body is absurdly large, it consumed adjacent content.
        # Produce a minimal stub body immediately (skip clean_body).
        if len(body) > 20000:
            sig = m.group(0)[:-1].strip()
            full_fn = f"{sig} {{\n    true // [TRUNCATED — body too large]\n}}"
            if looks_like_code(full_fn):
                results[fn_name] = full_fn
            continue

        cleaned = clean_body(body, truncated)
        sig = m.group(0)[:-1].strip()  # everything before `{`
        full_fn = f"{sig} {cleaned}"

        if looks_like_code(full_fn):
            results[fn_name] = full_fn

    return results


# ---------------------------------------------------------------------------
# Main extractor
# ---------------------------------------------------------------------------

def extract_all_functions(raw_text: str) -> dict[str, str]:
    """
    Split the raw generated file on <think>/<think> tags, then extract
    functions from each segment.  First occurrence of each function wins.
    """
    # Split on think tags (used as segment delimiters)
    segments = re.split(r'</?think>', raw_text)

    all_fns: dict[str, str] = {}

    for seg in segments:
        if 'pub open spec fn' not in seg:
            continue
        fns = extract_from_segment(seg)
        for fn_name, fn_text in fns.items():
            if fn_name not in all_fns:
                all_fns[fn_name] = fn_text
                truncated_marker = "[TRUNCATED]" if "[TRUNCATED]" in fn_text else ""
                simplified_marker = "[SIMPLIFIED]" if "[SIMPLIFIED]" in fn_text else ""
                flags = " ".join(filter(None, [truncated_marker, simplified_marker]))
                print(f"  extracted: {fn_name}" + (f" [{flags}]" if flags else ""))

    return all_fns


# ---------------------------------------------------------------------------
# Command → function matching
# ---------------------------------------------------------------------------

def cmd_to_spec_name(cmd: str) -> str:
    return cmd.lower() + '_spec'


def find_fn_for_cmd(cmd: str, fn_dict: dict[str, str]) -> str | None:
    target = cmd_to_spec_name(cmd)
    cmd_lower = cmd.lower()

    # Exact match
    if target in fn_dict:
        return fn_dict[target]

    # Case-insensitive scan
    for fn_name, fn_text in fn_dict.items():
        fl = fn_name.lower()
        if fl == target:
            return fn_text
        # e.g. psci_cpu_default_suspend_spec ↔ CPU_DEFAULT_SUSPEND
        if cmd_lower in fl and fl.endswith('_spec'):
            return fn_text
        # CPU_FREEZE_spec (uppercase name) ↔ CPU_FREEZE
        if fn_name.upper().rstrip('_SPEC') == cmd or \
                fn_name.upper() == cmd + '_SPEC':
            return fn_text

    return None


def make_stub(cmd: str) -> str:
    name = cmd_to_spec_name(cmd)
    return (
        f"// [STUB: {cmd} — model output failed]\n"
        f"pub open spec fn {name}(s: S) -> bool {{\n"
        f"    true\n"
        f"}}"
    )


# ---------------------------------------------------------------------------
# Assembly
# ---------------------------------------------------------------------------

def main() -> None:
    layer1_preamble = load_layer1()

    with open(GENERATED_PATH) as f:
        generated = f.read()

    print("Extracting functions from psci_generated.rs ...")
    fn_dict = extract_all_functions(generated)
    print(f"Total unique functions extracted: {len(fn_dict)}\n")

    parts = [layer1_preamble, "\n\n// --- Commands ---"]

    found: list[str] = []
    stubbed: list[str] = []

    for cmd in COMMANDS:
        fn_text = find_fn_for_cmd(cmd, fn_dict)
        if fn_text:
            parts.append(f"\n{fn_text}\n")
            found.append(cmd)
        else:
            parts.append(f"\n{make_stub(cmd)}\n")
            stubbed.append(cmd)

    parts.append("\n} // verus!\n")

    output = "\n".join(parts)

    with open(OUTPUT_PATH, 'w') as f:
        f.write(output)

    print(f"Written : {OUTPUT_PATH}  ({len(output):,} chars, "
          f"{output.count(chr(10))} lines)")
    print(f"From model  : {len(found):2d} commands — {found}")
    print(f"Stubbed out : {len(stubbed):2d} commands — {stubbed}")


if __name__ == "__main__":
    main()
