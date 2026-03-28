#!/usr/bin/env python3
"""
build_dataset.py

Builds a fine-tuning dataset by aligning:
  - Input:  per-command PDF section text  (sections/{version}/{cmd}_command.txt)
            + shared preamble context     (specs/{version}/preamble.rs)
  - Output: per-command Verus spec        (specs/{version}/{cmd}_spec.rs)

Split strategy (only 4 versions available):
  train  — eac5, rel0, alp11   (~82 % of examples)
  val    — alp12               (~18 % of examples)

Each JSONL line is one example in the OpenAI / HuggingFace chat format:
{
  "messages": [
    {"role": "system",  "content": "<system prompt>"},
    {"role": "user",    "content": "<preamble snippet + pdf section>"},
    {"role": "assistant","content": "<verus spec function>"}
  ],
  "metadata": {
    "version": "eac5",
    "command": "RMI_DATA_CREATE",
    "source_section": "RMI_DATA_CREATE_command.txt"
  }
}

Usage:
    python3 build_dataset.py
"""

import os
import json

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------
BASE_DIR   = os.path.dirname(os.path.abspath(__file__))
DATASET_DIR = os.path.join(BASE_DIR, "dataset")

TRAIN_VERSIONS = ["eac5", "rel0", "alp11", "alp12"]
VAL_VERSIONS   = ["alp13"]
TEST_VERSIONS  = ["alp14"]

SYSTEM_PROMPT_HELPERS = (
    "You are a formal specification assistant for Arm CCA (Confidential Compute "
    "Architecture) Realm Management Monitor (RMM). "
    "Given the specification text for an RMM helper function in ASL pseudocode, "
    "generate the Verus uninterpreted spec function stub (a single line ending with ';'). "
    "Output only the stub declaration in valid Verus syntax."
)

SYSTEM_PROMPT_TYPES = (
    "You are a formal specification assistant for Arm CCA (Confidential Compute "
    "Architecture) Realm Management Monitor (RMM). "
    "Given the specification text for an RMM type definition (enumeration, structure, "
    "or fieldset), generate the corresponding Verus/Rust type definition. "
    "Output only the type definition (pub enum or struct block) in valid Verus syntax."
)

SYSTEM_PROMPT = (
    "You are a formal specification assistant for Arm CCA (Confidential Compute "
    "Architecture) Realm Management Monitor (RMM). "
    "Given the specification text for an RMM command and the shared Verus type/function "
    "context (preamble), generate the Verus specification function for that command. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in "
    "valid Verus syntax."
)

# Preamble can be very long; truncate to the last N lines which contain the
# helper function signatures most relevant to the body (the enums/structs are
# earlier and rarely referenced directly in spec bodies).
PREAMBLE_TAIL_LINES = 200


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
def load_preamble(version: str, cascaded_dir: str | None = None) -> str:
    if cascaded_dir:
        # Load model-generated type context instead of golden preamble
        gen_path = os.path.join(cascaded_dir, f"{version}_types.rs")
        if os.path.exists(gen_path):
            with open(gen_path) as fh:
                return fh.read().strip()
    path = os.path.join(BASE_DIR, "specs", version, "preamble.rs")
    if not os.path.exists(path):
        return ""
    with open(path) as fh:
        lines = fh.readlines()
    # Keep only the tail (helper function signatures) to avoid ballooning context
    tail = lines[-PREAMBLE_TAIL_LINES:]
    return "".join(tail).strip()


def load_section(version: str, cmd_name: str) -> str | None:
    """Load the raw PDF section text for a command (e.g. 'RMI_DATA_CREATE')."""
    filename = cmd_name + "_command.txt"
    path = os.path.join(BASE_DIR, "sections", version, filename)
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        return fh.read().strip()


def load_spec(version: str, cmd_name: str) -> str | None:
    """Load the Verus _spec function text."""
    filename = cmd_name.lower() + "_spec.rs"
    path = os.path.join(BASE_DIR, "specs", version, filename)
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        text = fh.read().strip()
    # Skip excluded commands (coverage-patch marker)
    if "[EXCLUDED]" in text:
        return None
    return text


def list_commands(version: str) -> list[str]:
    """Return sorted list of command names (uppercase, no ' command' suffix)."""
    sec_dir = os.path.join(BASE_DIR, "sections", version)
    cmds = []
    for fname in sorted(os.listdir(sec_dir)):
        if fname.endswith("_command.txt"):
            # e.g. "RMI_DATA_CREATE_command.txt" → "RMI_DATA_CREATE"
            cmds.append(fname[:-12])
    return cmds


def build_user_message(preamble: str, section_text: str) -> str:
    return (
        "## Context (shared Verus types and helper function signatures)\n\n"
        f"```rust\n{preamble}\n```\n\n"
        "## Command Specification (from RMM spec PDF)\n\n"
        f"{section_text}"
    )


def load_type_section(version: str, type_name: str) -> str | None:
    path = os.path.join(BASE_DIR, "sections", version, "types", f"{type_name}.txt")
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        return fh.read().strip()


def load_type_verus(version: str, type_name: str) -> str | None:
    path = os.path.join(BASE_DIR, "specs", version, "types", f"{type_name}.rs")
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        return fh.read().strip()


def list_types(version: str) -> list[str]:
    types_dir = os.path.join(BASE_DIR, "specs", version, "types")
    if not os.path.exists(types_dir):
        return []
    return sorted(f[:-3] for f in os.listdir(types_dir) if f.endswith(".rs"))


def make_type_example(version: str, type_name: str) -> dict | None:
    section_text = load_type_section(version, type_name)
    if section_text is None:
        return None  # no PDF section for this type (e.g. RmmSystemRegisters is hardcoded)
    verus_text = load_type_verus(version, type_name)
    if verus_text is None:
        return None

    user_content = (
        "## Type Specification (from RMM spec PDF)\n\n"
        f"{section_text}"
    )
    return {
        "messages": [
            {"role": "system",    "content": SYSTEM_PROMPT_TYPES},
            {"role": "user",      "content": user_content},
            {"role": "assistant", "content": verus_text},
        ],
        "metadata": {
            "version":   version,
            "type":      type_name,
            "kind":      "type_definition",
        },
    }


def load_helper_section(version: str, fn_name: str) -> str | None:
    path = os.path.join(BASE_DIR, "sections", version, "helpers", f"{fn_name}.txt")
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        return fh.read().strip()


def load_helper_stub(version: str, fn_name: str) -> str | None:
    path = os.path.join(BASE_DIR, "specs", version, "helpers", f"{fn_name}.rs")
    if not os.path.exists(path):
        return None
    with open(path) as fh:
        return fh.read().strip()


def list_helpers(version: str) -> list[str]:
    helpers_dir = os.path.join(BASE_DIR, "specs", version, "helpers")
    if not os.path.exists(helpers_dir):
        return []
    return sorted(f[:-3] for f in os.listdir(helpers_dir) if f.endswith(".rs"))


def make_helper_example(version: str, fn_name: str) -> dict | None:
    section_text = load_helper_section(version, fn_name)
    if section_text is None:
        return None
    stub_text = load_helper_stub(version, fn_name)
    if stub_text is None:
        return None

    user_content = (
        "## Helper Function Specification (from RMM spec PDF)\n\n"
        f"{section_text}"
    )
    return {
        "messages": [
            {"role": "system",    "content": SYSTEM_PROMPT_HELPERS},
            {"role": "user",      "content": user_content},
            {"role": "assistant", "content": stub_text},
        ],
        "metadata": {
            "version":   version,
            "function":  fn_name,
            "kind":      "helper_stub",
        },
    }


def make_example(version: str, cmd_name: str, preamble: str) -> dict | None:
    section_text = load_section(version, cmd_name)
    if section_text is None:
        return None
    spec_text = load_spec(version, cmd_name)
    if spec_text is None:
        return None

    return {
        "messages": [
            {"role": "system",    "content": SYSTEM_PROMPT},
            {"role": "user",      "content": build_user_message(preamble, section_text)},
            {"role": "assistant", "content": spec_text},
        ],
        "metadata": {
            "version":        version,
            "command":        cmd_name,
            "source_section": f"{cmd_name}_command.txt",
        },
    }


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def build_split(versions: list[str], split_name: str, cascaded_dir: str | None = None):
    os.makedirs(DATASET_DIR, exist_ok=True)
    out_path = os.path.join(DATASET_DIR, f"{split_name}.jsonl")

    examples = []
    preamble_cache: dict[str, str] = {}

    for version in versions:
        if version not in preamble_cache:
            preamble_cache[version] = load_preamble(version, cascaded_dir)
        preamble = preamble_cache[version]

        # Command spec examples
        for cmd_name in list_commands(version):
            ex = make_example(version, cmd_name, preamble)
            if ex is None:
                print(f"  [SKIP] {version}/{cmd_name}")
                continue
            examples.append(ex)

        # Type definition examples
        for type_name in list_types(version):
            ex = make_type_example(version, type_name)
            if ex is None:
                continue  # type has no corresponding PDF section (hardcoded layer)
            examples.append(ex)

        # Helper function stub examples
        for fn_name in list_helpers(version):
            ex = make_helper_example(version, fn_name)
            if ex is None:
                continue
            examples.append(ex)

    with open(out_path, "w") as fh:
        for ex in examples:
            fh.write(json.dumps(ex, ensure_ascii=False) + "\n")

    print(f"  {split_name}: {len(examples)} examples → {out_path}")
    return examples


def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--cascaded-context", metavar="DIR",
                        help="Directory with model-generated type files "
                             "({version}_types.rs) to use instead of golden preamble")
    args = parser.parse_args()
    cascaded_dir = args.cascaded_context

    label = " (cascaded)" if cascaded_dir else ""
    print(f"Building train split{label}...")
    train_exs = build_split(TRAIN_VERSIONS, "train", cascaded_dir)

    print("Building val split...")
    val_exs = build_split(VAL_VERSIONS, "val")

    print("Building test split...")
    test_exs = build_split(TEST_VERSIONS, "test")

    total = len(train_exs) + len(val_exs) + len(test_exs)
    print(f"\nDataset summary:")
    print(f"  train : {len(train_exs):4d} examples  ({', '.join(TRAIN_VERSIONS)})")
    print(f"  val   : {len(val_exs):4d} examples  ({', '.join(VAL_VERSIONS)})")
    print(f"  test  : {len(test_exs):4d} examples  ({', '.join(TEST_VERSIONS)})")
    print(f"  total : {total:4d} examples")

    from collections import Counter
    for split_name, exs in [("train", train_exs), ("val", val_exs), ("test", test_exs)]:
        counts = Counter(ex["metadata"]["version"] for ex in exs)
        for v, n in sorted(counts.items()):
            print(f"    {split_name}/{v}: {n}")

    # Write helpers-only train split for layer 3 training
    helpers_only = [ex for ex in train_exs if ex["metadata"].get("kind") == "helper_stub"]
    if helpers_only:
        helpers_path = os.path.join(DATASET_DIR, "train_helpers.jsonl")
        with open(helpers_path, "w") as fh:
            for ex in helpers_only:
                fh.write(json.dumps(ex, ensure_ascii=False) + "\n")
        print(f"\n  helpers: {len(helpers_only)} examples → {helpers_path}")

    print("\nDone.")


if __name__ == "__main__":
    main()
