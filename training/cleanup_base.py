#!/usr/bin/env python3
"""
cleanup_base.py

Shared logic for all spec cleanup scripts (sdei, drtm, scmi, ffa).
Each spec defines its LAYER1_PATH, GENERATED_PATH, OUTPUT_PATH, COMMANDS,
and optional extra verus-ops regex and calls run_cleanup().
"""

import re
import os


# ---------------------------------------------------------------------------
# Body extraction
# ---------------------------------------------------------------------------

def extract_fn_body(text: str, brace_start: int) -> tuple[str, bool]:
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
    return text[brace_start:], True


# ---------------------------------------------------------------------------
# Body cleaning
# ---------------------------------------------------------------------------

def clean_body(body: str, truncated: bool, extra_ops: str = "") -> str:
    """
    1. Deduplicate consecutive identical non-empty lines.
    2. Replace lines with > 5 ==> occurrences with true // [SIMPLIFIED].
    3. If truncated: drop last incomplete line, add // [TRUNCATED] true }.
    4. Fix unmatched parentheses in the last code line.
    extra_ops: additional regex alternates for spec-specific Verus keywords.
    """
    lines = body.split('\n')

    seen: set[str] = set()
    deduped: list[str] = []
    for ln in lines:
        s = ln.strip()
        if s:
            if s in seen:
                continue
            seen.add(s)
        deduped.append(ln)

    out: list[str] = []
    for ln in deduped:
        if ln.count('==>') > 5:
            indent = len(ln) - len(ln.lstrip())
            out.append(' ' * indent + 'true // [DEEP_NESTING_SIMPLIFIED]')
        else:
            out.append(ln)

    if truncated:
        ops_pattern = (
            r'(==>|&&|\|\||::|\.is_Ok\(\)|\.is_Err\(\)|== true|== false'
        )
        # Add spec-specific keywords
        spec_kw = (
            r'|\bCpuIs|\bAddrIs|\bPSCI_|\bCallerIs|\bTrustedOs'  # PSCI
            r'|\bEventIs|\bPeIs|\bInterruptIs|\bSDEI_'            # SDEI
            r'|\bDrtmIs|\bTcbIs|\bDlmeIs|\bDRTM_'                # DRTM
            r'|\bAgentIs|\bPowerDomainIs|\bProtocolIs|\bSCMI_'    # SCMI
            r'|\bPartitionIs|\bMemHandle|\bFFA_|\bNotificationIs' # FF-A
        )
        if extra_ops:
            ops_pattern += '|' + extra_ops
        ops_pattern += spec_kw + ')'
        _verus_ops = re.compile(ops_pattern)

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
                if s.endswith(')'):
                    last_code_idx = idx2

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

    # Fix unmatched parens
    inner = body_out[1:-1] if (body_out.startswith('{') and body_out.endswith('}')) else body_out
    balance = 0
    for ch in inner:
        if ch == '(':
            balance += 1
        elif ch == ')':
            balance -= 1

    if balance < 0:
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
        closing = ')' * balance
        body_out = body_out[:-1].rstrip() + '\n    ' + closing + '\n}'

    return body_out


# ---------------------------------------------------------------------------
# BSON / JSON "value" extraction
# ---------------------------------------------------------------------------

def extract_bson_value(segment: str) -> str | None:
    m = re.search(r'"value":\s*"((?:[^"\\]|\\.)*)"\s*\}', segment, re.DOTALL)
    if m:
        raw = m.group(1)
        raw = raw.replace('\\n', '\n').replace('\\"', '"').replace('\\\\', '\\')
        if 'pub open spec fn' in raw:
            return raw

    m2 = re.search(
        r'"value":\s*"(pub open spec fn.*?)\n\s*\"\s*\}',
        segment,
        re.DOTALL,
    )
    if m2:
        return m2.group(1).rstrip('"').rstrip()

    return None


# ---------------------------------------------------------------------------
# Signature regex
# ---------------------------------------------------------------------------

_SIG_RE = re.compile(
    r'pub\s+open\s+spec\s+fn\s+(\w+)\s*'
    r'\((?:[^(){};]|\([^)]*\))*\)'
    r'\s*->\s*\w+\s*\{',
    re.DOTALL,
)


# ---------------------------------------------------------------------------
# Segment-level function extraction
# ---------------------------------------------------------------------------

def looks_like_code(text: str, extra_kw: str = "") -> bool:
    base = r'\b(CpuIs|AddrIs|result|PSCI_|CallerIs|TrustedOs'
    spec = r'|EventIs|PeIs|InterruptIs|SDEI_'
    spec += r'|DrtmIs|TcbIs|DlmeIs|DRTM_'
    spec += r'|AgentIs|PowerDomainIs|ProtocolIs|SCMI_'
    spec += r'|PartitionIs|MemHandle|FFA_|NotificationIs'
    spec += r'|==>|&&|PSCI_SUCCESS|SCMI_SUCCESS|SDEI_SUCCESS|DRTM_SUCCESS|FFA_SUCCESS'
    if extra_kw:
        spec += '|' + extra_kw
    pat = re.compile(base + spec + r')\b')
    return bool(pat.search(text[:600]))


def segment_clean(seg: str) -> str:
    seg = re.sub(r'^(assistant|user)\s*$', '', seg, flags=re.MULTILINE)
    seg = re.sub(r'^\s*verus!\s*\{', '', seg, flags=re.MULTILINE)
    seg = re.sub(r'^##\s.*$', '', seg, flags=re.MULTILINE)
    return seg


def extract_from_segment(seg: str, extra_ops: str = "") -> dict[str, str]:
    results: dict[str, str] = {}

    if '"value"' in seg and 'pub open spec fn' in seg:
        value_text = extract_bson_value(seg)
        if value_text:
            inner = extract_from_segment(value_text, extra_ops)
            results.update(inner)
            if inner:
                return results

    seg2 = segment_clean(seg)
    seg2 = seg2.replace('\\n', '\n').replace('\\"', '"')

    for m in _SIG_RE.finditer(seg2):
        fn_name = m.group(0).split('fn')[1].split('(')[0].strip()
        brace_pos = m.end() - 1
        body, truncated = extract_fn_body(seg2, brace_pos)

        if len(body) > 20000:
            sig = m.group(0)[:-1].strip()
            full_fn = f"{sig} {{\n    true // [TRUNCATED — body too large]\n}}"
            if looks_like_code(full_fn, extra_ops):
                results[fn_name] = full_fn
            continue

        cleaned = clean_body(body, truncated, extra_ops)
        sig = m.group(0)[:-1].strip()
        full_fn = f"{sig} {cleaned}"

        if looks_like_code(full_fn, extra_ops):
            results[fn_name] = full_fn

    return results


def extract_all_functions(raw_text: str, extra_ops: str = "") -> dict[str, str]:
    segments = re.split(r'</?think>', raw_text)
    all_fns: dict[str, str] = {}

    for seg in segments:
        if 'pub open spec fn' not in seg:
            continue
        fns = extract_from_segment(seg, extra_ops)
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
    # Normalized: strip underscores for fuzzy matching (handles PascalCase model output)
    cmd_norm = cmd_lower.replace('_', '')

    if target in fn_dict:
        return fn_dict[target]

    for fn_name, fn_text in fn_dict.items():
        fl = fn_name.lower()
        fl_norm = fl.replace('_', '')
        if fl == target:
            return fn_text
        # cmd_lower + _spec embedded in fn_name (handles prefixes like "scmi_base_")
        if (cmd_lower + '_spec') in fl:
            return fn_text
        # Normalized exact: strip underscores, must match exactly (handles PascalCase)
        if fl_norm == cmd_norm + 'spec':
            return fn_text
        # PascalCase exact: uppercase fn_name must match "CMD_SPEC"
        if fn_name.upper() == cmd + '_SPEC':
            return fn_text

    return None


def make_stub(cmd: str, prefix: str = "CMD") -> str:
    name = cmd_to_spec_name(cmd)
    return (
        f"// [STUB: {cmd} — model output failed]\n"
        f"pub open spec fn {name}(s: S) -> bool {{\n"
        f"    true\n"
        f"}}"
    )


# ---------------------------------------------------------------------------
# Layer 1 preamble loader
# ---------------------------------------------------------------------------

def load_layer1(layer1_path: str) -> str:
    with open(layer1_path) as f:
        text = f.read()
    text = re.sub(r'\n\}\s*//\s*verus!\s*$', '', text)
    return text.rstrip()


# ---------------------------------------------------------------------------
# Main cleanup runner
# ---------------------------------------------------------------------------

def run_cleanup(
    layer1_path: str,
    generated_path: str,
    output_path: str,
    commands: list[str],
    extra_ops: str = "",
) -> None:
    layer1_preamble = load_layer1(layer1_path)

    with open(generated_path) as f:
        generated = f.read()

    print(f"Extracting functions from {os.path.basename(generated_path)} ...")
    fn_dict = extract_all_functions(generated, extra_ops)
    print(f"Total unique functions extracted: {len(fn_dict)}\n")

    parts = [layer1_preamble, "\n\n// --- Commands ---"]
    found: list[str] = []
    stubbed: list[str] = []
    used_fn_texts: set[int] = set()  # track by id() to avoid emitting same fn twice

    for cmd in sorted(commands):
        fn_text = find_fn_for_cmd(cmd, fn_dict)
        if fn_text and id(fn_text) not in used_fn_texts:
            parts.append(f"\n{fn_text}\n")
            found.append(cmd)
            used_fn_texts.add(id(fn_text))
        elif fn_text and id(fn_text) in used_fn_texts:
            # Already emitted — add a stub referencing the real command
            parts.append(f"\n{make_stub(cmd)}\n")
            stubbed.append(cmd)
        else:
            parts.append(f"\n{make_stub(cmd)}\n")
            stubbed.append(cmd)

    parts.append("\n} // verus!\n")
    output = "\n".join(parts)

    with open(output_path, 'w') as f:
        f.write(output)

    print(f"Written : {output_path}  ({len(output):,} chars, "
          f"{output.count(chr(10))} lines)")
    print(f"From model  : {len(found):2d} commands — {found}")
    print(f"Stubbed out : {len(stubbed):2d} commands — {stubbed}")
