#!/usr/bin/env python3
"""Run the prompt-engineering pipeline with only the V3-Structured prompt."""

import argparse
import os
import sys
from typing import Any, Dict, List, Tuple

from dataset_loader import load_dataset
from prompt_engineering import (
    ROOT_DIR,
    ClaudeHaikuModel,
    PromptVariant,
    aggregate_metrics,
    evaluate_prompt_variant,
    load_dotenv_fallback,
    resolve_verusfmt_binary,
)

V3_KEY = "v3"

PROMPT_V3_SYSTEM = """Generate Verus formal specifications for RMM commands.

Follow this structure and rules:
1) Identify command inputs and state variables.
2) Extract all failure conditions as implications:
    - (pre_fail ==> error_result)
3) Extract success condition as implication:
    - (all_required_preconditions ==> (success_result && success_postconditions))
4) Combine all clauses with &&.
5) Use old_s only for pre-state checks; use new_s for post-state checks.

Priority policy:
- Semantic correctness is higher priority than stylistic formatting.
- Do not invent behavior that is not supported by context/spec text.

Core constraints:
- Function name should be lowercase snake_case: {cmd_name_lower}_spec.
- Include an explicit result parameter when command returns a status/result code.
- Do NOT use old_s.result or new_s.result; use the function argument `result`.
- Do NOT invent helper/predicate/function names that are not present in the provided context/spec text.
- If a convenient abstraction is not explicitly available in context/spec, write the condition directly or return `true` rather than fabricating a new helper.
- Keep predicate/function arity consistent with provided context signatures.
- Prefer precise implication style over free-form narrative or comments.
- If the command semantics are effectively unconstrained in spec text, return `true`.
- For pure query / feature / version / count / get_* commands, do not force a full failure/success scaffold unless the context explicitly describes state changes; many of these should remain minimal or `true`.

Output self-check (must pass before final output):
- Family consistency: command prefix and symbol family must match (do not mix pdev/vdev, psmmu/vsmmu, etc.).
- Domain symbol anchoring for complex commands: for pdev/vdev/rtt/psmmu families, prefer helper/accessor names already present in context and avoid analogy-based renaming.
- Signature and type alignment: follow context/spec conventions; if uncertain, preserve semantically correct clauses rather than forcing cosmetic ordering/alias changes.
- State-parameter discipline: preserve oracle/context state style; do not collapse `old_s, new_s` into a single `s: S` parameter.
- Naming consistency: function name should remain lowercase snake_case and match {cmd_name_lower}_spec.
- For pure query / feature / version / count / get_* commands, preserve the oracle's signature order exactly when available; do not reorder arguments for stylistic reasons.

Targeted prescriptions for historically low-scoring commands (RTT/VDEV/DATA families):
- Apply this block ONLY when command semantics involve RTT/VDEV/DATA families; do not force these constraints onto unrelated families (e.g., PSCI/RSI).
- Signature completeness first: do not omit command operands (rd/ipa/level/index/rtt/top/data/vdev_ptr/addr, etc.).
- Do not replace missing operands with hidden fields like old_s.cmd_input_x* unless the spec/context explicitly defines that API style for this command.
- State-transition sanity: avoid impossible postconditions (e.g., X(new_s) == X(new_s) + delta). For updates, compare new_s against old_s.
- RTT error branches: when returning RMI_ERROR_RTT / RMI_ERROR_RTT_AUX with level payload, derive payload from the relevant walk condition consistently (avoid mixing old/new state references arbitrarily).
- Map/Unmap commands: ensure assigned/unassigned state transitions are paired with coherent granule-state transitions and unchanged-field framing for error paths.

EXAMPLE (style template):
Command: REC_EXIT
Input:
    B3.1.2 REC_EXIT command...
    Failure conditions: ID1, pre: condition1, post: result == ERROR_X
    Success conditions: ID2, pre: condition2
Output:
pub open spec fn rec_exit_spec(result: RmiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!condition1(old_s) ==> result == ERROR_X)
    && (condition2(old_s) ==> <success postconditions>)
}

Output ONLY the function body."""

PROMPT_V3_TEMPLATE = """{context}

{spec}

Requirements:
- Signature: pub open spec fn {cmd_name_lower}_spec(...) -> bool
- Handle all failure/success cases
- Use old_s/new_s for state before/after
- Preserve state parameters as `old_s: S, new_s: S` when that is the established context/oracle pattern; do not replace them with a single `s: S`
- Failure clauses should map to the correct error code
- Success should assert both result code and state postconditions
- Keep unchanged-state constraints when implied by the command behavior
- Keep symbol family consistent with command domain (e.g., pdev command uses pdev symbols)
- Do not introduce helper/predicate/function names unless they already appear in the provided context/spec
- For pure query / feature / version / count / get_* commands, avoid inventing side effects or post-state transitions when the oracle/context indicates no state change
- Prefer Bits64/UInt64/UInt32 aliases when present in context/spec, but do not sacrifice semantic correctness for alias formatting
- For complex families (pdev/vdev/rtt/psmmu), prioritize correct helper/function selection over cosmetic signature ordering
- For RTT/VDEV/DATA commands only, explicitly include command operands in the function signature and avoid deriving them implicitly from hidden state fields
- For RTT/VDEV/DATA commands only, write mutable counter/field postconditions as old_s -> new_s transitions (never new_s -> new_s self-comparisons)
- Return: single boolean expression
- No explanations, code only"""

V3_PROMPT = PromptVariant("V3-Structured", PROMPT_V3_SYSTEM, PROMPT_V3_TEMPLATE)


def parse_cli_args(argv: List[str]) -> Dict[str, Any]:
    parser = argparse.ArgumentParser(description="Run prompt engineering with only the V3 prompt")
    parser.add_argument("--split", default="test", choices=["train", "val", "test", "all"])
    parser.add_argument("--limit", type=int, default=3)
    parser.add_argument("--n-samples", type=int, default=1)
    parser.add_argument("--api-key", default=None)
    parser.add_argument("--save-results", action="store_true")
    parser.add_argument(
        "--resume",
        action="store_true",
        help="Reuse successful saved commands and rerun only failed/missing ones",
    )
    args = parser.parse_args(argv)
    return {
        "split": args.split,
        "limit": args.limit,
        "n_samples": args.n_samples,
        "api_key": args.api_key,
        "save_results": args.save_results,
        "resume": args.resume,
    }


def run_v3_only(
    dataset: List[Any],
    limit: int = 10,
    n_samples: int = 1,
    api_key: str = None,
    save_results: bool = False,
    resume: bool = False,
) -> Dict[str, Any] | None:
    """Evaluate only the V3 prompt variant and report Best@k metrics."""
    print(f"\n{'=' * 70}")
    print(f"V3-only evaluation")
    print(f"Model: Claude 4.5 Haiku | Problems: {limit} | Samples/problem: {n_samples}")
    print(f"Prompt: {V3_PROMPT.name}")
    print(f"{'=' * 70}")

    try:
        model = ClaudeHaikuModel(api_key=api_key)
        print("Connected to Claude API\n")
    except Exception as e:
        print(f"Failed to connect to Claude API: {e}\n")
        return None

    result = evaluate_prompt_variant(
        V3_PROMPT,
        variant_key=V3_KEY,
        dataset=dataset,
        model=model,
        limit=limit,
        n_samples=n_samples,
        save_results=save_results,
        resume=resume,
    )

    ks: Tuple[int, ...] = (1, 3, 5)
    metrics = aggregate_metrics(result["problem_results"], ks=ks)

    print(f"\n{'=' * 70}")
    print("Results (CodeBLEU Best@k):")
    print(f"{'=' * 70}")
    for k in ks:
        print(f"Best@{k}: {metrics[f'best@{k}']:.4f}")
    print(f"{'=' * 70}\n")

    print(f"Prompt ({V3_PROMPT.name}):")
    print(f"System:\n{V3_PROMPT.system}\n")
    print(f"Template (first 300 chars):\n{V3_PROMPT.user_template[:300]}...\n")

    return {
        "prompt": V3_PROMPT.name,
        "problem_results": result["problem_results"],
        "metrics": metrics,
    }


def main() -> None:
    cli = parse_cli_args(sys.argv[1:])
    load_dotenv_fallback(ROOT_DIR / ".env")

    verusfmt_bin = resolve_verusfmt_binary()
    if verusfmt_bin:
        print(f"[info] verusfmt binary: {verusfmt_bin}")
    else:
        print("[info] verusfmt binary: <not found>; fallback to fence-stripped text")

    print(f"Loading {cli['split']} split from raw sections...")
    dataset = load_dataset(split=cli["split"])

    if not dataset:
        print("No data loaded")
        return

    limit = min(cli["limit"], len(dataset))
    print(f"Loaded {len(dataset)} samples (running first {limit})\n")

    api_key = os.getenv("ANTHROPIC_API_KEY") or cli["api_key"]
    if not api_key:
        print("ANTHROPIC_API_KEY not set")
        return

    result = run_v3_only(
        dataset,
        limit=limit,
        n_samples=max(1, cli["n_samples"]),
        api_key=api_key,
        save_results=cli["save_results"],
        resume=cli["resume"],
    )

    if result is None:
        print("V3 evaluation failed")


if __name__ == "__main__":
    main()
