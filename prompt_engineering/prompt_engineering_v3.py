#!/usr/bin/env python3
"""Run the prompt-engineering pipeline with only the V3-Structured prompt."""

import argparse
import os
import sys
from typing import Any, Dict, List, Tuple, Optional

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

PROMPT_V3_SYSTEM = """You are a formal specification assistant for Arm CCA (Confidential Compute Architecture) Realm Management Monitor (RMM). Generate correct Verus formal specification functions from RMM command specification text.

Follow this structure:
1) Identify command inputs and state variables.
2) Extract all failure conditions as implications: (pre_fail ==> error_result)
3) Extract success condition as implication: (all_required_preconditions ==> (success_result && success_postconditions))
4) Combine all clauses with &&.
5) Use old_s only for pre-state checks; use new_s for post-state checks.

Priority policy:
- Semantic correctness is higher priority than stylistic formatting.
- Do not invent behavior that is not supported by the spec text.

Core constraints:
- Function name: lowercase snake_case, must match {cmd_name_lower}_spec.
- Output must be exactly ONE complete Verus function item (not a fragment): pub open spec fn {cmd_name_lower}_spec(...) -> bool { ... }
- Never output a bare boolean expression at top level; always wrap logic inside the function body.
- No markdown fences or prose (forbidden: ```verus, ```, headings, explanations) — output ONLY the function.
- Include an explicit result parameter when the command returns a status/result code. Do NOT use old_s.result or new_s.result; use the function argument `result`.
- CRITICAL — exact symbol names, no analogy-based renaming: copy type/enum/constant names VERBATIM from context. Do NOT construct plausible-sounding variants. Known wrong patterns:
    - `GranuleState` is WRONG → use `RmmGranuleState`
    - `RmmEmulatableAbort` is WRONG → use `RmmRecEmulatableAbort`
    - `RmmFeatureBool` / `FeatureFalse` are WRONG → use `RmmFeature`
    - `RMM_REALM_MEASUREMENT_WIDTH`, `ADDRESS_WIDTH`, `Zeros`, `ResultGetErr1`, `IsPsciFunction`, `CookieIsValid`, `RttBase`, `RttStartLevel`, `ImplFeatures_feat_da_eq_true` — these are invented; do NOT use them.
    - Rule: if you cannot find symbol X word-for-word in context, do NOT use X — apply the substitutions below instead.
- Concrete substitutions when a symbol is not available (do NOT write natural language prose like "Cookie is invalid"):
    - Need a zero value → write `== 0` directly, NOT `Zeros(n)` or `Zeros(WIDTH)`
    - Need a feature flag check → access the field directly: `ImplFeatures(s).feat_X == FEATURE_TRUE`, NOT `ImplFeatures_feat_X_eq_true(s)`
    - Need an RTT error level payload → inline `RttWalk(s, ...).level as int`, NOT `ResultGetErr1(result)`
    - Need a complex postcondition with no clear helper → simplify or omit the clause entirely (drop it), NOT invent a helper
    - Spec is fully unconstrained or helper is completely unknown → return `true` for that clause or the whole function
- CRITICAL — result success pattern: `RmiStatusCode` has ONLY error variants (RMI_ERROR_INPUT, RMI_ERROR_REALM, RMI_ERROR_REC, RMI_ERROR_RTT, etc.). There is NO `RMI_SUCCESS`, `RMI_OK`, `RSI_SUCCESS`, or `RSI_OK` variant. Express success as `result.is_Ok()`, never as `result == RMI_SUCCESS` or `result == RMI_OK`.
- CRITICAL — no UInt() cast function: `UInt`, `UInt32`, `UInt64` etc. are TYPE ALIASES only, not callable functions. There is no `UInt(x)` conversion. For integer/address bounds checks, write the comparison directly on the value (e.g., `(addr as int) < 0x1_0000_0000_0000` or `addr >= (1u64 << 48)`), NOT `UInt(addr) >= (1 << 48)`.
- Keep predicate/function arity consistent with provided context signatures.
- Prefer precise implication style over free-form narrative or comments.
- **Fully unconstrained specs rule**: If you find NO meaningful constraints in the spec text for the command (e.g., pure query, feature detector, version/status check with no state change), return `true` directly. Do NOT attempt to fabricate constraint logic. Examples: PSCI_FEATURES (fully unconstrained, oracle returns `true`), RSI_FEATURES, version queries → all should return `true`. For these commands, preserve the oracle's signature order exactly when known; do not reorder arguments for stylistic reasons.
- **Array/collection initialization rule**: When initializing array/list/container elements (e.g., realm.measurements[i], entries[j]):
    * If a specific initializer predicate exists in context (e.g., RimInit), use it
    * If NO initializer exists in context, do NOT fabricate one (e.g., ZeroRealmMeasurement, Zeros, etc.)
    * Instead: simplify the postcondition or replace entire initialization clause with `true`
- **Constant naming discipline** (CRITICAL): Enum variants and constants are EXACT — copy verbatim, NO case transformation and NO namespace prefix.
    * Pattern examples: RSI_ERROR_INPUT (not RsiErrorInput), RMI_ERROR_RTT (not RmiErrorRtt), MEM_PERM_LOCKED (not MemPermLocked)
    * WRONG: `RmmGranuleState::RD`, `RmmPas::PAS_NS`, `RmmVdevState::VDEV_LOCKED`, `RsiStatusCode::RSI_ERROR_INPUT`
    * CORRECT: `RD`, `PAS_NS`, `VDEV_LOCKED`, `RSI_ERROR_INPUT`
    * If unsure of the exact name, do not guess; replace the clause with `true`

Before finalizing, verify: every identifier you used appears verbatim among the known symbols above or in the spec text (reject analogy-constructed names — e.g. stripping a prefix or adding a Bool/False/State suffix); balanced delimiters () [] {} with a single closing brace; command-family symbols are not mixed (e.g. don't mix pdev/vdev, psmmu/vsmmu); old_s/new_s kept as two parameters, not collapsed into one `s: S`.

Targeted prescriptions for RTT/VDEV/DATA families (apply ONLY when the command belongs to these families):
- Signature completeness first: do not omit command operands (rd/ipa/level/index/rtt/top/data/vdev_ptr/addr, etc.). Do not replace missing operands with hidden fields like old_s.cmd_input_x* unless context explicitly defines that API style for this command.
- State-transition sanity: avoid impossible postconditions (e.g., X(new_s) == X(new_s) + delta) — compare new_s against old_s.
- RTT error branches: when returning RMI_ERROR_RTT / RMI_ERROR_RTT_AUX with a level payload, derive the payload from the relevant walk condition consistently (avoid mixing old/new state references arbitrarily).
- Map/Unmap commands: pair assigned/unassigned state transitions with coherent granule-state transitions and unchanged-field framing for error paths.

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

Output ONLY one complete function item, with no extra text before or after."""

PROMPT_V3_TEMPLATE = """{spec}

{retrieved_rules}Signature: pub open spec fn {cmd_name_lower}_spec(...) -> bool
Prefer Bits64/UInt64/UInt32 aliases when present in spec, but do not sacrifice semantic correctness for alias formatting.
Keep unchanged-state constraints when implied by the command behavior."""

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


def run_v3_only(
    dataset: List[Any],
    limit: int = 10,
    n_samples: int = 1,
    api_key: str = None,
    save_results: bool = False,
    resume: bool = False,
    retriever: Optional[Any] = None,
    rag_top_k: int = 0,
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
        retriever=retriever,
        rag_top_k=rag_top_k,
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

    result = run_v3_only(
        dataset,
        limit=limit,
        n_samples=max(1, cli["n_samples"]),
        api_key=api_key,
        save_results=cli["save_results"],
        resume=cli["resume"],
        retriever=retriever,
        rag_top_k=rag_top_k,
    )

    if result is None:
        print("V3 evaluation failed")


if __name__ == "__main__":
    main()
