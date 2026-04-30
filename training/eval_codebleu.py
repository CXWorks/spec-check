#!/usr/bin/env python3
"""
eval_codebleu.py — compute CodeBLEU between generated and gold command specs.

Strategy:
  - Gold: specs/alp14/{cmd}_spec.rs  (one file per command)
  - Generated: alp14_generated.rs     (extract fn blocks per command)
  - Format each file with verusfmt if possible (skip on error)
  - Report CodeBLEU overall + per-command breakdown
"""

import os, re, subprocess, sys
from pathlib import Path
from codebleu import calc_codebleu

BASE = Path(__file__).parent
VERUSFMT = BASE / "verusfmt/target/release/verusfmt"
GOLD_DIR = BASE / "specs/alp14"
GEN_FILE = BASE / "alp14_generated.rs"

# ── 1. Load gold per-command specs ─────────────────────────────────────────
gold = {}
for p in sorted(GOLD_DIR.glob("*_spec.rs")):
    cmd = p.stem[:-5].upper()  # rmi_data_create_spec → RMI_DATA_CREATE
    gold[cmd] = p.read_text().strip()

print(f"[GOLD] {len(gold)} command specs in specs/alp14/")

# ── 2. Extract generated per-command specs ──────────────────────────────────
gen_text = GEN_FILE.read_text()

# Each generated cmd block: "pub open spec fn {name}_spec(..." until next "pub open spec fn" or end
fn_pat = re.compile(
    r'(pub open spec fn (\w+)_spec\b.*?)(?=\npub open spec fn |\n// ---|\Z)',
    re.DOTALL
)
generated = {}
for m in fn_pat.finditer(gen_text):
    body = m.group(1).strip()
    raw_name = m.group(2).upper()
    generated[raw_name] = body

print(f"[GEN]  {len(generated)} command specs extracted from alp14_generated.rs")

# Normalize key matching: strip underscores so "PSCI_CPU_OFF" matches "PSCICPUOFF"
def norm(k): return k.replace("_", "")
gold_norm = {norm(k): k for k in gold}
gen_norm  = {norm(k): k for k in generated}
remapped = {}
for nk, gk in gen_norm.items():
    canonical = gold_norm.get(nk, gk)   # map to gold key if possible
    remapped[canonical] = generated[gk]
generated = remapped
print(f"[GEN]  {len(generated)} after key normalization")

# ── 3. Attempt verusfmt on individual files ─────────────────────────────────
def try_format(code: str) -> str:
    """Run verusfmt on code string; return formatted or original on error."""
    if not VERUSFMT.exists():
        return code
    import tempfile
    with tempfile.NamedTemporaryFile(suffix=".rs", mode="w", delete=False) as f:
        # wrap in verus! block so verusfmt can parse it
        f.write(f"use vstd::prelude::*;\nverus! {{\n{code}\n}}\n")
        fname = f.name
    try:
        r = subprocess.run([str(VERUSFMT), fname], capture_output=True, timeout=10)
        if r.returncode == 0:
            txt = Path(fname).read_text()
            # strip the wrapper — verusfmt emits "} // verus!" at end
            inner = re.search(r'verus!\s*\{(.*)\}\s*(?://[^\n]*)?\s*$', txt, re.DOTALL)
            if inner:
                return inner.group(1).strip()
    except Exception:
        pass
    finally:
        os.unlink(fname)
    return code

# ── 4. Match gold ↔ generated and format ────────────────────────────────────
matched, only_gold, only_gen = [], [], []
refs, hyps, names = [], [], []

for cmd, gtext in sorted(gold.items()):
    if cmd in generated:
        matched.append(cmd)
        g_fmt = try_format(gtext)
        h_fmt = try_format(generated[cmd])
        refs.append(g_fmt)
        hyps.append(h_fmt)
        names.append(cmd)
    else:
        only_gold.append(cmd)

for cmd in sorted(generated):
    if cmd not in gold:
        only_gen.append(cmd)

print(f"\n[MATCH] {len(matched)} matched | {len(only_gold)} gold-only | {len(only_gen)} gen-only")
if only_gold[:5]:
    print(f"  Gold-only (first 5): {only_gold[:5]}")
if only_gen[:5]:
    print(f"  Gen-only  (first 5): {only_gen[:5]}")

# ── 5. CodeBLEU ─────────────────────────────────────────────────────────────
if not refs:
    print("[ERROR] No matched pairs — cannot compute CodeBLEU")
    sys.exit(1)

result = calc_codebleu(
    [[r] for r in refs],
    hyps,
    lang="rust",
    weights=(0.25, 0.25, 0.25, 0.25),
)

print("\n" + "="*55)
print("  CodeBLEU Results (alp14, model vs gold)")
print("="*55)
print(f"  CodeBLEU          : {result['codebleu']:.4f}")
print(f"  ngram_match       : {result['ngram_match_score']:.4f}")
print(f"  weighted_ngram    : {result['weighted_ngram_match_score']:.4f}")
print(f"  syntax_match      : {result['syntax_match_score']:.4f}")
print(f"  dataflow_match    : {result['dataflow_match_score']:.4f}")
print(f"  Matched pairs     : {len(matched)} / {len(gold)} gold commands")
print("="*55)

# ── 6. Per-command breakdown (bottom 10) ────────────────────────────────────
per_cmd = []
for r, h, n in zip(refs, hyps, names):
    s = calc_codebleu([[r]], [h], lang="rust")
    per_cmd.append((n, s['codebleu']))

per_cmd.sort(key=lambda x: x[1])
print("\n  Worst 10 commands:")
for n, s in per_cmd[:10]:
    print(f"    {s:.3f}  {n}")

print("\n  Best 10 commands:")
for n, s in per_cmd[-10:][::-1]:
    print(f"    {s:.3f}  {n}")
