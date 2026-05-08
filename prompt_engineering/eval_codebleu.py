#!/usr/bin/env python3
"""
Evaluate CodeBLEU for generated alp14 command specs.

Designed to reproduce the reporting style in STATUS.md / verusfmt-retraining-results.md:
- Aggregate: CodeBLEU + 4 sub-metrics + matched commands
- Optional side-by-side comparison (baseline vs fmt)
- Optional per-command best/worst table

Important: verusfmt normalization logic intentionally matches training/pipeline.py:
- binary: training/verusfmt/target/release/verusfmt
- invocation: verusfmt --verus-only <tempfile>
- snippet wrapped with `use vstd::prelude::*;\nverus! { ... }`
- fallback to original snippet on failure
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import tempfile
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, List, Optional

try:
    from codebleu import calc_codebleu
except ImportError as e:
    raise SystemExit(
        "codebleu package is required. Install with: pip install codebleu tree-sitter-rust==0.21.2"
    ) from e


ROOT_DIR = Path(__file__).resolve().parent.parent
VERUSFMT = ROOT_DIR / "training" / "verusfmt" / "target" / "release" / "verusfmt"


@dataclass
class CommandScore:
    command: str
    codebleu: float
    ngram_match_score: float
    weighted_ngram_match_score: float
    syntax_match_score: float
    dataflow_match_score: float


@dataclass
class EvalSummary:
    label: str
    matched: int
    total_gold: int
    codebleu: float
    ngram_match: float
    weighted_ngram: float
    syntax_match: float
    dataflow_match: float
    scores: List[CommandScore]


def fmt_code(code: str) -> str:
    """Match teammate's verusfmt normalization method from training/pipeline.py."""
    if not VERUSFMT.exists():
        return code

    with tempfile.NamedTemporaryFile(suffix=".rs", mode="w", delete=False) as f:
        f.write(f"use vstd::prelude::*;\nverus! {{\n{code}\n}}\n")
        fname = f.name

    try:
        r = subprocess.run(
            [str(VERUSFMT), "--verus-only", fname],
            capture_output=True,
            timeout=10,
        )
        if r.returncode == 0:
            txt = Path(fname).read_text(encoding="utf-8", errors="ignore")
            inner = re.search(r"verus!\s*\{(.*)\}\s*(?://[^\n]*)?\s*$", txt, re.DOTALL)
            if inner:
                return inner.group(1).strip()
    except Exception:
        pass
    finally:
        try:
            os.unlink(fname)
        except OSError:
            pass

    return code


def _extract_function_block(text: str, fn_start: int, brace_start: int) -> str:
    depth = 0
    i = brace_start
    n = len(text)
    while i < n:
        ch = text[i]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[fn_start : i + 1]
        i += 1
    return text[fn_start:]


def parse_generated_specs(generated_file: Path) -> Dict[str, str]:
    """Extract {COMMAND_NAME_UPPER: function_text} from assembled generated .rs file."""
    txt = generated_file.read_text(encoding="utf-8", errors="ignore")
    pattern = re.compile(
        r"pub\s+open\s+spec\s+fn\s+([A-Za-z0-9_]+)_spec\s*\([^\)]*\)\s*->\s*bool\s*\{",
        re.MULTILINE,
    )

    out: Dict[str, str] = {}
    for m in pattern.finditer(txt):
        fn_name = m.group(1)
        cmd = fn_name.upper()
        brace_start = txt.find("{", m.start())
        if brace_start == -1:
            continue
        fn_block = _extract_function_block(txt, m.start(), brace_start)
        out[cmd] = fn_block.strip()

    return out


def parse_gold_specs(gold_dir: Path) -> Dict[str, str]:
    """Load {COMMAND_NAME_UPPER: full_spec_text} from *_spec.rs files."""
    out: Dict[str, str] = {}
    for p in sorted(gold_dir.glob("*_spec.rs")):
        cmd = p.stem[: -len("_spec")].upper()
        out[cmd] = p.read_text(encoding="utf-8", errors="ignore").strip()
    return out


def _avg(vals: List[float]) -> float:
    return (sum(vals) / len(vals)) if vals else 0.0


def evaluate_one(
    label: str,
    generated_file: Path,
    gold_dir: Path,
    use_verusfmt: bool,
) -> EvalSummary:
    gen_map = parse_generated_specs(generated_file)
    gold_map = parse_gold_specs(gold_dir)

    if not gold_map:
        raise RuntimeError(f"No *_spec.rs found under gold dir: {gold_dir}")

    scores: List[CommandScore] = []
    for cmd, gold in gold_map.items():
        gen = gen_map.get(cmd)
        if not gen:
            continue

        gold_in = fmt_code(gold) if use_verusfmt else gold
        gen_in = fmt_code(gen) if use_verusfmt else gen

        result = calc_codebleu(
            references=[[gold_in]],
            predictions=[gen_in],
            lang="rust",
            weights=(0.25, 0.25, 0.25, 0.25),
            tokenizer=None,
        )

        scores.append(
            CommandScore(
                command=cmd,
                codebleu=float(result.get("codebleu", 0.0)),
                ngram_match_score=float(result.get("ngram_match_score", 0.0)),
                weighted_ngram_match_score=float(
                    result.get("weighted_ngram_match_score", 0.0)
                ),
                syntax_match_score=float(result.get("syntax_match_score", 0.0)),
                dataflow_match_score=float(result.get("dataflow_match_score", 0.0)),
            )
        )

    return EvalSummary(
        label=label,
        matched=len(scores),
        total_gold=len(gold_map),
        codebleu=_avg([s.codebleu for s in scores]),
        ngram_match=_avg([s.ngram_match_score for s in scores]),
        weighted_ngram=_avg([s.weighted_ngram_match_score for s in scores]),
        syntax_match=_avg([s.syntax_match_score for s in scores]),
        dataflow_match=_avg([s.dataflow_match_score for s in scores]),
        scores=scores,
    )


def print_aggregate_table(summaries: List[EvalSummary]) -> None:
    print("\n## Aggregate CodeBLEU")
    print("| Model | CodeBLEU | ngram_match | weighted_ngram | syntax_match | dataflow_match | Matched |")
    print("|---|---:|---:|---:|---:|---:|---:|")
    for s in summaries:
        print(
            f"| {s.label} | {s.codebleu:.3f} | {s.ngram_match:.3f} | "
            f"{s.weighted_ngram:.3f} | {s.syntax_match:.3f} | {s.dataflow_match:.3f} | "
            f"{s.matched} / {s.total_gold} |"
        )


def print_best_worst(summary: EvalSummary, top_k: int) -> None:
    if not summary.scores:
        print(f"\nNo matched commands for {summary.label}; skipping per-command table.")
        return

    sorted_scores = sorted(summary.scores, key=lambda x: x.codebleu)
    worst = sorted_scores[:top_k]
    best = list(reversed(sorted_scores[-top_k:]))

    print(f"\n## {summary.label} — Worst {len(worst)}")
    print("| Score | Command |")
    print("|---:|---|")
    for row in worst:
        print(f"| {row.codebleu:.3f} | {row.command} |")

    print(f"\n## {summary.label} — Best {len(best)}")
    print("| Score | Command |")
    print("|---:|---|")
    for row in best:
        print(f"| {row.codebleu:.3f} | {row.command} |")


def detect_default_gold_dir() -> Optional[Path]:
    candidates = [
        ROOT_DIR / "training" / "specs" / "alp14",
        ROOT_DIR / "training-dataset" / "specs" / "alp14",
    ]
    for c in candidates:
        if c.exists():
            return c
    return None


def main() -> None:
    parser = argparse.ArgumentParser(description="Evaluate CodeBLEU for baseline/fmt generated files")
    parser.add_argument("--baseline-file", default=str(ROOT_DIR / "training" / "alp14_generated.rs"))
    parser.add_argument("--fmt-file", default=str(ROOT_DIR / "training" / "alp14_generated_fmt.rs"))
    parser.add_argument("--gold-dir", default=None, help="Directory containing *_spec.rs gold files")
    parser.add_argument("--label-baseline", default="Round 1 — baseline")
    parser.add_argument("--label-fmt", default="Round 2 — fmt models")
    parser.add_argument("--no-verusfmt", action="store_true", help="Disable verusfmt normalization")
    parser.add_argument("--show-per-command", action="store_true", help="Print best/worst command tables")
    parser.add_argument("--top-k", type=int, default=10)
    args = parser.parse_args()

    gold_dir = Path(args.gold_dir) if args.gold_dir else detect_default_gold_dir()
    if not gold_dir or not gold_dir.exists():
        raise SystemExit(
            "Gold directory not found. Use --gold-dir, e.g. --gold-dir training-dataset/specs/alp14"
        )

    use_verusfmt = not args.no_verusfmt

    summaries: List[EvalSummary] = []

    baseline_file = Path(args.baseline_file)
    if baseline_file.exists():
        summaries.append(
            evaluate_one(args.label_baseline, baseline_file, gold_dir, use_verusfmt=use_verusfmt)
        )
    else:
        print(f"[WARN] baseline file not found: {baseline_file}")

    fmt_file = Path(args.fmt_file)
    if fmt_file.exists():
        summaries.append(evaluate_one(args.label_fmt, fmt_file, gold_dir, use_verusfmt=use_verusfmt))
    else:
        print(f"[WARN] fmt file not found: {fmt_file}")

    if not summaries:
        raise SystemExit("No evaluable generated files found. Provide --baseline-file and/or --fmt-file.")

    print_aggregate_table(summaries)

    if args.show_per_command:
        for s in summaries:
            print_best_worst(s, top_k=max(1, args.top_k))


if __name__ == "__main__":
    main()
