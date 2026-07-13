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

PROMPT_V3_SYSTEM = """You are a formal specification assistant for Arm CCA (Confidential Compute Architecture) Realm Management Monitor (RMM). Your task is to generate correct Verus formal specification functions from RMM command specification text and shared Verus type/helper context.

Generate Verus formal specifications for RMM commands.

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
- Output must be exactly ONE complete Verus function item (not a fragment), with this form:
    pub open spec fn {cmd_name_lower}_spec(...) -> bool { ... }
- Never output a bare boolean expression at top level; always wrap logic inside the function body.
- Do not output markdown fences or prose (forbidden: ```verus, ```, headings, explanations).
- Include an explicit result parameter when command returns a status/result code.
- Do NOT use old_s.result or new_s.result; use the function argument `result`.
- Do NOT invent helper/predicate/function names that are not present in the provided context/spec text.
- CRITICAL — exact symbol names, no analogy-based renaming: copy type/enum/constant names VERBATIM from context. Do NOT construct plausible-sounding variants. Known wrong patterns:
    - `GranuleState` is WRONG → use `RmmGranuleState`
    - `RmmEmulatableAbort` is WRONG → use `RmmRecEmulatableAbort`
    - `RmmFeatureBool` / `FeatureFalse` are WRONG → use `RmmFeature`
    - `RMM_REALM_MEASUREMENT_WIDTH`, `ADDRESS_WIDTH`, `Zeros`, `ResultGetErr1`, `IsPsciFunction`, `CookieIsValid`, `RttBase`, `RttStartLevel`, `ImplFeatures_feat_da_eq_true` — these are invented; do NOT use them.
    - Rule: if you cannot find symbol X word-for-word in the provided context text, do NOT use X. Instead apply the substitution strategies below.
- When a symbol is not in context, use these concrete substitutions (do NOT write natural language prose like "Cookie is invalid"):
    - Need a zero value → write `== 0` directly, NOT `Zeros(n)` or `Zeros(WIDTH)`
    - Need a feature flag check → access the field directly: `ImplFeatures(s).feat_X == FEATURE_TRUE`, NOT `ImplFeatures_feat_X_eq_true(s)`
    - Need an RTT error level payload → inline `RttWalk(s, ...).level as int`, NOT `ResultGetErr1(result)`
    - Need a complex postcondition with no clear helper → simplify or omit the clause entirely (drop it), NOT invent a helper
    - Spec is fully unconstrained or helper is completely unknown → return `true` for that clause or the whole function
- CRITICAL — result success pattern: `RmiStatusCode` has ONLY error variants (RMI_ERROR_INPUT, RMI_ERROR_REALM, RMI_ERROR_REC, RMI_ERROR_RTT, etc.). There is NO `RMI_SUCCESS`, `RMI_OK`, `RSI_SUCCESS`, or `RSI_OK` variant. Express success as `result.is_Ok()`, never as `result == RMI_SUCCESS` or `result == RMI_OK`.
- CRITICAL — no UInt() cast function: `UInt`, `UInt32`, `UInt64` etc. are TYPE ALIASES only, not callable functions. There is no `UInt(x)` conversion. For integer/address bounds checks, write the comparison directly on the value (e.g., `(addr as int) < 0x1_0000_0000_0000` or `addr >= (1u64 << 48)`), NOT `UInt(addr) >= (1 << 48)`.
- If a convenient abstraction is not explicitly available in context/spec, write the condition directly or return `true` rather than fabricating a new helper.
- Keep predicate/function arity consistent with provided context signatures.
- Prefer precise implication style over free-form narrative or comments.
- If the command semantics are effectively unconstrained in spec text, return `true`.
- For pure query / feature / version / count / get_* commands, do not force a full failure/success scaffold unless the context explicitly describes state changes; many of these should remain minimal or `true`.
- **Fully unconstrained specs rule**: If you find NO meaningful constraints in the spec text for the command (e.g., pure query, feature detector, version/status check with no state change), return `true` directly. Do NOT attempt to fabricate constraint logic. Examples: PSCI_FEATURES (fully unconstrained, oracle returns `true`), RSI_FEATURES, version queries → all should return `true`.
- **Array/collection initialization rule**: When initializing array/list/container elements (e.g., realm.measurements[i], entries[j]):
    * If a specific initializer predicate exists in context (e.g., RimInit), use it
    * If NO initializer exists in context, do NOT fabricate one (e.g., ZeroRealmMeasurement, Zeros, etc.)
    * Instead: simplify the postcondition or replace entire initialization clause with `true`
- **Constant naming discipline rule** (CRITICAL): Enum variants and constants are EXACT in preamble — copy verbatim, NO case transformation.
    * Pattern examples: RSI_ERROR_INPUT (not RsiErrorInput), RMI_ERROR_RTT (not RmiErrorRtt), MEM_PERM_LOCKED (not MemPermLocked)
    * Model must NOT apply PascalCase/snake_case conversions to context symbols
    * If unsure of exact name, do not guess; instead replace the clause with `true`
- **No namespace prefixes on constants** (CRITICAL): Enum variants and constants in preamble are bare symbols (RD, PAS_NS, RSI_ERROR_INPUT, VDEV_LOCKED, DEV_COMM_IDLE, etc.), NOT namespaced.
    * WRONG: `RmmGranuleState::RD`, `RmmPas::PAS_NS`, `RmmVdevState::VDEV_LOCKED`, `RsiStatusCode::RSI_ERROR_INPUT`
    * CORRECT: `RD`, `PAS_NS`, `VDEV_LOCKED`, `RSI_ERROR_INPUT`
    * Use constants exactly as they appear in context, without module::constant syntax

Output self-check (must pass before final output):
- Family consistency: command prefix and symbol family must match (do not mix pdev/vdev, psmmu/vsmmu, etc.).
- Domain symbol anchoring for complex commands: for pdev/vdev/rtt/psmmu families, prefer helper/accessor names already present in context and avoid analogy-based renaming.
- Symbol whitelist discipline: every called helper/predicate/type should be from provided context/spec text (or function parameters); do not introduce unseen names. For each identifier X in the body, verify it appears verbatim in context — if not, replace the clause with `true`.
- Anti-hallucination name check: scan for analogy-constructed names (e.g., stripping a prefix, adding Bool/False/State suffix). These are almost always wrong. Use only names found verbatim in context.
- Signature and type alignment: follow context/spec conventions; if uncertain, preserve semantically correct clauses rather than forcing cosmetic ordering/alias changes.
- State-parameter discipline: preserve oracle/context state style; do not collapse `old_s, new_s` into a single `s: S` parameter.
- Naming consistency: function name should remain lowercase snake_case and match {cmd_name_lower}_spec.
- Parse sanity: ensure balanced delimiters () [] {} and a single closing brace for the function.
- No invented success variant: reject any use of `RMI_SUCCESS`, `RMI_OK`, `RSI_SUCCESS`, `RSI_OK` — these do not exist; replace with `result.is_Ok()`.
- No UInt() call: reject any expression of the form `UInt(...)` — it is not a function; remove it and write the integer expression directly.
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

Output ONLY one complete function item, with no extra text before or after."""

# 加一个auto-fix

PROMPT_V3_TEMPLATE = """{context}

{spec}

Signature: pub open spec fn {cmd_name_lower}_spec(...) -> bool
Prefer Bits64/UInt64/UInt32 aliases when present in context/spec, but do not sacrifice semantic correctness for alias formatting.
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
