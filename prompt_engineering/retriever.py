#!/usr/bin/env python3
"""
Retrieve relevant rules using semantic search.

Dependencies: fastembed, numpy

Usage:
    from retriever import RuleRetriever
    
    retriever = RuleRetriever("index")
    results = retriever.search("如何命名变量", top_k=5)
    for rule in results:
        print(rule["rule_id"], rule["title"])
"""

import json
import os
import argparse
from typing import List, Dict, Any

try:
    from fastembed import TextEmbedding
    import numpy as np
except ImportError:
    print("Error: fastembed or numpy not installed.")
    print("Install with: pip install fastembed numpy")
    exit(1)


class RuleRetriever:
    """Retrieve relevant rules using semantic search."""
    
    def __init__(self, index_path):
        """Load index from JSON + npz files (base path without extension)."""
        # Strip known extensions so callers can pass either the base or a full path
        base = index_path
        for ext in (".json", ".npz", ".pkl"):
            if base.endswith(ext):
                base = base[: -len(ext)]
                break

        rules_path = base + ".json"
        embeddings_path = base + ".npz"

        if not os.path.exists(rules_path):
            raise FileNotFoundError(f"Rules file not found: {rules_path}")
        if not os.path.exists(embeddings_path):
            raise FileNotFoundError(f"Embeddings file not found: {embeddings_path}")

        print(f"Loading index from {rules_path} and {embeddings_path}")
        with open(rules_path, 'r', encoding='utf-8') as f:
            metadata = json.load(f)

        self.rules = metadata["rules"]
        self.model_name = metadata["model_name"]
        self.embeddings = np.load(embeddings_path, allow_pickle=False)["embeddings"]

        print(f"Loaded {len(self.rules)} rules with model: {self.model_name}")
        
        # Load embedding model
        self.model = TextEmbedding(model_name=self.model_name)

    def search(self, query: str, top_k: int = 5) -> List[Dict[str, Any]]:
        """
        Search for relevant rules.
        
        Args:
            query: Natural language query
            top_k: Number of results to return
        
        Returns:
            List of rules with similarity scores
        """
        # Encode query
        query_embedding = np.array(list(self.model.embed([query])))[0]
        
        # Compute similarity (cosine)
        similarities = np.dot(self.embeddings, query_embedding) / (
            np.linalg.norm(self.embeddings, axis=1) * np.linalg.norm(query_embedding)
        )
        
        # Get top-k indices
        top_indices = np.argsort(similarities)[::-1][:top_k]
        
        # Build results
        results = []
        for idx in top_indices:
            rule = self.rules[int(idx)]
            results.append({
                "rule_id": rule["rule_id"],
                "title": rule["title"],
                "category": rule["category"],
                "section": rule["section"],
                "has_lint": rule["has_lint"],
                "lint_names": rule["lint_names"],
                "score": float(similarities[idx]),
                "content": rule["content"],
            })
        
        return results


def main():
    parser = argparse.ArgumentParser(description="Search for relevant Rust coding rules")
    parser.add_argument(
        "--index",
        type=str,
        default=os.path.join(os.path.dirname(os.path.abspath(__file__)), "index"),
        help="Base path to vector index files (default: index in script directory, loads index.json + index.npz)",
    )
    parser.add_argument(
        "query",
        type=str,
        help="Search query",
    )
    parser.add_argument(
        "--top-k",
        type=int,
        default=5,
        help="Number of results to return (default: 5)",
    )
    
    args = parser.parse_args()
    
    # Initialize retriever
    retriever = RuleRetriever(args.index)
    
    # Search
    print(f"\nSearching for: {args.query}\n")
    results = retriever.search(args.query, top_k=args.top_k)
    
    # Display results
    for i, rule in enumerate(results, 1):
        print(f"{i}. [{rule['rule_id']}] {rule['title']}")
        print(f"   Category: {rule['category']} | Section: {rule['section']} | Score: {rule['score']:.4f}")
        if rule['has_lint']:
            print(f"   Lint: {', '.join(rule['lint_names'])}")
        print(f"   {rule['content'][:150]}...")
        print()


if __name__ == "__main__":
    main()
