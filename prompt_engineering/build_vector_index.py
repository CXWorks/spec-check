#!/usr/bin/env python3
"""
Build vector index for Rust coding rules using fastembed (ONNX, no PyTorch).

Dependencies: fastembed, numpy

Usage:
    python3 build_vector_index.py --input rules.jsonl --output index
"""

import json
import argparse
import os
import numpy as np

try:
    from fastembed import TextEmbedding
except ImportError:
    print("Error: fastembed not installed.")
    print("Install with: pip install fastembed")
    exit(1)


def load_rules(rules_path):
    """Load rules from JSONL file."""
    rules = []
    with open(rules_path, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            if line:
                rules.append(json.loads(line))
    return rules


def build_index(rules, model_name="sentence-transformers/all-MiniLM-L6-v2"):
    """Build vector index for rules."""
    print(f"Loading embedding model: {model_name}")
    model = TextEmbedding(model_name=model_name)
    
    print(f"Vectorizing {len(rules)} rules...")
    contents = [rule.get("content", "") for rule in rules]
    embeddings = np.array(list(model.embed(contents)))
    
    print(f"Embeddings shape: {embeddings.shape}")
    
    index_data = {
        "rules": rules,
        "embeddings": embeddings,
        "model_name": model_name,
        "num_rules": len(rules),
    }
    
    return index_data


def save_index(index_data, output_path):
    """Save index to JSON (rules/metadata) and npz (embeddings) files."""
    # Strip extensions so users can pass either a base name or a full path
    base = output_path
    for ext in (".json", ".npz", ".pkl"):
        if base.endswith(ext):
            base = base[: -len(ext)]
            break

    rules_path = base + ".json"
    embeddings_path = base + ".npz"

    metadata = {
        "rules": index_data["rules"],
        "model_name": index_data["model_name"],
        "num_rules": index_data["num_rules"],
    }
    with open(rules_path, 'w', encoding='utf-8') as f:
        json.dump(metadata, f, ensure_ascii=False, indent=2)

    np.savez(embeddings_path, embeddings=index_data["embeddings"], allow_pickle=False)

    print(f"Index saved to {rules_path} and {embeddings_path}")
    print(f"  - {index_data['num_rules']} rules")
    print(f"  - Embedding shape: {index_data['embeddings'].shape}")


def main():
    parser = argparse.ArgumentParser(description="Build vector index for Rust coding rules")
    parser.add_argument(
        "--input",
        type=str,
        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "rules.jsonl"),
        help="Path to rules JSONL file (default: rules.jsonl in script directory)",
    )
    parser.add_argument(
        "--output",
        type=str,
        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "index"),
        help="Base path to save vector index (default: index in script directory, writes index.json + index.npz)",
    )
    parser.add_argument(
        "--model",
        type=str,
        default="sentence-transformers/all-MiniLM-L6-v2",
        help="fastembed model name (default: sentence-transformers/all-MiniLM-L6-v2)",
    )
    
    args = parser.parse_args()
    
    # Validate input file
    if not os.path.exists(args.input):
        print(f"Error: Input file not found: {args.input}")
        exit(1)
    
    # Load rules
    print(f"Loading rules from {args.input}")
    rules = load_rules(args.input)
    print(f"Loaded {len(rules)} rules")
    
    # Build index
    index_data = build_index(rules, args.model)
    
    # Save index
    save_index(index_data, args.output)
    print("Done!")


if __name__ == "__main__":
    main()
