#!/usr/bin/env python3
"""
score_opus5.py

Compute the CodeBLEU figures for the Opus 5 Baseline-2 rerun, using the same
methodology every other number in BASELINE2_GENERAL_MODEL_COMPARISON.md uses:
CodeBLEU of `generated.formatted.rs` against `oracle.formatted.rs`, computed
locally where the `codebleu` package is importable.

Two figures are produced:

  * pre-repair Best@1/3/5 -- from the pre-repair snapshot's meta.json
    candidate_scores (5 independent samples per command).
  * post-repair mean CodeBLEU -- recomputed directly from the final
    generated.formatted.rs of every command.  Deliberately NOT read from
    meta.json: repair_loop_verus.save_repair_result() appends each repair
    attempt onto candidate_scores, so post-repair that array no longer means
    "5 independent samples" and Best@k over it would be meaningless.

Usage:
    python3 tools/score_opus5.py --base results/ab_test/v3_opus5
"""

from __future__ import annotations

import argparse
import json
import statistics
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "prompt_engineering"))

from prompt_engineering import compute_codebleu, CODEBLEU_AVAILABLE  # noqa: E402


def best_at_k(per_command_scores: list[list[float]], k: int) -> float:
    vals = [max(s[:k], default=0.0) for s in per_command_scores if s]
    return sum(vals) / len(vals) if vals else 0.0


def load_candidate_scores(root: Path) -> tuple[list[list[float]], list[str]]:
    scores, missing = [], []
    for d in sorted(p for p in root.iterdir() if p.is_dir()):
        meta = d / "meta.json"
        if not meta.exists():
            missing.append(d.name)
            continue
        try:
            cand = json.loads(meta.read_text(encoding="utf-8")).get("candidate_scores") or []
        except Exception:
            missing.append(d.name)
            continue
        scores.append([float(c) for c in cand])
    return scores, missing


def recompute_codebleu(root: Path) -> dict[str, float]:
    out: dict[str, float] = {}
    for d in sorted(p for p in root.iterdir() if p.is_dir()):
        gen = d / "generated.formatted.rs"
        orc = d / "oracle.formatted.rs"
        if not gen.exists() or not orc.exists():
            continue
        g = gen.read_text(encoding="utf-8", errors="ignore")
        o = orc.read_text(encoding="utf-8", errors="ignore")
        out[d.name] = compute_codebleu(g, o)
    return out


def summarize_verus(path: Path) -> dict | None:
    if not path.exists():
        return None
    try:
        return json.loads(path.read_text(encoding="utf-8")).get("summary")
    except Exception:
        return None


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--base", default="results/ab_test/v3_opus5",
                    help="Run directory holding alp14/ and the summary JSONs")
    args = ap.parse_args()

    base = Path(args.base).resolve()
    out_dir = base / "alp14"
    pre_dir = base / "alp14_prerepair"

    if not CODEBLEU_AVAILABLE:
        print("[fatal] `codebleu` package not importable -- compute_codebleu() would "
              "silently return 0.0 for everything. Install it before scoring.")
        raise SystemExit(2)

    report: dict = {"base": str(base)}

    # ---- pre-repair Best@k -------------------------------------------------
    if pre_dir.exists():
        scores, missing = load_candidate_scores(pre_dir)
        report["pre_repair"] = {
            "commands": len(scores),
            "missing_meta": missing,
            "best@1": best_at_k(scores, 1),
            "best@3": best_at_k(scores, 3),
            "best@5": best_at_k(scores, 5),
        }
        # Sanity: every command should carry 5 samples pre-repair.
        n_samples = {len(s) for s in scores}
        report["pre_repair"]["sample_counts"] = sorted(n_samples)
    else:
        report["pre_repair"] = None

    # ---- post-repair CodeBLEU ---------------------------------------------
    post = recompute_codebleu(out_dir)
    report["post_repair"] = {
        "commands": len(post),
        "mean_codebleu": statistics.mean(post.values()) if post else 0.0,
        "median_codebleu": statistics.median(post.values()) if post else 0.0,
        "per_command": post,
    }

    # ---- Verus pass rates --------------------------------------------------
    report["verus"] = {
        "pre_repair": summarize_verus(base / "alp14_verus_check_summary.json"),
        "post_repair_loop": summarize_verus(base / "alp14_verus_check_summary_repaired.json"),
        "post_repair_independent_recheck": summarize_verus(
            base / "alp14_verus_check_summary_recheck.json"),
    }

    # ---- repair attempt distribution --------------------------------------
    attempts, resolved, unresolved = [], 0, []
    for d in sorted(p for p in out_dir.iterdir() if p.is_dir()):
        log = d / "repair_log.json"
        if not log.exists():
            continue
        try:
            l = json.loads(log.read_text(encoding="utf-8"))
        except Exception:
            continue
        attempts.append(l.get("attempts", 0))
        if l.get("resolved"):
            resolved += 1
        else:
            unresolved.append(d.name)
    report["repair"] = {
        "commands_repaired": len(attempts),
        "resolved": resolved,
        "unresolved": unresolved,
        "attempts_mean": statistics.mean(attempts) if attempts else 0.0,
        "attempts_max": max(attempts) if attempts else 0,
        "attempts_gt5": sum(1 for a in attempts if a > 5),
    }

    # ---- CLI ledger totals -------------------------------------------------
    ledger = base / "cli_calls.jsonl"
    if ledger.exists():
        calls = cache_hits = quota_waits = 0
        cost = wait_s = 0.0
        dur = 0.0
        for line in ledger.read_text(encoding="utf-8", errors="ignore").splitlines():
            try:
                r = json.loads(line)
            except Exception:
                continue
            ev = r.get("event")
            if ev == "call":
                calls += 1
                cost += float(r.get("cost_usd") or 0.0)
                dur += float(r.get("duration_s") or 0.0)
            elif ev == "cache_hit":
                cache_hits += 1
            elif ev == "quota_wait":
                quota_waits += 1
                wait_s += float(r.get("sleep_s") or 0.0)
        report["cli"] = {
            "calls": calls,
            "cache_hits": cache_hits,
            "total_cost_usd": round(cost, 2),
            "api_seconds": round(dur),
            "quota_wait_events": quota_waits,
            "quota_wait_seconds": round(wait_s),
        }

    out_path = base / "opus5_scores.json"
    out_path.write_text(json.dumps(report, indent=2), encoding="utf-8")

    # ---- human-readable ----------------------------------------------------
    print("=" * 68)
    print("Opus 5 (effort=high, via `claude -p`) -- alp14, 98 commands")
    print("=" * 68)
    pr = report.get("pre_repair")
    if pr:
        print(f"Pre-repair  CodeBLEU  Best@1={pr['best@1']:.4f}  "
              f"Best@3={pr['best@3']:.4f}  Best@5={pr['best@5']:.4f}  "
              f"({pr['commands']} cmds, samples/cmd={pr['sample_counts']})")
    v = report["verus"]
    for label, key in (("pre-repair", "pre_repair"),
                       ("post-repair (loop)", "post_repair_loop"),
                       ("post-repair (recheck)", "post_repair_independent_recheck")):
        s = v.get(key)
        if s:
            print(f"Verus {label:22s} {s['pass']}/{s['checked']} ({s['pass_rate']:.2f}%)")
    print(f"Post-repair CodeBLEU  mean={report['post_repair']['mean_codebleu']:.4f}  "
          f"median={report['post_repair']['median_codebleu']:.4f}  "
          f"({report['post_repair']['commands']} cmds)")
    r = report["repair"]
    print(f"Repair loop: {r['resolved']}/{r['commands_repaired']} resolved, "
          f"mean {r['attempts_mean']:.1f} attempts, max {r['attempts_max']}, "
          f"{r['attempts_gt5']} needed >5")
    if r["unresolved"]:
        print(f"  unresolved: {', '.join(r['unresolved'])}")
    if "cli" in report:
        c = report["cli"]
        print(f"CLI: {c['calls']} calls (+{c['cache_hits']} cached), "
              f"${c['total_cost_usd']}, {c['api_seconds']}s API, "
              f"{c['quota_wait_events']} quota waits ({c['quota_wait_seconds']}s)")
    print(f"\n[info] wrote {out_path}")


if __name__ == "__main__":
    main()
