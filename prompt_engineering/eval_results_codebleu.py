#!/usr/bin/env python3
"""
Evaluate existing results from results/ab_test/ using CodeBLEU.

Compares raw (unformatted) vs formatted (verusfmt) code for each prompt variant.
Outputs aggregate metrics and per-command analysis.
"""

import json
import sys
from pathlib import Path
from typing import Dict, List, Tuple
from collections import defaultdict

try:
    from codebleu import calc_codebleu as _calc_codebleu
    CODEBLEU_AVAILABLE = True
except ImportError:
    CODEBLEU_AVAILABLE = False
    print("Warning: codebleu package not available. Install with: pip install codebleu")


def compute_codebleu(generated: str, oracle: str) -> float:
    """Compute CodeBLEU score (0.0–1.0) against oracle."""
    if not CODEBLEU_AVAILABLE:
        return 0.0
    if not generated or generated == "ERROR" or not oracle:
        return 0.0
    try:
        result = _calc_codebleu(
            [[oracle]], 
            [generated], 
            lang="rust",
            weights=(0.25, 0.25, 0.25, 0.25)
        )
        return float(result["codebleu"])
    except Exception as e:
        return 0.0


def load_code_file(path: Path) -> str:
    """Load code from a .rs file."""
    if not path.exists():
        return ""
    try:
        return path.read_text().strip()
    except Exception:
        return ""


def evaluate_variant(variant_dir: Path) -> Dict:
    """
    Evaluate one prompt variant across all commands.
    Returns: {
        "variant": variant_name,
        "commands": [
            {
                "command": cmd_name,
                "version": version,
                "raw_codebleu": float,
                "formatted_codebleu": float,
                "improvement": float (formatted - raw),
            },
            ...
        ],
        "aggregate": {
            "avg_raw": float,
            "avg_formatted": float,
            "avg_improvement": float,
            "num_commands": int,
        }
    }
    """
    variant_name = variant_dir.name
    command_results = []
    
    # Iterate over versions (alp14, alp13, etc.)
    for version_dir in sorted(variant_dir.iterdir()):
        if not version_dir.is_dir():
            continue
        
        version = version_dir.name
        
        # Iterate over commands
        for cmd_dir in sorted(version_dir.iterdir()):
            if not cmd_dir.is_dir():
                continue
            
            cmd_name = cmd_dir.name
            
            # Load files
            generated_raw = load_code_file(cmd_dir / "generated.raw.rs")
            generated_fmt = load_code_file(cmd_dir / "generated.formatted.rs")
            oracle_raw = load_code_file(cmd_dir / "oracle.raw.rs")
            oracle_fmt = load_code_file(cmd_dir / "oracle.formatted.rs")
            
            # If we have formatted oracle, use it for comparison
            oracle_ref = oracle_fmt if oracle_fmt else oracle_raw
            
            if not oracle_ref or not generated_raw or not generated_fmt:
                continue
            
            # Compute CodeBLEU for both versions
            codebleu_raw = compute_codebleu(generated_raw, oracle_ref)
            codebleu_fmt = compute_codebleu(generated_fmt, oracle_ref)
            improvement = codebleu_fmt - codebleu_raw
            
            command_results.append({
                "command": cmd_name,
                "version": version,
                "raw_codebleu": codebleu_raw,
                "formatted_codebleu": codebleu_fmt,
                "improvement": improvement,
            })
    
    # Compute aggregates
    if command_results:
        avg_raw = sum(r["raw_codebleu"] for r in command_results) / len(command_results)
        avg_fmt = sum(r["formatted_codebleu"] for r in command_results) / len(command_results)
        avg_imp = sum(r["improvement"] for r in command_results) / len(command_results)
    else:
        avg_raw = avg_fmt = avg_imp = 0.0
    
    return {
        "variant": variant_name,
        "commands": command_results,
        "aggregate": {
            "avg_raw": avg_raw,
            "avg_formatted": avg_fmt,
            "avg_improvement": avg_imp,
            "num_commands": len(command_results),
        }
    }


def print_summary_table(all_results: List[Dict]):
    """Print a summary table of all variants."""
    print("\n" + "=" * 100)
    print("SUMMARY: Raw vs Formatted (verusfmt) CodeBLEU Comparison")
    print("=" * 100)
    print(f"{'Prompt Variant':<20} {'Avg Raw':<12} {'Avg Formatted':<16} {'Improvement':<15} {'# Commands':<12}")
    print("-" * 100)
    
    for result in sorted(all_results, key=lambda r: r["aggregate"]["avg_improvement"], reverse=True):
        variant = result["variant"]
        agg = result["aggregate"]
        print(
            f"{variant:<20} {agg['avg_raw']:>10.4f}  {agg['avg_formatted']:>14.4f}  "
            f"{agg['avg_improvement']:>13.4f}  {agg['num_commands']:>10}"
        )
    
    print("=" * 100)


def print_best_worst(all_results: List[Dict], top_k: int = 5):
    """Print best and worst commands for each variant."""
    print("\n" + "=" * 100)
    print(f"COMMAND RANKINGS: Best and Worst {top_k} (by improvement)")
    print("=" * 100)
    
    for result in all_results:
        variant = result["variant"]
        commands = sorted(
            result["commands"],
            key=lambda c: c["improvement"],
            reverse=True
        )
        
        print(f"\n{variant}:")
        print(f"  Best improvements (formatted > raw):")
        for cmd in commands[:top_k]:
            imp_pct = cmd["improvement"] * 100
            print(
                f"    {cmd['command']:<30} {cmd['raw_codebleu']:>7.4f} → "
                f"{cmd['formatted_codebleu']:>7.4f} (+{imp_pct:6.2f}%)"
            )
        
        print(f"  Worst (formatted < raw):")
        for cmd in commands[-top_k:]:
            imp_pct = cmd["improvement"] * 100
            if cmd["improvement"] < 0:
                print(
                    f"    {cmd['command']:<30} {cmd['raw_codebleu']:>7.4f} → "
                    f"{cmd['formatted_codebleu']:>7.4f} ({imp_pct:6.2f}%)"
                )


def main():
    results_dir = Path(__file__).resolve().parent.parent / "results" / "ab_test"
    
    if not results_dir.exists():
        print(f"Error: results directory not found at {results_dir}")
        sys.exit(1)
    
    if not CODEBLEU_AVAILABLE:
        print("\nWarning: CodeBLEU not available. Install with: pip install codebleu==0.7.0")
    
    print(f"\nEvaluating results from: {results_dir}")
    print(f"CodeBLEU available: {CODEBLEU_AVAILABLE}\n")
    
    # Evaluate each variant
    all_results = []
    variant_dirs = sorted([d for d in results_dir.iterdir() if d.is_dir()])
    
    for variant_dir in variant_dirs:
        print(f"Evaluating {variant_dir.name}...", end=" ", flush=True)
        result = evaluate_variant(variant_dir)
        all_results.append(result)
        print(f"({result['aggregate']['num_commands']} commands)")
    
    # Print results
    print_summary_table(all_results)
    print_best_worst(all_results, top_k=5)
    
    # Detailed output option
    if "--all-commands" in sys.argv:
        print("\n" + "=" * 100)
        print("ALL COMMANDS")
        print("=" * 100)
        for result in all_results:
            print(f"\n{result['variant']}:")
            for cmd in result["commands"]:
                imp_pct = cmd["improvement"] * 100
                print(
                    f"  {cmd['command']:<30} raw={cmd['raw_codebleu']:.4f} "
                    f"fmt={cmd['formatted_codebleu']:.4f} Δ={imp_pct:+7.2f}%"
                )


if __name__ == "__main__":
    main()
