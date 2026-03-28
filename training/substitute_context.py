#!/usr/bin/env python3
"""
substitute_context.py

Cascaded training context substitution.

After training the L2 (type) model, run it on all train PDFs to generate
predicted type definitions. This script replaces the golden preamble context
in command training examples with the model-generated type context.

Output: dataset/train_cascaded.jsonl — for use in command model training.

Usage (run on GPU server after L2 training):
    python3 substitute_context.py \\
        --input  dataset/train.jsonl \\
        --gen-dir generated_types/ \\
        --output dataset/train_cascaded.jsonl

Where generated_types/ contains {version}_types.rs files produced by running
the L2 model on each training version's type sections.
"""

import argparse
import json
import os
import re
import sys

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

PREAMBLE_TAIL_LINES = 200


def load_generated_context(gen_dir: str, version: str) -> str | None:
    """
    Load model-generated type text for a version.
    Expected file: {gen_dir}/{version}_types.rs
    """
    path = os.path.join(gen_dir, f"{version}_types.rs")
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        return fh.read().strip()


def replace_context_in_message(user_content: str, new_context: str) -> str:
    """
    Replace the preamble snippet in a user message.

    The user message format for command examples is:
        ## Context (shared Verus types and helper function signatures)

        ```rust
        {preamble}
        ```

        ## Command Specification ...

    We replace the content between the ```rust fences.
    """
    pattern = re.compile(
        r"(## Context[^\n]*\n\n```rust\n)(.*?)(\n```\n)",
        re.DOTALL,
    )
    replacement = rf"\g<1>{new_context}\g<3>"
    new_content, n = pattern.subn(replacement, user_content)
    if n == 0:
        # No context block found — return unchanged
        return user_content
    return new_content


def main():
    parser = argparse.ArgumentParser(
        description="Replace golden L2 context with model-generated context"
    )
    parser.add_argument("--input",   default=os.path.join(BASE_DIR, "dataset", "train.jsonl"),
                        help="Input JSONL file (command examples)")
    parser.add_argument("--gen-dir", required=True,
                        help="Directory with {version}_types.rs generated files")
    parser.add_argument("--output",  default=os.path.join(BASE_DIR, "dataset", "train_cascaded.jsonl"),
                        help="Output JSONL file")
    args = parser.parse_args()

    if not os.path.exists(args.input):
        print(f"[ERROR] Input file not found: {args.input}", file=sys.stderr)
        sys.exit(1)

    if not os.path.exists(args.gen_dir):
        print(f"[ERROR] Gen dir not found: {args.gen_dir}", file=sys.stderr)
        sys.exit(1)

    # Cache generated contexts by version
    gen_cache: dict[str, str | None] = {}

    n_total = 0
    n_substituted = 0
    n_skipped = 0

    out_examples = []
    with open(args.input) as fh:
        for line in fh:
            line = line.strip()
            if not line:
                continue
            ex = json.loads(line)
            n_total += 1

            meta = ex.get("metadata", {})
            kind = meta.get("kind", "command")
            version = meta.get("version", "")

            # Only substitute context for command examples (not type or helper examples)
            if kind in ("type_definition", "helper_stub"):
                out_examples.append(ex)
                continue

            if version not in gen_cache:
                gen_cache[version] = load_generated_context(args.gen_dir, version)

            gen_context = gen_cache[version]
            if gen_context is None:
                # No generated context for this version — keep original
                n_skipped += 1
                out_examples.append(ex)
                continue

            # Replace context in user message
            new_ex = dict(ex)
            new_messages = list(ex["messages"])
            user_idx = next(
                (i for i, m in enumerate(new_messages) if m["role"] == "user"), None
            )
            if user_idx is not None:
                new_user_content = replace_context_in_message(
                    new_messages[user_idx]["content"], gen_context
                )
                new_messages = list(new_messages)
                new_messages[user_idx] = dict(new_messages[user_idx])
                new_messages[user_idx]["content"] = new_user_content
                new_ex["messages"] = new_messages
                new_ex["metadata"] = dict(meta, cascaded=True)
                n_substituted += 1

            out_examples.append(new_ex)

    with open(args.output, "w") as fh:
        for ex in out_examples:
            fh.write(json.dumps(ex, ensure_ascii=False) + "\n")

    print(f"Processed {n_total} examples:")
    print(f"  Substituted : {n_substituted}")
    print(f"  Skipped (no generated context): {n_skipped}")
    print(f"  Written → {args.output}")


if __name__ == "__main__":
    main()
