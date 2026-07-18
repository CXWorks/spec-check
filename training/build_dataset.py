#!/usr/bin/env python3
"""
build_dataset.py

Builds a fine-tuning dataset by aligning:
  - Input:  per-command PDF section text  (sections/{version}/{cmd}_command.txt)
            + shared preamble context     (specs/{version}/preamble.rs)
  - Output: per-command Verus spec        (specs/{version}/{cmd}_spec.rs)

Command examples use the V3 system prompt (prompt_engineering/prompt_engineering_v3.py)
plus preamble+spec in the user message. Preamble is trained INTO the model here so
it learns the symbol names, even though PROMPT_V3_TEMPLATE (used at inference by
run_qwen_v3.py) no longer supplies preamble — the model is expected to already know
it from training.

Split strategy: item-based 80/10/10 split across all versions (see split_names()).

Each JSONL line is one example in the OpenAI / HuggingFace chat format:
{
  "messages": [
    {"role": "system",  "content": "<system prompt>"},
    {"role": "user",    "content": "<preamble + pdf section + signature reminder>"},
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
import random
import sys

# ---------------------------------------------------------------------------
# Config
# ---------------------------------------------------------------------------
BASE_DIR   = os.path.dirname(os.path.abspath(__file__))
DATASET_DIR = os.path.join(BASE_DIR, "dataset")

for _candidate in (os.path.join(BASE_DIR, "prompt_engineering"),        # server layout: spec-gen/build_dataset.py + spec-gen/prompt_engineering/
                   os.path.join(BASE_DIR, "..", "prompt_engineering")):  # repo layout: training/build_dataset.py + prompt_engineering/
    if os.path.isdir(_candidate):
        sys.path.insert(0, _candidate)
        break
from prompt_engineering_v3 import V3_PROMPT  # command-kind prompt: no preamble, deduped rules

ALL_VERSIONS = ["eac5", "rel0", "alp11", "alp12", "alp13", "alp14"]
EVAL_VERSION = "alp14"   # val and test items use only this version
SPLIT_SEED   = 42

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

SYSTEM_PROMPT = V3_PROMPT.system

# Preamble is trained INTO the model (embedded in every training example) so it
# learns the symbol names, even though the V3 prompt used at inference time no
# longer supplies preamble (see PROMPT_V3_TEMPLATE in prompt_engineering_v3.py).
# Truncate to the last N lines, which contain the helper function signatures
# most relevant to spec bodies (the enums/structs are earlier and rarely
# referenced directly).
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


def build_user_message(preamble: str, section_text: str, cmd_name: str) -> str:
    # Same trailing "Signature / Prefer aliases / Keep unchanged-state" reminder as
    # PROMPT_V3_TEMPLATE, but with preamble prepended — training sees preamble so the
    # model learns the symbols; inference (PROMPT_V3_TEMPLATE) no longer supplies it.
    return (
        f"{preamble}\n\n"
        f"{section_text}\n\n"
        f"Signature: pub open spec fn {cmd_name.lower()}_spec(...) -> bool\n"
        "Prefer Bits64/UInt64/UInt32 aliases when present in context/spec, but do not sacrifice semantic correctness for alias formatting.\n"
        "Keep unchanged-state constraints when implied by the command behavior."
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
            {"role": "user",      "content": build_user_message(preamble, section_text, cmd_name)},
            {"role": "assistant", "content": spec_text},
        ],
        "metadata": {
            "version":        version,
            "command":        cmd_name,
            "source_section": f"{cmd_name}_command.txt",
        },
    }


# ---------------------------------------------------------------------------
# Item-based split helpers
# ---------------------------------------------------------------------------

def split_names(names: list[str], fracs=(0.80, 0.10, 0.10), seed=SPLIT_SEED):
    """Split a sorted list of item names into (train, val, test) sets."""
    names = sorted(names)
    rng = random.Random(seed)
    rng.shuffle(names)
    n = len(names)
    n_train = int(n * fracs[0])
    n_val   = int(n * fracs[1])
    train_set = set(names[:n_train])
    val_set   = set(names[n_train:n_train + n_val])
    test_set  = set(names[n_train + n_val:])
    return train_set, val_set, test_set


def all_item_names(kind: str) -> list[str]:
    """Collect unique item names across ALL_VERSIONS for a given kind."""
    names = set()
    for version in ALL_VERSIONS:
        if kind == "command":
            names.update(list_commands(version))
        elif kind == "type":
            names.update(list_types(version))
        elif kind == "helper":
            names.update(list_helpers(version))
    return sorted(names)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--cascaded-context", metavar="DIR",
                        help="Directory with model-generated type files "
                             "({version}_types.rs) to use instead of golden preamble")
    args = parser.parse_args()
    cascaded_dir = args.cascaded_context

    os.makedirs(DATASET_DIR, exist_ok=True)

    # --- Split item names independently per kind (no overlap between train and test) ---
    cmd_train,    cmd_val,    cmd_test    = split_names(all_item_names("command"))
    type_train,   type_val,   type_test   = split_names(all_item_names("type"))
    helper_train, helper_val, helper_test = split_names(all_item_names("helper"))

    print(f"Commands — train:{len(cmd_train)} val:{len(cmd_val)} test:{len(cmd_test)}")
    print(f"Types    — train:{len(type_train)} val:{len(type_val)} test:{len(type_test)}")
    print(f"Helpers  — train:{len(helper_train)} val:{len(helper_val)} test:{len(helper_test)}")

    preamble_cache: dict[str, str] = {}

    def get_preamble(version: str) -> str:
        if version not in preamble_cache:
            preamble_cache[version] = load_preamble(version, cascaded_dir)
        return preamble_cache[version]

    train_exs: list[dict] = []
    val_exs:   list[dict] = []
    test_exs:  list[dict] = []

    # Commands: train items use all versions; val/test use EVAL_VERSION only
    for version in ALL_VERSIONS:
        preamble = get_preamble(version)
        for cmd in list_commands(version):
            ex = make_example(version, cmd, preamble)
            if ex is None:
                continue
            ex["metadata"]["kind"] = "command"
            if cmd in cmd_train:
                train_exs.append(ex)
            elif cmd in cmd_val and version == EVAL_VERSION:
                val_exs.append(ex)
            elif cmd in cmd_test and version == EVAL_VERSION:
                test_exs.append(ex)

    # Types: same rule
    for version in ALL_VERSIONS:
        for t in list_types(version):
            ex = make_type_example(version, t)
            if ex is None:
                continue
            if t in type_train:
                train_exs.append(ex)
            elif t in type_val and version == EVAL_VERSION:
                val_exs.append(ex)
            elif t in type_test and version == EVAL_VERSION:
                test_exs.append(ex)

    # Helpers: same rule
    for version in ALL_VERSIONS:
        for fn in list_helpers(version):
            ex = make_helper_example(version, fn)
            if ex is None:
                continue
            if fn in helper_train:
                train_exs.append(ex)
            elif fn in helper_val and version == EVAL_VERSION:
                val_exs.append(ex)
            elif fn in helper_test and version == EVAL_VERSION:
                test_exs.append(ex)

    # Write JSONL files
    for name, exs in [("train", train_exs), ("val", val_exs), ("test", test_exs)]:
        path = os.path.join(DATASET_DIR, f"{name}.jsonl")
        with open(path, "w") as fh:
            for ex in exs:
                fh.write(json.dumps(ex, ensure_ascii=False) + "\n")
        print(f"  {name}: {len(exs)} examples → {path}")

    total = len(train_exs) + len(val_exs) + len(test_exs)
    print(f"\nDataset summary: {len(train_exs)} train / {len(val_exs)} val / {len(test_exs)} test = {total} total")

    # Write helpers-only train split for staged training
    helpers_only = [ex for ex in train_exs if ex["metadata"].get("kind") == "helper_stub"]
    if helpers_only:
        helpers_path = os.path.join(DATASET_DIR, "train_helpers.jsonl")
        with open(helpers_path, "w") as fh:
            for ex in helpers_only:
                fh.write(json.dumps(ex, ensure_ascii=False) + "\n")
        print(f"  helpers: {len(helpers_only)} examples → {helpers_path}")

    print("\nDone.")


if __name__ == "__main__":
    main()
