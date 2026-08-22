#!/usr/bin/env python3
"""
Phase 1: Dataset Loader & Evaluation Metrics (Prompt Engineering Path)

Loads data directly from raw PDF sections + gold Verus specs.

Data sources:
  - Raw input:  training-dataset/sections/{version}/{CMD}_command.txt
  - Context:    training-dataset/specs/{version}/preamble.rs  (last N lines)
  - Gold output: training-dataset/specs/{version}/{cmd}_spec.rs

Version splits:
  Train: eac5, rel0, alp11, alp12
  Val:   alp13
  Test:  alp14
"""

import os
import sys
import re
from pathlib import Path
from typing import List, Optional

# ============================================================================
# Config
# ============================================================================

TRAIN_VERSIONS = ["eac5", "rel0", "alp11", "alp12"]
VAL_VERSIONS = ["alp13"]
TEST_VERSIONS = ["alp14"]
ALL_VERSIONS = TRAIN_VERSIONS + VAL_VERSIONS + TEST_VERSIONS

PREAMBLE_TAIL_LINES = 200


# ============================================================================
# Data Classes
# ============================================================================

class SpecOracle:
    """Single sample: raw PDF section + preamble context + gold Verus code."""

    def __init__(
        self,
        command: str,
        version: str,
        section_text: str,
        preamble: str,
        oracle: str,
    ):
        self.command = command          # e.g. "RMI_DATA_CREATE"
        self.version = version          # e.g. "alp14"
        self.section_text = section_text  # raw PDF text from _command.txt
        self.preamble = preamble        # preamble context (tail of preamble.rs)
        self.oracle = oracle            # gold Verus spec function

    def __repr__(self):
        sec_preview = self.section_text[:80].replace('\n', ' ') + "..."
        return f"SpecOracle(cmd={self.command}, v={self.version}, sec={sec_preview})"


# ============================================================================
# Raw File Loaders
# ============================================================================

def _find_data_root() -> Path:
    """Locate the training-dataset directory relative to this script."""
    return Path(__file__).resolve().parent.parent / "training-dataset"


def _decl_blocks(lines):
    """(symbol, block) for each top-level declaration in a preamble."""
    out, i = [], 0
    while i < len(lines):
        m = re.match(
            r"^pub (?:open )?spec (?:fn|const)\s+([A-Za-z_]\w*)"
            r"|^pub (?:enum|struct)\s+([A-Za-z_]\w*)"
            r"|^(?:enum|struct)\s+([A-Za-z_]\w*)", lines[i].strip())
        if not m:
            i += 1
            continue
        sym = m.group(1) or m.group(2) or m.group(3)
        depth = lines[i].count("{") - lines[i].count("}")
        blk = [lines[i]]
        i += 1
        while i < len(lines) and depth > 0:
            depth += lines[i].count("{") - lines[i].count("}")
            blk.append(lines[i])
            i += 1
        out.append((sym, blk))
    return out


def load_preamble(version: str, tail_lines: int = PREAMBLE_TAIL_LINES,
                  section_text: str = None) -> str:
    """Preamble context for a command.

    Default is the last `tail_lines` lines, which is what every checkpoint so far
    was trained and evaluated with. **That window is badly chosen** and it is the
    largest single defect found in this pipeline:

        eac5   preamble  683 lines, window = 484-683 -> hides 21% of the API gold uses
        alp14  preamble 1632 lines, window = 1433-1632 -> hides 51%

    On alp14 -- the version the 49-command eval uses -- half the functions gold
    calls are invisible to the model at inference time, including
    `AddrIsGranuleAligned` and `AddrIsProtected`, which appear in almost every
    failure condition.

    The clearest instance: gold calls `RttWalk_(s, rd, addr, level)`, declared at
    line 75 and outside the window. Inside the window sits `RttWalk(s, rd, addr)`,
    a different uninterpreted function. With no preamble the 9B writes
    `RttWalk(...)` with FOUR arguments 254 times and zero times with three -- the
    right arity for the function it learned in training, under the only name it
    can recall -- which is exactly `E0061: this function takes 3 arguments but 4
    were supplied`, the dominant repair failure. Shown the tail, it switches to
    the three-argument `RttWalk` 144 times: now it compiles, and now it means
    something else, so every success condition that walks the RTT disagrees with
    gold. That is the mechanism behind "the preamble raises compilation and not
    correctness".

    Passing `section_text` selects declarations by relevance instead: every symbol
    named in the command's own document section, one transitive step through the
    types those declarations mention, plus all constants and type definitions,
    which are small and carry the literals (`DELEGATED`, `RAM`) that conditions
    compare against. It is ~5x SMALLER than the tail and covers far more.

    Kept opt-in. Every published number was produced with the tail, and switching
    the default would silently make old and new runs incomparable.
    """
    path = _find_data_root() / "specs" / version / "preamble.rs"
    if not path.exists():
        print(f"Warning: preamble not found: {path}")
        return ""
    raw = path.read_text(encoding="utf-8", errors="replace")
    if section_text is None:
        lines = raw.splitlines(keepends=True)
        return "".join(lines[-tail_lines:]).strip()

    lines = raw.splitlines()
    blocks = _decl_blocks(lines)
    by = {sym: blk for sym, blk in blocks}
    # Constants and type definitions are always included: a condition comparing
    # against DELEGATED is unusable without the enum that declares it, and they
    # are a few hundred characters in total.
    core = {sym for sym, blk in blocks
            if re.match(r"^pub (?:enum|struct)|^(?:enum|struct)|^pub spec const",
                        blk[0].strip())}
    seed = (set(re.findall(r"[A-Za-z_]\w*", section_text)) & set(by)) | core
    sel = set(seed)
    for sym in list(seed):
        sel |= set(re.findall(r"[A-Za-z_]\w*", "\n".join(by[sym]))) & set(by)
    order = {sym: i for i, (sym, _) in enumerate(blocks)}
    return "\n".join("\n".join(by[s]) for s in sorted(sel, key=order.get)).strip()


def load_section(version: str, command: str) -> Optional[str]:
    """Load raw PDF section text for a command (e.g. 'RMI_DATA_CREATE').

    `errors="replace"` because this text comes out of a PDF extractor and is not
    guaranteed clean: a single undecodable byte anywhere in a corpus would
    otherwise abort a whole generation run partway through, losing the commands
    already done. A mangled character in one section is a far smaller problem.
    """
    path = _find_data_root() / "sections" / version / f"{command}_command.txt"
    if not path.exists():
        return None
    return path.read_text(encoding="utf-8", errors="replace").strip()


def load_gold_spec(version: str, command: str) -> Optional[str]:
    """Load gold Verus spec function for a command."""
    filename = command.lower() + "_spec.rs"
    path = _find_data_root() / "specs" / version / filename
    if not path.exists():
        return None
    text = path.read_text().strip()
    if "[EXCLUDED]" in text:
        return None
    return text


def list_commands(version: str) -> List[str]:
    """List all commands available for a version (from sections directory)."""
    sec_dir = _find_data_root() / "sections" / version
    if not sec_dir.exists():
        return []
    cmds = []
    for f in sorted(sec_dir.iterdir()):
        if f.name.endswith("_command.txt"):
            cmds.append(f.name[:-len("_command.txt")])
    return cmds


# ============================================================================
# Dataset Loading
# ============================================================================

def load_version(version: str) -> List[SpecOracle]:
    """Load all command samples for a single version from raw files."""
    preamble = load_preamble(version)
    commands = list_commands(version)
    samples = []

    for cmd in commands:
        section = load_section(version, cmd)
        if section is None:
            continue
        gold = load_gold_spec(version, cmd)
        if gold is None:
            continue
        samples.append(SpecOracle(
            command=cmd,
            version=version,
            section_text=section,
            preamble=preamble,
            oracle=gold,
        ))

    return samples


DATASET_DIR_ENV = "SPEC_CHECK_DATASET_DIR"


def dataset_dir_name() -> str:
    """Which built dataset to read the split from. Defaults to dataset_clean.

    A second dataset exists once benchmark commands are forced out of training
    (build_dataset.py --hold-out-file), and it has a LARGER held-out set. Scoring
    a checkpoint against the wrong one either misses the commands it was retrained
    to hold out, or scores it on commands it was trained on — so the directory is
    explicit rather than guessed.
    """
    return os.environ.get(DATASET_DIR_ENV, "dataset_clean")


def load_held_out_commands() -> set:
    """Command names held out for evaluation, per <dataset dir>/splits.json.

    This is the authoritative eval set. The version-based TEST_VERSIONS split
    above is NOT: 79 of alp14's 98 commands have their gold answer verbatim in
    the training set, because build_dataset.py splits by item name while this
    module splits by version. See docs/data-leakage.md.
    """
    import json
    name = dataset_dir_name()
    path = _find_data_root() / name / "splits.json"
    if not path.exists():
        raise FileNotFoundError(
            f"{path} not found — run `python3 training/build_dataset.py` first"
            + (f" with --out-dir {name}" if name != "dataset_clean" else "")
            + ".\nEvaluating without it silently scores on leaked commands."
        )
    doc = json.loads(path.read_text())
    held = set(doc["command_test"])
    print(f"[dataset] held-out split: {name}/splits.json — {len(held)} commands"
          + (f", prompt {doc['prompt_variant']}" if doc.get("prompt_variant") else ""))
    return held


def load_dataset(
    versions: Optional[List[str]] = None,
    split: Optional[str] = None,
    all_commands: bool = False,
) -> List[SpecOracle]:
    """
    Load dataset from raw section files + gold specs.

    Args:
        versions: Explicit list of versions to load, e.g. ["alp14"].
                  Mutually exclusive with `split`.
        split:    One of "train", "val", "test", "all".
                  Ignored if `versions` is provided.
        all_commands: For split="test" only. Default False restricts the result
                  to the held-out commands in dataset_clean/splits.json. Pass
                  True to get every alp14 command, which re-introduces the
                  train/test overlap documented in docs/data-leakage.md — only
                  valid for models that never saw the gold answers.

    Returns:
        List of SpecOracle samples.
    """
    if versions is None:
        split = split or "test"
        versions = {
            "train": TRAIN_VERSIONS,
            "val": VAL_VERSIONS,
            "test": TEST_VERSIONS,
            "all": ALL_VERSIONS,
        }.get(split)
        if versions is None:
            raise ValueError(f"Unknown split '{split}'. Use train/val/test/all.")

    dataset = []
    for v in versions:
        samples = load_version(v)
        dataset.extend(samples)
        print(f"  {v}: {len(samples)} commands loaded")

    # Default to the clean eval set. Fail closed: a caller that forgets to filter
    # gets correct behaviour, and a caller that wants the leaky set must say so.
    if split == "test" and not all_commands:
        held = load_held_out_commands()
        before = len(dataset)
        dataset = [s for s in dataset if s.command in held]
        print(f"  held-out filter: {before} -> {len(dataset)} commands "
              f"(dataset_clean/splits.json)")

    print(f"Total: {len(dataset)} samples from {versions}")
    return dataset


# ============================================================================
# Evaluation Metrics
# ============================================================================

class EvaluationMetrics:
    """Metrics for a single sample."""

    def __init__(self, command: str, version: str):
        self.command = command
        self.version = version
        self.generated = None
        self.oracle = None
        self.exact_match = False
        self.error_msg = None

    def evaluate(self, generated: str, oracle: str):
        self.generated = generated
        self.oracle = oracle
        self.exact_match = (generated.strip() == oracle.strip())

    def __repr__(self):
        status = "PASS" if self.exact_match else "FAIL"
        return f"[{status}] {self.command} ({self.version})"


# ============================================================================
# Main — quick sanity check
# ============================================================================

def main():
    split = sys.argv[1] if len(sys.argv) > 1 else "test"
    print(f"Loading split={split}...")

    dataset = load_dataset(split=split)
    if not dataset:
        print("No data loaded.")
        return

    print(f"\n--- Sample #{1} ---")
    s = dataset[0]
    print(f"  Command:      {s.command}")
    print(f"  Version:      {s.version}")
    print(f"  Section len:  {len(s.section_text)} chars")
    print(f"  Preamble len: {len(s.preamble)} chars")
    print(f"  Oracle len:   {len(s.oracle)} chars")
    print(f"  Section head: {s.section_text[:120].replace(chr(10), ' ')}...")
    print(f"  Oracle head:  {s.oracle[:120].replace(chr(10), ' ')}...")

    # Summary by version
    from collections import Counter
    by_v = Counter(s.version for s in dataset)
    print(f"\nBy version: {dict(by_v)}")


if __name__ == "__main__":
    main()
