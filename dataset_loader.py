#!/usr/bin/env python3
"""
Phase 1: Dataset Loader & Pipeline Skeleton

Tasks:
1. Load dataset from JSONL (test.jsonl / train.jsonl)
2. Extract spec text (user message) → oracle code (assistant message)
3. Create inference pipeline skeleton with fake model
4. Verify pipeline end-to-end
"""

import json
import sys
from pathlib import Path
from typing import List, Dict, Any

# ============================================================================
# Dataset Loader
# ============================================================================

class SpecOracle:
    """Single sample: spec text + oracle (ground truth) Verus code"""
    def __init__(self, spec: str, oracle: str, version: str, command: str):
        self.spec = spec
        self.oracle = oracle
        self.version = version
        self.command = command

    def __repr__(self):
        spec_preview = self.spec[:100].replace('\n', ' ') + "..."
        oracle_preview = self.oracle[:80].replace('\n', ' ') + "..."
        return f"SpecOracle(cmd={self.command}, v={self.version}, spec={spec_preview}, oracle={oracle_preview})"


def load_dataset(jsonl_path: str) -> List[SpecOracle]:
    """
    Load dataset from JSONL file.
    Expected format:
    {
        "messages": [
            {"role": "system", "content": "..."},
            {"role": "user", "content": "...spec text..."},
            {"role": "assistant", "content": "...oracle verus code..."}
        ],
        "metadata": {
            "version": "alp14",
            "command": "PSCI_AFFINITY_INFO",
            "source_section": "..."
        }
    }
    """
    dataset = []
    path = Path(jsonl_path)
    
    if not path.exists():
        print(f"File not found: {jsonl_path}")
        return dataset
    
    with open(path, 'r') as f:
        for line_num, line in enumerate(f, 1):
            try:
                data = json.loads(line)
                
                # Extract messages
                messages = data.get("messages", [])
                metadata = data.get("metadata", {})
                
                # Find user (spec) and assistant (oracle) messages
                spec_text = None
                oracle_code = None
                for msg in messages:
                    if msg.get("role") == "user":
                        spec_text = msg.get("content", "")
                    elif msg.get("role") == "assistant":
                        oracle_code = msg.get("content", "")
                
                if spec_text and oracle_code:
                    sample = SpecOracle(
                        spec=spec_text,
                        oracle=oracle_code,
                        version=metadata.get("version", "unknown"),
                        command=metadata.get("command", "unknown")
                    )
                    dataset.append(sample)
                else:
                    print(f"Line {line_num}: Missing user or assistant message")
            
            except json.JSONDecodeError as e:
                print(f"Line {line_num}: JSON decode error: {e}")
            except Exception as e:
                print(f"Line {line_num}: Unexpected error: {e}")
    
    print(f"Loaded {len(dataset)} samples from {jsonl_path}")
    return dataset


# ============================================================================
# Task 2: Fake Model (for pipeline verification)
# ============================================================================

class FakeModel:
    """
    Placeholder model that returns a dummy output.
    Used to verify pipeline structure before connecting real Claude API.
    Node: skip this for now
    """
    def __init__(self, mode: str = "dummy", oracle_dict: Dict[str, str] = None):
        """
        mode:
          - "dummy": Return placeholder code
          - "echo_oracle": Return oracle (cheating, for testing evaluation)
          - "empty": Return empty string
        oracle_dict: For echo_oracle mode, map spec → oracle (pass from outside)
        """
        self.mode = mode
        self.call_count = 0
        self.oracle_dict = oracle_dict or {}
    
    def generate(self, spec_text: str, oracle: str = None) -> str:
        """Generate Verus code from spec text."""
        self.call_count += 1
        
        if self.mode == "echo_oracle":
            # This is cheating - for testing evaluation logic
            # Return the oracle directly (pass it as argument)
            return oracle if oracle else "ERROR: no oracle provided"
        elif self.mode == "dummy":
            # Realistic dummy response
            return "pub open spec fn placeholder_spec(...) -> bool {\n  true\n}"
        elif self.mode == "empty":
            return ""
        else:
            raise ValueError(f"Unknown mode: {self.mode}")


# ============================================================================
# Task 3: Evaluation Skeleton
# ============================================================================

class EvaluationMetrics:
    """Metrics for a single sample"""
    def __init__(self, command: str, version: str):
        self.command = command
        self.version = version
        self.generated = None
        self.oracle = None
        self.exact_match = False
        self.error_msg = None
    
    def evaluate(self, generated: str, oracle: str):
        """Basic evaluation: exact match"""
        self.generated = generated
        self.oracle = oracle
        self.exact_match = (generated.strip() == oracle.strip())
    
    def __repr__(self):
        status = "good" if self.exact_match else "not good"
        return f"{status} {self.command} ({self.version}): exact_match={self.exact_match}"


def evaluate_sample(generated: str, oracle: str) -> EvaluationMetrics:
    """Evaluate a single sample"""
    # NOTE: will add more sophisticated evaluation (semantic match, constraint coverage, etc.) later
    metric = EvaluationMetrics("temp", "temp")
    metric.evaluate(generated, oracle)
    return metric


# ============================================================================
# Task 4: Pipeline Skeleton
# ============================================================================

def run_pipeline(dataset: List[SpecOracle], model: FakeModel, limit: int = None) -> Dict[str, Any]:
    """
    End-to-end pipeline:
      spec → model → evaluation
    
    Args:
        dataset: List of SpecOracle samples
        model: Model (fake or real)
        limit: Max samples to process (None = all)
    
    Returns:
        Summary dict with metrics
    """
    if limit is not None:
        dataset = dataset[:limit]
    
    results = []
    summary = {
        "total": len(dataset),
        "exact_matches": 0,
        "errors": 0,
        "by_version": {},
        "by_command": {}
    }
    
    print(f"\n{'='*70}")
    print(f"Running pipeline on {len(dataset)} samples (model mode: {model.mode})")
    print(f"{'='*70}\n")
    
    for i, sample in enumerate(dataset, 1):
        # Generate
        try:
            generated = model.generate(sample.spec, oracle=sample.oracle)
        except Exception as e:
            print(f"[{i}/{len(dataset)}] Error generating for {sample.command}: {e}")
            summary["errors"] += 1
            continue
        
        # Evaluate
        metric = EvaluationMetrics(sample.command, sample.version)
        metric.evaluate(generated, sample.oracle)
        results.append(metric)
        
        # Update summary
        if metric.exact_match:
            summary["exact_matches"] += 1
        
        # By version
        if sample.version not in summary["by_version"]:
            summary["by_version"][sample.version] = {"total": 0, "correct": 0}
        summary["by_version"][sample.version]["total"] += 1
        if metric.exact_match:
            summary["by_version"][sample.version]["correct"] += 1
        
        # By command
        if sample.command not in summary["by_command"]:
            summary["by_command"][sample.command] = {"total": 0, "correct": 0}
        summary["by_command"][sample.command]["total"] += 1
        if metric.exact_match:
            summary["by_command"][sample.command]["correct"] += 1
        
        # Print progress
        if i % 5 == 0 or i == len(dataset):
            print(f"[{i}/{len(dataset)}] {metric}")
    
    # Aggregate summary
    summary["accuracy"] = summary["exact_matches"] / len(dataset) if len(dataset) > 0 else 0
    
    print(f"\n{'='*70}")
    print(f"Summary:")
    print(f"  Total samples: {summary['total']}")
    print(f"  Exact matches: {summary['exact_matches']}")
    print(f"  Accuracy: {summary['accuracy']:.1%}")
    print(f"  Errors: {summary['errors']}")
    print(f"\nBy version:")
    for version, counts in summary["by_version"].items():
        acc = counts["correct"] / counts["total"] if counts["total"] > 0 else 0
        print(f"  {version}: {counts['correct']}/{counts['total']} ({acc:.1%})")
    print(f"\nTop 10 commands:")
    cmd_list = sorted(summary["by_command"].items(), key=lambda x: x[1]["total"], reverse=True)[:10]
    for cmd, counts in cmd_list:
        acc = counts["correct"] / counts["total"] if counts["total"] > 0 else 0
        print(f"  {cmd}: {counts['correct']}/{counts['total']} ({acc:.1%})")
    print(f"{'='*70}\n")
    
    return {
        "summary": summary,
        "results": results
    }


# ============================================================================
# Main
# ============================================================================

def main():
    # Determine dataset path
    dataset_dir = Path("/Users/xiangzhushan/Desktop/spec-check/training-dataset/dataset")
    test_jsonl = dataset_dir / "test.jsonl"
    
    if len(sys.argv) > 1:
        test_jsonl = Path(sys.argv[1])
    
    print(f"Loading dataset from: {test_jsonl}")
    
    # Load dataset
    dataset = load_dataset(str(test_jsonl))
    if not dataset:
        print("No data loaded. Exiting.")
        return
    
    # Show sample
    print(f"\nFirst sample:")
    sample = dataset[0]
    print(f"  Command: {sample.command} (v{sample.version})")
    print(f"  Spec length: {len(sample.spec)} chars")
    print(f"  Oracle length: {len(sample.oracle)} chars")
    print(f"  Spec preview: {sample.spec[:120].replace(chr(10), ' ')}...\n")
    
    # Run pipeline with fake model
    print("\nTesting pipeline with FAKE model (mode=dummy)...")
    model = FakeModel(mode="dummy")
    result = run_pipeline(dataset, model, limit=10)
    
    print("\nTesting pipeline with FAKE model (mode=echo_oracle) - should get 100% match...")
    model = FakeModel(mode="echo_oracle")
    result = run_pipeline(dataset, model, limit=10)


if __name__ == "__main__":
    main()
