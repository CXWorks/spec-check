#!/usr/bin/env python3
"""
Phase 2: Prompt Engineering & Claude Integration

1. Extract current prompt from train.jsonl
2. Design 5 prompt variants
3. A/B test with small sample (N=10)
4. Select best, run full batch with Claude API
"""

import json
import sys
from pathlib import Path
from typing import List, Dict, Any, Callable
from dataset_loader import SpecOracle, load_dataset, EvaluationMetrics, load_preamble

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
    
    def format(self, spec: str, context: str, cmd_name: str) -> Dict[str, str]:
        """Format messages for API call"""
        user_content = self.user_template.format(
            system_prompt=self.system if "{system_prompt}" in self.user_template else "",
            context=context,
            spec=spec,
            cmd_name=cmd_name
        )
        return {
            "system": self.system,
            "user": user_content
        }


# All prompt variants
PROMPTS = {
    "v0": PromptVariant("V0-Baseline", PROMPT_V0_SYSTEM, PROMPT_V0_TEMPLATE),
    "v1": PromptVariant("V1-Minimal", PROMPT_V1_SYSTEM, PROMPT_V1_TEMPLATE),
    "v2": PromptVariant("V2-FewShot", PROMPT_V2_SYSTEM, PROMPT_V2_TEMPLATE),
    "v3": PromptVariant("V3-Structured", PROMPT_V3_SYSTEM, PROMPT_V3_TEMPLATE),
    "v4": PromptVariant("V4-BestPractices", PROMPT_V4_SYSTEM, PROMPT_V4_TEMPLATE),
}


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
        
        try:
            # Extract system message
            system_msg = None
            user_msgs = []
            for msg in messages:
                if msg.get("role") == "system":
                    system_msg = msg.get("content")
                else:
                    user_msgs.append(msg)
            
            # Call Claude API
            response = self.client.messages.create(
                model=self.name,
                max_tokens=2048,
                system=system_msg,
                messages=user_msgs
            )
            
            return response.content[0].text.strip()
        except Exception as e:
            print(f"Claude API error (call #{self.call_count}): {e}")
            return "ERROR"


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
    dataset: List[SpecOracle],
    model: Callable,
    limit: int = 10
) -> Dict[str, Any]:
    """
    Test a single prompt variant on small sample.
    Returns: accuracy, average coverage, etc.
    """
    results = []
    total_exact_match = 0
    total_coverage = 0.0
    
    print(f"\n  Testing {prompt_variant.name}...")
    
    for i, sample in enumerate(dataset[:limit], 1):
        # Raw section text is the spec; preamble is the context
        context = sample.preamble
        spec = sample.section_text
        
        # Format messages using prompt variant
        messages_dict = prompt_variant.format(spec, context, sample.command)
        messages = [
            {"role": "system", "content": messages_dict["system"]},
            {"role": "user", "content": messages_dict["user"]}
        ]
        
        # Generate using Claude API
        generated = model.generate(messages)
        
        # Evaluate
        metric = EvaluationMetricsV2(sample.command, sample.version)
        metric.evaluate(generated, sample.oracle)
        results.append(metric)
        
        if metric.exact_match:
            total_exact_match += 1
        total_coverage += metric.coverage
        
        if i % 5 == 0:
            print(f"    [{i}/{limit}] {metric}")
    
    accuracy = total_exact_match / len(results) if results else 0
    avg_coverage = total_coverage / len(results) if results else 0
    
    return {
        "prompt": prompt_variant.name,
        "accuracy": accuracy,
        "avg_coverage": avg_coverage,
        "results": results
    }


def run_ab_testing(dataset: List[SpecOracle], limit: int = 10, api_key: str = None):
    """
    A/B test all prompt variants.
    Find the best one.
    """
    print(f"\n{'='*70}")
    print(f"A/B Testing {len(PROMPTS)} prompt variants (N={limit})")
    print(f"Model: Claude 4.5 Haiku")
    print(f"{'='*70}")
    
    # Create Claude model instance
    try:
        model = ClaudeHaikuModel(api_key=api_key)
        print(f"Connected to Claude API\n")
    except Exception as e:
        print(f"Failed to connect to Claude API: {e}\n")
        return None, {}
    
    all_results = {}
    for key, prompt in PROMPTS.items():
        result = evaluate_prompt_variant(prompt, dataset, model, limit=limit)
        all_results[key] = result
    
    # Summary table
    print(f"\n{'='*70}")
    print(f"Results:")
    print(f"{'='*70}")
    print(f"{'Prompt':<20} {'Accuracy':<12} {'Avg Coverage':<15}")
    print(f"{'-'*47}")
    for key in sorted(PROMPTS.keys()):
        result = all_results[key]
        print(f"{result['prompt']:<20} {result['accuracy']:.1%}         {result['avg_coverage']:.1%}")
    
    # Find best
    best_key = max(all_results.keys(), key=lambda k: all_results[k]['accuracy'])
    best_result = all_results[best_key]
    print(f"\nBest: {best_result['prompt']} (accuracy={best_result['accuracy']:.1%})")
    print(f"{'='*70}\n")
    
    return best_key, all_results


# ============================================================================
# Main
# ============================================================================

def main():
    import os
    
    # Load dataset from raw section files (not JSONL)
    print("Loading test split from raw sections...")
    dataset = load_dataset(split="test")
    
    if not dataset:
        print("No data loaded")
        return
    
    print(f"Loaded {len(dataset)} samples\n")
    
    # Get API key
    api_key = os.getenv("ANTHROPIC_API_KEY") or (
        sys.argv[1] if len(sys.argv) > 1 else None
    )
    
    if not api_key:
        print("ANTHROPIC_API_KEY not set")
        return
    
    # A/B test prompt variants
    best_key, results = run_ab_testing(dataset, limit=3, api_key=api_key)
    
    if best_key is None:
        print("A/B testing failed")
        return
    
    # Show best prompt
    best_prompt = PROMPTS[best_key]
    print(f"Best Prompt ({best_prompt.name}):")
    print(f"System:\n{best_prompt.system}\n")
    print(f"Template (first 300 chars):\n{best_prompt.user_template[:300]}...\n")
    
    # Option: Batch generate on full dataset
    if len(sys.argv) > 2 and sys.argv[2] == "--batch":
        print(f"\n{'='*70}")
        print(f"Batch generating on full dataset ({len(dataset)} samples)...")
        print(f"{'='*70}\n")
        
        model = ClaudeHaikuModel(api_key=api_key)
        batch_results = evaluate_prompt_variant(best_prompt, dataset, model, limit=len(dataset))
        
        print(f"\n{'='*70}")
        print(f"Full Dataset Results (Prompt: {best_prompt.name}):")
        print(f"  Accuracy: {batch_results['accuracy']:.1%}")
        print(f"  Avg Coverage: {batch_results['avg_coverage']:.1%}")
        print(f"{'='*70}\n")


if __name__ == "__main__":
    main()
