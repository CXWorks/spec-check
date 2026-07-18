#!/usr/bin/env python3
"""
Phase 2: Prompt Engineering & Claude Integration

1. Extract current prompt from train.jsonl
2. Design 5 prompt variants
3. A/B test with small sample (N=10)
4. Select best, run full batch with Claude API
"""

import argparse
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from typing import List, Dict, Any, Callable, Tuple, Optional
from dataset_loader import SpecOracle, load_dataset, EvaluationMetrics

try:
    from codebleu import calc_codebleu as _calc_codebleu

    CODEBLEU_AVAILABLE = True
except ImportError:
    CODEBLEU_AVAILABLE = False

# ============================================================================
# Prompt Variants
# ============================================================================

# V0: Current baseline (extracted from train.jsonl)
PROMPT_V0_SYSTEM = """You are a formal specification assistant for Arm CCA (Confidential Compute Architecture) Realm Management Monitor (RMM). Given the specification text for an RMM command and the shared Verus type/function context (preamble), generate the Verus specification function for that command. The output should be a single `pub open spec fn {cmd}_spec(...)` function body in valid Verus syntax."""

PROMPT_V0_TEMPLATE = """{system_prompt}

## Input

### Context (shared Verus types and helper function signatures)
{context}

### Command Specification (from RMM spec PDF)
{spec}

## Task

Generate the Verus specification function body in the format:
```verus
pub open spec fn {cmd_name}_spec(...) -> bool {{
  // your code here
}}
```

Return ONLY the function body, no markdown, no comments."""

# V1: Minimal (spec only, no extra instructions)
PROMPT_V1_SYSTEM = """Generate Verus specification functions from RMM command specifications."""

PROMPT_V1_TEMPLATE = """{context}

{spec}

Generate the Verus `pub open spec fn {cmd_name}_spec(...)` function."""

# V2: Few-shot (add example)
PROMPT_V2_SYSTEM = """You are a formal specification assistant for Arm CCA (Confidential Compute Architecture) Realm Management Monitor (RMM). Generate Verus specification functions from command specifications.

EXAMPLE:
Command: REC_EXIT
Input:
  B3.1.2 REC_EXIT command...
  Failure conditions: ID1, pre: condition1, post: result == ERROR_X
  Success conditions: ID2, pre: condition2
Output:
pub open spec fn rec_exit_spec(result: RmiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!condition1(old_s) ==> result == ERROR_X)
  && (condition2(old_s) ==> <success postconditions>)
}"""

PROMPT_V2_TEMPLATE = """{system_prompt}

## Input
{context}

{spec}

## Generate the Verus function body only."""

# structure 优化：得分低或者错误的代码，看一下哪里对哪里不对，分析一下是哪里没理解，
# 哪里没覆盖到，看看有没有什么规律。然后针对这些问题，设计一些规则或者提示，让模型更好地理解和覆盖这些情况。
# 限定一下情况
# 有没有别的prompt做对的地方，能不能补充到这个prompt里，形成一个更全面的提示，结合起来
# e.g. v2对psci_features做得好，可以分析一下它是怎么描述这个条件的，然后把类似的描述加到v3里，形成一个更全面的提示。
# cost, improvements, 根据数字decide 一个trade off，看看是要更复杂的prompt，还是要更多的样本，还是要更强的模型，还是要更好的后处理（比如verusfmt或者一些规则化工具）来提升质量。
# 
# V3: Structured (explicit output format)
PROMPT_V3_SYSTEM = """Generate Verus formal specifications for RMM commands.

Follow this structure:
1. List all input parameters from the spec
2. Extract failure conditions: pre1 → post1, pre2 → post2, ...
3. Extract success conditions
4. Combine into boolean expression with:
   - Implications (pre ==> post)
   - Conjunctions (&&)
   - State updates (new_s vs old_s)

Output ONLY the function body."""

PROMPT_V3_TEMPLATE = """{context}

{spec}

Requirements:
- Signature: pub open spec fn {cmd_name}_spec(...) -> bool
- Handle all failure/success cases
- Use old_s/new_s for state before/after
- Return: single boolean expression
- No explanations, code only"""

# V4: Best practices (idioms + rules)
PROMPT_V4_SYSTEM = """Generate Verus formal specifications for RMM commands. Use these rules:

RULES:
1. Every pre-condition failure → post-condition assertion
2. Format: (pre_fail ==> post_fail) && (pre_fail2 ==> post_fail2) ...
3. Success: all preconditions + postconditions
4. State: old_s = before, new_s = after
5. Types: Address, Bits64, UInt32, RmmRealm, etc.
6. Predicates: AddrIsProtected(s, addr, realm), MpidrIsUsed(s, mpidr), etc.
7. Conjoin all conditions with &&
8. Use as int for type conversions

OUTPUT: function body only, no markdown/comments."""

PROMPT_V4_TEMPLATE = """{context}

Command: {cmd_name}

Specification:
{spec}

Generate: pub open spec fn {cmd_name}_spec(...) -> bool {{ ... }}"""


# ============================================================================
# Prompt Manager
# ============================================================================

class PromptVariant:
    """Single prompt variant with name and template"""

    def __init__(self, name: str, system: str, user_template: str):
        self.name = name
        self.system = system
        self.user_template = user_template

    def format(
        self,
        spec: str,
        context: str,
        cmd_name: str,
        retrieved_rules: str = "",
    ) -> Dict[str, str]:
        """Format messages for API call"""
        user_content = self.user_template.format(
            system_prompt=self.system if "{system_prompt}" in self.user_template else "",
            context=context,
            spec=spec,
            cmd_name=cmd_name,
            cmd_name_lower=cmd_name.lower(),
            retrieved_rules=retrieved_rules,
        )
        return {
            "system": self.system,
            "user": user_content,
        }


# All prompt variants
PROMPTS = {
    "v0": PromptVariant("V0-Baseline", PROMPT_V0_SYSTEM, PROMPT_V0_TEMPLATE),
    "v1": PromptVariant("V1-Minimal", PROMPT_V1_SYSTEM, PROMPT_V1_TEMPLATE),
    "v2": PromptVariant("V2-FewShot", PROMPT_V2_SYSTEM, PROMPT_V2_TEMPLATE),
    "v3": PromptVariant("V3-Structured", PROMPT_V3_SYSTEM, PROMPT_V3_TEMPLATE),
    "v4": PromptVariant("V4-BestPractices", PROMPT_V4_SYSTEM, PROMPT_V4_TEMPLATE),
}


def format_retrieved_rules_block(results: List[Dict[str, Any]]) -> str:
    """Render retrieved rules in a fixed, deterministic template block."""
    if not results:
        return ""

    lines = ["[Retrieved Rust Coding Rules]"]
    for i, rule in enumerate(results, 1):
        content = (rule.get("content") or "").strip()
        if content:
            lines.append(f"{i}. {content}")

    lines.append("(Reference only; do not invent symbols beyond context/spec; command spec takes priority on conflict.)")
    return "\n".join(lines)


# ============================================================================
# Claude Model (Placeholder for real API)
# ============================================================================

class ClaudeHaikuModel:
    """
    Real Claude 4.5 Haiku API integration.
    """

    def __init__(self, api_key: str = None):
        import os

        self.name = "claude-haiku-4-5-20251001"  # Haiku 4.5 - faster and cheaper
        self.api_key = api_key or os.getenv("ANTHROPIC_API_KEY")
        self.call_count = 0

        if not self.api_key:
            raise ValueError("ANTHROPIC_API_KEY not set. Provide api_key or set environment variable.")

        try:
            import anthropic

            self.client = anthropic.Anthropic(api_key=self.api_key)
        except ImportError:
            raise ImportError("anthropic package not installed. Run: pip install anthropic")

    def generate(self, messages: List[Dict[str, str]]) -> str:
        """
        Generate using Claude API.
        Args:
            messages: [{"role": "system", "content": "..."}, {"role": "user", "content": "..."}]
        Returns:
            Generated Verus code
        """
        self.call_count += 1

        # Extract system message once
        system_msg = None
        user_msgs = []
        for msg in messages:
            if msg.get("role") == "system":
                system_msg = msg.get("content")
            else:
                user_msgs.append(msg)

        # Retry with exponential backoff on rate-limit errors
        wait = 15
        for attempt in range(5):
            try:
                response = self.client.messages.create(
                    model=self.name,
                    max_tokens=4096,
                    system=system_msg,
                    messages=user_msgs,
                )
                time.sleep(1.5)  # throttle requests to reduce token/min spikes
                return response.content[0].text.strip()
            except Exception as e:
                err = str(e)
                if "rate_limit_error" in err or "429" in err:
                    print(f"  Rate limit hit (call #{self.call_count}, attempt {attempt + 1}), waiting {wait}s...")
                    time.sleep(wait)
                    wait = min(wait * 2, 120)
                else:
                    print(f"Claude API error (call #{self.call_count}): {e}")
                    return "ERROR"

        print(f"Claude API error (call #{self.call_count}): max retries exceeded")
        return "ERROR"


# ============================================================================
# CodeBLEU + Formatting helpers
# ============================================================================

ROOT_DIR = Path(__file__).resolve().parent.parent
VERUSFMT = ROOT_DIR / "training" / "verusfmt" / "target" / "release" / "verusfmt"
_VERUSFMT_MISSING_WARNED = False


def resolve_verusfmt_binary() -> str:
    """Resolve verusfmt binary path with fallbacks.

    Priority:
      1) Repo-local default path (training/verusfmt/...)
      2) VERUSFMT_BIN environment variable
      3) verusfmt from system PATH
    """
    if VERUSFMT.exists() and os.access(VERUSFMT, os.X_OK):
        return str(VERUSFMT)

    env_bin = os.getenv("VERUSFMT_BIN")
    if env_bin:
        p = Path(env_bin).expanduser()
        if p.exists() and os.access(p, os.X_OK):
            return str(p)

    path_bin = shutil.which("verusfmt")
    if path_bin:
        return path_bin

    return ""


def _extract_inner_verus_block(text: str) -> str:
    m = re.search(r"verus!\s*\{(.*)\}\s*(?://[^\n]*)?\s*$", text, re.DOTALL)
    if m:
        return m.group(1).strip()
    return text.strip()


def _strip_markdown_code_fence(text: str) -> str:
    """Strip leading/trailing markdown code fence (```...```) when present."""
    if not text:
        return text

    t = text.strip()
    m = re.match(r"^```(?:\w+)?\s*\n(.*?)\n```\s*$", t, re.DOTALL)
    if m:
        return m.group(1).strip()

    # Fallback for uneven fence variants
    if t.startswith("```"):
        lines = t.splitlines()
        if len(lines) >= 2 and lines[0].startswith("```") and lines[-1].strip() == "```":
            return "\n".join(lines[1:-1]).strip()

    return t


def normalize_verus_with_verusfmt(code: str) -> str:
    """Normalize snippet through verusfmt; fallback to original code."""
    global _VERUSFMT_MISSING_WARNED
    if not code:
        return code
    cleaned = _strip_markdown_code_fence(code)
    verusfmt_bin = resolve_verusfmt_binary()
    if not verusfmt_bin:
        if not _VERUSFMT_MISSING_WARNED:
            print("[warn] verusfmt not found; using fence-stripped text without formatter. "
                  "Set VERUSFMT_BIN or install `verusfmt` in PATH.")
            _VERUSFMT_MISSING_WARNED = True
        return cleaned

    wrapped = f"use vstd::prelude::*;\nverus! {{\n{cleaned}\n}}\n"
    with tempfile.NamedTemporaryFile(suffix=".rs", mode="w", delete=False) as f:
        f.write(wrapped)
        fname = f.name

    try:
        r = subprocess.run(
            [verusfmt_bin, "--verus-only", fname],
            capture_output=True,
            timeout=15,
        )
        if r.returncode == 0:
            txt = Path(fname).read_text(encoding="utf-8", errors="ignore")
            return _extract_inner_verus_block(txt)
    except Exception:
        pass
    finally:
        try:
            os.unlink(fname)
        except OSError:
            pass

    return cleaned


def compute_codebleu(generated: str, oracle: str) -> float:
    if not CODEBLEU_AVAILABLE:
        return 0.0
    if not generated or generated == "ERROR" or not oracle:
        return 0.0
    try:
        result = _calc_codebleu(
            [[oracle]],
            [generated],
            lang="rust",
            weights=(0.25, 0.25, 0.25, 0.25),
        )
        return float(result["codebleu"])
    except Exception:
        return 0.0


def aggregate_metrics(problem_results: List[Dict], ks: Tuple[int, ...] = (1, 3, 5)) -> Dict[str, float]:
    """Best@k: for each problem, max CodeBLEU among first k samples, then average."""
    metrics: Dict[str, float] = {}
    for k in ks:
        scores = []
        for prob in problem_results:
            candidates = prob["samples"][:k]
            best = max((s["codebleu"] for s in candidates), default=0.0)
            scores.append(best)
        metrics[f"best@{k}"] = sum(scores) / len(scores) if scores else 0.0
    return metrics


def dump_sample_result(
    variant_key: str,
    sample: SpecOracle,
    candidates: List[Dict[str, Any]],
    results_root: Path,
) -> None:
    """Save best candidate and metadata under results/ab_test/{variant}/{version}/{command}/"""
    out_dir = results_root / variant_key / sample.version / sample.command.lower()
    out_dir.mkdir(parents=True, exist_ok=True)

    oracle_raw = sample.oracle.strip()
    oracle_fmt = normalize_verus_with_verusfmt(oracle_raw)

    best_idx = max(range(len(candidates)), key=lambda i: candidates[i]["codebleu"]) if candidates else -1
    best = candidates[best_idx] if best_idx >= 0 else {"raw": "ERROR", "formatted": "ERROR", "codebleu": 0.0}

    (out_dir / "generated.raw.rs").write_text(best["raw"], encoding="utf-8")
    (out_dir / "generated.formatted.rs").write_text(best["formatted"], encoding="utf-8")
    (out_dir / "oracle.raw.rs").write_text(oracle_raw, encoding="utf-8")
    (out_dir / "oracle.formatted.rs").write_text(oracle_fmt, encoding="utf-8")
    (out_dir / "meta.json").write_text(
        json.dumps(
            {
                "command": sample.command,
                "version": sample.version,
                "n_samples": len(candidates),
                "best_index": best_idx,
                "best_codebleu": float(best.get("codebleu", 0.0)),
                "candidate_scores": [float(c.get("codebleu", 0.0)) for c in candidates],
            },
            indent=2,
        ),
        encoding="utf-8",
    )


def load_saved_candidates(variant_key: str, sample: SpecOracle, results_root: Path) -> List[Dict[str, Any]]:
    """Load candidate scores from existing meta.json for resume mode."""
    meta_path = results_root / variant_key / sample.version / sample.command.lower() / "meta.json"
    if not meta_path.exists():
        return []
    try:
        d = json.loads(meta_path.read_text(encoding="utf-8", errors="ignore"))
        scores = d.get("candidate_scores") or []
        return [{"raw": "<cached>", "formatted": "<cached>", "codebleu": float(s)} for s in scores]
    except Exception:
        return []


def has_successful_saved_result(
    variant_key: str,
    sample: SpecOracle,
    results_root: Path,
    required_samples: int,
) -> bool:
    """A saved command is reusable if generated.raw.rs is non-ERROR and meta has enough samples."""
    cmd_dir = results_root / variant_key / sample.version / sample.command.lower()
    raw_path = cmd_dir / "generated.raw.rs"
    if not raw_path.exists():
        return False
    raw_text = raw_path.read_text(encoding="utf-8", errors="ignore").strip()
    if not raw_text or raw_text == "ERROR":
        return False
    saved = load_saved_candidates(variant_key, sample, results_root)
    return len(saved) >= required_samples


# ============================================================================
# Evaluation with Coverage
# ============================================================================

def count_constraints(verus_code: str) -> int:
    """Count how many constraint operators appear"""
    operators = ["==>", "&&", "||", "==", "!=", ">", "<"]
    count = 0
    for op in operators:
        count += verus_code.count(op)
    return count


class EvaluationMetricsV2(EvaluationMetrics):
    """Extended metrics with constraint coverage"""

    def __init__(self, command: str, version: str):
        super().__init__(command, version)
        self.oracle_constraints = 0
        self.generated_constraints = 0
        self.coverage = 0.0

    def evaluate(self, generated: str, oracle: str):
        """Evaluate exact match + constraint coverage"""
        super().evaluate(generated, oracle)
        self.oracle_constraints = count_constraints(oracle)
        self.generated_constraints = count_constraints(generated)
        if self.oracle_constraints > 0:
            self.coverage = min(1.0, self.generated_constraints / self.oracle_constraints)

    def __repr__(self):
        status = "good" if self.exact_match else "not good"
        return f"{status} {self.command} ({self.version}): exact={self.exact_match}, coverage={self.coverage:.1%}"


# ============================================================================
# A/B Testing
# ============================================================================

def evaluate_prompt_variant(
    prompt_variant: PromptVariant,
    variant_key: str,
    dataset: List[SpecOracle],
    model: Callable,
    limit: int = 10,
    n_samples: int = 1,
    save_results: bool = False,
    resume: bool = False,
    retriever: Optional[Any] = None,
    rag_top_k: int = 0,
    results_root: Path = ROOT_DIR / "results" / "ab_test",
) -> Dict[str, Any]:
    """
    Generate n_samples per problem and compute Best@k CodeBLEU.
    """
    problem_results: List[Dict[str, Any]] = []

    print(f"\n  Testing {prompt_variant.name} (n_samples={n_samples})...")

    for i, sample in enumerate(dataset[:limit], 1):
        if save_results and resume and has_successful_saved_result(
            variant_key,
            sample,
            results_root,
            required_samples=n_samples,
        ):
            cached_candidates = load_saved_candidates(variant_key, sample, results_root)[:n_samples]
            problem_results.append(
                {
                    "command": sample.command,
                    "version": sample.version,
                    "samples": cached_candidates,
                }
            )
            if i % 5 == 0 or i == limit:
                m = aggregate_metrics([problem_results[-1]])
                print(f"    [{i}/{limit}] {sample.command}  best@1={m['best@1']:.3f} (cached)")
            continue

        context = sample.preamble
        spec = sample.section_text

        retrieved_rules_block = ""
        if retriever is not None and rag_top_k > 0:
            query = f"{sample.command}\n{sample.section_text}"
            try:
                retrieved = retriever.search(query, top_k=rag_top_k)
                retrieved_rules_block = format_retrieved_rules_block(retrieved)
            except Exception as e:
                print(f"    [warn] RAG retrieval failed for {sample.command}: {e}")

        if retrieved_rules_block:
            context = f"{context}\n\n{retrieved_rules_block}"

        messages_dict = prompt_variant.format(
            spec,
            context,
            sample.command,
            retrieved_rules=retrieved_rules_block,
        )
        messages = [
            {"role": "system", "content": messages_dict["system"]},
            {"role": "user", "content": messages_dict["user"]},
        ]

        oracle_fmt = normalize_verus_with_verusfmt(sample.oracle)
        candidates = []
        for _ in range(n_samples):
            raw = model.generate(messages)
            formatted = normalize_verus_with_verusfmt(raw)
            score = compute_codebleu(formatted, oracle_fmt)
            candidates.append({"raw": raw, "formatted": formatted, "codebleu": score})

        problem_results.append({
            "command": sample.command,
            "version": sample.version,
            "samples": candidates,
        })

        if save_results:
            dump_sample_result(variant_key, sample, candidates, results_root)

        if i % 5 == 0 or i == limit:
            m = aggregate_metrics([problem_results[-1]])
            print(f"    [{i}/{limit}] {sample.command}  best@1={m['best@1']:.3f}")

    return {
        "prompt": prompt_variant.name,
        "problem_results": problem_results,
    }


def run_ab_testing(
    dataset: List[SpecOracle],
    limit: int = 10,
    n_samples: int = 1,
    api_key: str = None,
    save_results: bool = False,
    resume: bool = False,
    retriever: Optional[Any] = None,
    rag_top_k: int = 0,
):
    """
    A/B test all prompt variants.
    Find the best one.
    """
    print(f"\n{'='*70}")
    print(f"A/B Testing {len(PROMPTS)} prompt variants")
    print(f"Model: Claude 4.5 Haiku | Problems: {limit} | Samples/problem: {n_samples}")
    print(f"{'='*70}")

    # Create Claude model instance
    try:
        model = ClaudeHaikuModel(api_key=api_key)
        print("Connected to Claude API\n")
    except Exception as e:
        print(f"Failed to connect to Claude API: {e}\n")
        return None, {}

    all_results = {}
    for key, prompt in PROMPTS.items():
        result = evaluate_prompt_variant(
            prompt,
            variant_key=key,
            dataset=dataset,
            model=model,
            limit=limit,
            n_samples=n_samples,
            save_results=save_results,
            resume=resume,
            retriever=retriever,
            rag_top_k=rag_top_k,
        )
        all_results[key] = result

    # Summary table
    ks = (1, 3, 5)
    print(f"\n{'='*70}")
    print("Results (CodeBLEU Best@k):")
    print(f"{'='*70}")
    header = f"{'Prompt':<22}" + "".join(f"  Best@{k:<5}" for k in ks)
    print(header)
    print("-" * len(header))
    scored: Dict[str, Dict[str, float]] = {}
    for key in sorted(PROMPTS.keys()):
        result = all_results[key]
        m = aggregate_metrics(result["problem_results"], ks=ks)
        scored[key] = m
        row = f"{result['prompt']:<22}" + "".join(f"  {m[f'best@{k}']:<9.4f}" for k in ks)
        print(row)

    best_key = max(scored.keys(), key=lambda k: scored[k]["best@1"])
    print(f"\nBest prompt by Best@1: {PROMPTS[best_key].name}  (Best@1={scored[best_key]['best@1']:.4f})")
    print(f"{'='*70}\n")

    return best_key, all_results


# ============================================================================
# Main
# ============================================================================

def parse_cli_args(argv: List[str]) -> Dict[str, Any]:
    parser = argparse.ArgumentParser(description="Prompt A/B testing with Best@k")
    parser.add_argument("--split", default="test", choices=["train", "val", "test", "all"])
    parser.add_argument("--limit", type=int, default=3)
    parser.add_argument("--n-samples", type=int, default=1)
    parser.add_argument("--api-key", default=None)
    parser.add_argument("--save-results", action="store_true")
    parser.add_argument("--resume", action="store_true", help="Reuse successful saved commands and rerun only failed/missing ones")
    parser.add_argument(
        "--rag-index",
        default=None,
        help="Path to RAG index.pkl; if set with --rag-top-k > 0, inject retrieved rule block",
    )
    parser.add_argument(
        "--rag-top-k",
        type=int,
        default=0,
        help="How many retrieved rules to inject (default: 0 = disabled)",
    )
    args = parser.parse_args(argv)
    return {
        "split": args.split,
        "limit": args.limit,
        "n_samples": args.n_samples,
        "api_key": args.api_key,
        "save_results": args.save_results,
        "resume": args.resume,
        "rag_index": args.rag_index,
        "rag_top_k": args.rag_top_k,
    }


def load_dotenv_fallback(dotenv_path: Path) -> None:
    """Minimal .env loader (only sets missing keys)."""
    if not dotenv_path.exists():
        return
    try:
        for raw in dotenv_path.read_text(encoding="utf-8", errors="ignore").splitlines():
            line = raw.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, val = line.split("=", 1)
            key = key.strip()
            val = val.strip().strip('"').strip("'")
            if key and key not in os.environ:
                os.environ[key] = val
    except Exception:
        pass


def main():
    cli = parse_cli_args(sys.argv[1:])
    load_dotenv_fallback(ROOT_DIR / ".env")

    verusfmt_bin = resolve_verusfmt_binary()
    if verusfmt_bin:
        print(f"[info] verusfmt binary: {verusfmt_bin}")
    else:
        print("[info] verusfmt binary: <not found>; fallback to fence-stripped text")

    # Load dataset from raw section files (not JSONL)
    print(f"Loading {cli['split']} split from raw sections...")
    dataset = load_dataset(split=cli["split"])

    if not dataset:
        print("No data loaded")
        return

    limit = min(cli["limit"], len(dataset))
    print(f"Loaded {len(dataset)} samples (running first {limit})\n")

    # Get API key
    api_key = os.getenv("ANTHROPIC_API_KEY") or cli["api_key"]

    if not api_key:
        print("ANTHROPIC_API_KEY not set")
        return

    retriever = None
    rag_top_k = max(0, int(cli.get("rag_top_k", 0)))
    rag_index = cli.get("rag_index")
    if rag_top_k > 0:
        if not rag_index:
            print("[warn] --rag-top-k > 0 but --rag-index not provided; RAG disabled")
            rag_top_k = 0
        else:
            try:
                if str(ROOT_DIR) not in sys.path:
                    sys.path.insert(0, str(ROOT_DIR))
                from rag.retriever import RuleRetriever

                retriever = RuleRetriever(rag_index)
                print(f"[info] RAG enabled: top_k={rag_top_k}, index={rag_index}")
            except Exception as e:
                print(f"[warn] Failed to initialize retriever: {e}; RAG disabled")
                retriever = None
                rag_top_k = 0

    # A/B test prompt variants
    best_key, _results = run_ab_testing(
        dataset,
        limit=limit,
        n_samples=max(1, cli["n_samples"]),
        api_key=api_key,
        save_results=cli["save_results"],
        resume=cli["resume"],
        retriever=retriever,
        rag_top_k=rag_top_k,
    )

    if best_key is None:
        print("A/B testing failed")
        return

    # Show best prompt
    best_prompt = PROMPTS[best_key]
    print(f"Best Prompt ({best_prompt.name}):")
    print(f"System:\n{best_prompt.system}\n")
    print(f"Template (first 300 chars):\n{best_prompt.user_template[:300]}...\n")

if __name__ == "__main__":
    main()
