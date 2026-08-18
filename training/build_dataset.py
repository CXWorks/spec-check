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

Split strategy: leak-free, name-based.
  - Commands: CMD_TEST_SIZE names drawn from EVAL_VERSION are held out for
    evaluation and removed from training at EVERY version. All remaining names
    contribute their instances from all six versions.
  - Types/helpers: 90/10 name split into train/val. These are not evaluated, so
    val exists only to give training an eval_loss signal without spending
    scarce command names on it.
Do NOT evaluate on all of EVAL_VERSION — only on splits.json["command_test"].
See docs/data-leakage.md for why version is not a valid split dimension here.

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
_SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))


def _find_data_root() -> str:
    """Locate the directory holding sections/ and specs/.

    Server layout: spec-gen/build_dataset.py alongside spec-gen/sections/.
    Repo layout:   training/build_dataset.py, data under training-dataset/.
    """
    for cand in (_SCRIPT_DIR, os.path.join(_SCRIPT_DIR, "..", "training-dataset")):
        if os.path.isdir(os.path.join(cand, "sections")):
            return os.path.abspath(cand)
    raise SystemExit(
        "Cannot find sections/ — expected under this script's directory or ../training-dataset/"
    )


BASE_DIR    = _find_data_root()
# Default output directory. `--out-dir` overrides it, so a variant build never
# overwrites the shipped dataset_clean/ that existing checkpoints were trained on.
DATASET_DIR = os.path.join(BASE_DIR, "dataset_clean")

for _candidate in (os.path.join(_SCRIPT_DIR, "prompt_engineering"),        # server layout
                   os.path.join(_SCRIPT_DIR, "..", "prompt_engineering")):  # repo layout
    if os.path.isdir(_candidate):
        sys.path.insert(0, _candidate)
        break
from prompt_engineering_v3 import V3_PROMPT, get_v3_prompt  # command-kind prompt: no preamble, deduped rules

ALL_VERSIONS = ["eac5", "rel0", "alp11", "alp12", "alp13", "alp14"]
EVAL_VERSION = "alp14"   # test commands are drawn from (and scored on) this version
SPLIT_SEED   = 42

# --- Leak-free split configuration -----------------------------------------
# The six spec versions are successive drafts of ONE document, not six
# independent corpora: alp13 and alp14 gold specs are byte-identical for 67 of
# 93 shared commands (mean similarity 0.957). Splitting by *version* therefore
# does not produce held-out data. The only sound split dimension is the item
# NAME: a name assigned to test is removed from training at every version.
# See docs/data-leakage.md.
CMD_TEST_SIZE = 40    # commands held out for evaluation, drawn from EVAL_VERSION
TH_VAL_FRAC   = 0.10  # fraction of type/helper NAMES used for training-time monitoring

# Commands are split train/test only. Validation is drawn from types+helpers,
# which the benchmark does not evaluate — this buys an eval_loss signal without
# spending scarce command names on it. Command names number only 121 across all
# versions, so every name held out costs ~3.9 training examples.

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

def split_two(names: list[str], n_held: int, seed=SPLIT_SEED):
    """Hold out `n_held` names; return (kept, held). Deterministic given the seed."""
    names = sorted(names)
    rng = random.Random(seed)
    rng.shuffle(names)
    held = set(names[:n_held])
    return set(names) - held, held


def assert_no_leakage(splits: dict[str, list[dict]]) -> None:
    """Fail loudly if any item name appears in more than one split.

    This is the guard that the previous version of this script lacked: its train
    branch had no version filter, so a train-split command name contributed all
    six of its per-version instances to training, including the version the
    evaluator scored on. See docs/data-leakage.md.
    """
    def name_of(ex):
        m = ex["metadata"]
        return (m.get("kind"), m.get("command") or m.get("type") or m.get("function"))

    seen: dict[tuple, str] = {}
    bad: list[str] = []
    for split, exs in splits.items():
        for key in {name_of(e) for e in exs}:
            if key in seen and seen[key] != split:
                bad.append(f"{key[0]} {key[1]!r}: in both {seen[key]!r} and {split!r}")
            seen[key] = seen.get(key, split)
    if bad:
        raise SystemExit(
            "LEAKAGE: item names appear in multiple splits:\n  " + "\n  ".join(bad[:20])
        )


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

def load_hold_out_file(path: str) -> list[str]:
    """Command names from a JSON list, or {"commands": [...]}, or one per line."""
    raw = open(path).read()
    try:
        doc = json.loads(raw)
        names = doc if isinstance(doc, list) else doc["commands"]
    except (json.JSONDecodeError, KeyError, TypeError):
        names = [l.split("#")[0].strip() for l in raw.splitlines()]
    return [n.upper() for n in names if n and not n.startswith("#")]


def main():
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument("--cascaded-context", metavar="DIR",
                        help="Directory with model-generated type files "
                             "({version}_types.rs) to use instead of golden preamble")
    parser.add_argument("--out-dir", metavar="DIR", default=DATASET_DIR,
                        help="Where to write train/val/test.jsonl + splits.json "
                             f"(default: {os.path.relpath(DATASET_DIR, BASE_DIR)}/). "
                             "Point a variant build somewhere else so the shipped "
                             "dataset stays intact and revertible.")
    parser.add_argument("--hold-out-commands", nargs="+", metavar="CMD", default=[],
                        help="Extra command names to force into the held-out set, "
                             "on top of the random CMD_TEST_SIZE draw. Removed from "
                             "training at EVERY version. Use for commands a benchmark "
                             "scores — otherwise the model was trained on the answer.")
    parser.add_argument("--hold-out-file", metavar="PATH", default=None,
                        help="File of command names to force into the held-out set "
                             "(JSON list, {'commands': [...]}, or one per line).")
    parser.add_argument("--prompt-variant", default=None, choices=["v3", "v3.1"],
                        help="Command-kind system prompt. Default v3 — the frozen "
                             "prompt every existing checkpoint was trained on. v3.1 "
                             "corrects the unconstrained-specs bullet (RSI_FEATURES "
                             "and version queries are NOT `true`); using it requires "
                             "a retrain, since the prompt is baked into the weights.")
    args = parser.parse_args()
    cascaded_dir = args.cascaded_context

    out_dir = os.path.abspath(args.out_dir)

    # Selecting a non-default prompt changes what the model is trained on, so it
    # may not silently overwrite the shipped dataset.
    prompt = get_v3_prompt(args.prompt_variant)
    global SYSTEM_PROMPT
    SYSTEM_PROMPT = prompt.system
    if prompt is not V3_PROMPT and out_dir == os.path.abspath(DATASET_DIR):
        raise SystemExit(
            f"--prompt-variant {args.prompt_variant} would overwrite the shipped "
            f"{os.path.relpath(DATASET_DIR, BASE_DIR)}/ with a dataset built on a "
            "different system prompt. Pass --out-dir to write it elsewhere."
        )

    forced = {n.upper() for n in args.hold_out_commands}
    if args.hold_out_file:
        forced |= set(load_hold_out_file(args.hold_out_file))
    if forced and out_dir == os.path.abspath(DATASET_DIR):
        raise SystemExit(
            f"--hold-out-commands would overwrite the shipped "
            f"{os.path.relpath(DATASET_DIR, BASE_DIR)}/, whose split existing eval "
            "results are keyed to. Pass --out-dir to write it elsewhere."
        )

    os.makedirs(out_dir, exist_ok=True)
    print(f"Prompt variant: {prompt.name}")
    print(f"Output dir:     {out_dir}")

    # --- Commands: hold out CMD_TEST_SIZE names drawn from EVAL_VERSION ---------
    # Held-out names are removed from training at EVERY version, not just EVAL_VERSION.
    eval_cmds = [c for c in list_commands(EVAL_VERSION) if load_spec(EVAL_VERSION, c)]
    _, cmd_test = split_two(eval_cmds, CMD_TEST_SIZE)

    # Forced hold-outs are ADDED to the random draw, never substituted into it, so
    # the original CMD_TEST_SIZE names stay exactly as they were and every eval
    # already run on them remains comparable to an eval run on the superset.
    all_cmds = set(all_item_names("command"))
    if forced:
        unknown = sorted(forced - all_cmds)
        if unknown:
            raise SystemExit(
                "--hold-out-commands names no command in any version: "
                + ", ".join(unknown)
            )
        # A forced name absent from EVAL_VERSION still leaves training, but it
        # cannot produce a test example — say so rather than let the count confuse.
        no_eval = sorted(c for c in forced if not load_spec(EVAL_VERSION, c))
        added   = sorted(forced - cmd_test)
        already = sorted(forced & cmd_test)
        cmd_test = cmd_test | forced
        print(f"Forced hold-out: {len(forced)} requested, {len(added)} added, "
              f"{len(already)} already held out")
        if added:
            print("  added:   " + ", ".join(added))
        if already:
            print("  already: " + ", ".join(already))
        if no_eval:
            print(f"  WARNING: absent from {EVAL_VERSION}, so removed from training "
                  f"but NOT evaluable: " + ", ".join(no_eval))

    cmd_train = all_cmds - cmd_test

    # --- Types / helpers: name-based 90/10 train/val, used only for monitoring ---
    type_names   = all_item_names("type")
    helper_names = all_item_names("helper")
    type_train,   type_val   = split_two(type_names,   int(len(type_names)   * TH_VAL_FRAC))
    helper_train, helper_val = split_two(helper_names, int(len(helper_names) * TH_VAL_FRAC))

    print(f"Command names — train:{len(cmd_train)} test:{len(cmd_test)} "
          f"(of {len(all_item_names('command'))} across all versions; "
          f"{len(eval_cmds)} exist in {EVAL_VERSION})")
    print(f"Type names    — train:{len(type_train)} val:{len(type_val)}")
    print(f"Helper names  — train:{len(helper_train)} val:{len(helper_val)}")

    preamble_cache: dict[str, str] = {}

    def get_preamble(version: str) -> str:
        if version not in preamble_cache:
            preamble_cache[version] = load_preamble(version, cascaded_dir)
        return preamble_cache[version]

    train_exs: list[dict] = []
    val_exs:   list[dict] = []
    test_exs:  list[dict] = []

    # Commands: train names contribute every version; test names contribute
    # EVAL_VERSION only and are absent from training entirely.
    for version in ALL_VERSIONS:
        preamble = get_preamble(version)
        for cmd in list_commands(version):
            ex = make_example(version, cmd, preamble)
            if ex is None:
                continue
            ex["metadata"]["kind"] = "command"
            if cmd in cmd_test:
                if version == EVAL_VERSION:
                    test_exs.append(ex)
                # other versions of a test command are dropped, never trained on
            elif cmd in cmd_train:
                train_exs.append(ex)

    # Types: all versions on both sides — they are not evaluated, so val here is
    # purely a training-time eval_loss signal.
    for version in ALL_VERSIONS:
        for t in list_types(version):
            ex = make_type_example(version, t)
            if ex is None:
                continue
            if t in type_val:
                val_exs.append(ex)
            elif t in type_train:
                train_exs.append(ex)

    # Helpers: same rule as types
    for version in ALL_VERSIONS:
        for fn in list_helpers(version):
            ex = make_helper_example(version, fn)
            if ex is None:
                continue
            if fn in helper_val:
                val_exs.append(ex)
            elif fn in helper_train:
                train_exs.append(ex)

    assert_no_leakage({"train": train_exs, "val": val_exs, "test": test_exs})

    # Write JSONL files
    for name, exs in [("train", train_exs), ("val", val_exs), ("test", test_exs)]:
        path = os.path.join(out_dir, f"{name}.jsonl")
        with open(path, "w") as fh:
            for ex in exs:
                fh.write(json.dumps(ex, ensure_ascii=False) + "\n")
        print(f"  {name}: {len(exs)} examples → {path}")

    # The evaluator needs the held-out command list to score the right subset.
    splits_path = os.path.join(out_dir, "splits.json")
    with open(splits_path, "w") as fh:
        json.dump({
            "seed": SPLIT_SEED,
            "eval_version": EVAL_VERSION,
            "cmd_test_size": CMD_TEST_SIZE,
            "th_val_frac": TH_VAL_FRAC,
            # Recorded so an artifact says which prompt and which forced hold-outs
            # produced it. A dataset that does not carry this is unidentifiable
            # once two variants exist side by side.
            "prompt_variant": prompt.name,
            "forced_hold_out": sorted(forced),
            "command_test":  sorted(cmd_test),
            "command_train": sorted(cmd_train),
            "type_val":      sorted(type_val),
            "helper_val":    sorted(helper_val),
        }, fh, indent=2)
    print(f"  splits: → {splits_path}")

    total = len(train_exs) + len(val_exs) + len(test_exs)
    print(f"\nDataset summary: {len(train_exs)} train / {len(val_exs)} val / {len(test_exs)} test = {total} total")

    by_kind = {}
    for ex in train_exs:
        k = ex["metadata"].get("kind", "?")
        by_kind[k] = by_kind.get(k, 0) + 1
    print(f"  train by kind: {by_kind}")
    print(f"\nEvaluate on the {len(cmd_test)} commands in splits.json['command_test'] "
          f"({EVAL_VERSION}) — NOT on all of {EVAL_VERSION}. See docs/data-leakage.md.")

    print("\nDone.")


if __name__ == "__main__":
    main()
