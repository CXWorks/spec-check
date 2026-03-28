#!/usr/bin/env python3
"""
pipeline.py

End-to-end pipeline: takes a new spec PDF (as pre-extracted text) and outputs
a complete .rs file using trained models for each layer.

Usage:
    pipeline.py --txt ccaspec/alp15.txt --target alp15 \\
                --l1-rs boilerplate/layer1.rs \\
                --l2-model models/layer2/ \\
                --l3-model models/layer3/ \\
                --cmd-model models/commands/ \\
                --out alp15_generated.rs

For oracle/testing mode (use golden files instead of models):
    pipeline.py --txt ccaspec/eac5.txt --target eac5 --oracle --out eac5_assembled.rs

Note: Model loading (--l2-model, --l3-model, --cmd-model) requires GPU inference.
      Implement load_model() and run_model() below to connect your inference stack.
"""

import argparse
import os
import sys
import re

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

# ---------------------------------------------------------------------------
# Model interface (stub — implement for GPU inference)
# ---------------------------------------------------------------------------

def load_model(model_path: str):
    """Load a fine-tuned model. Returns a model handle."""
    raise NotImplementedError(
        f"Implement load_model() to load fine-tuned model from {model_path}"
    )


def run_model(model, system_prompt: str, user_content: str) -> str:
    """Run inference on a loaded model. Returns the assistant response."""
    raise NotImplementedError(
        "Implement run_model() to call your inference backend"
    )


# ---------------------------------------------------------------------------
# Oracle mode: use golden files instead of models
# ---------------------------------------------------------------------------

def oracle_l2(version: str) -> dict[str, str]:
    """Return {type_name: verus_text} from specs/{version}/types/."""
    types_dir = os.path.join(BASE_DIR, "specs", version, "types")
    result = {}
    if os.path.exists(types_dir):
        for fname in sorted(os.listdir(types_dir)):
            if fname.endswith(".rs"):
                type_name = fname[:-3]
                with open(os.path.join(types_dir, fname)) as fh:
                    result[type_name] = fh.read().strip()
    return result


def oracle_l3(version: str) -> dict[str, str]:
    """Return {fn_name: stub_text} from specs/{version}/helpers/."""
    helpers_dir = os.path.join(BASE_DIR, "specs", version, "helpers")
    result = {}
    if os.path.exists(helpers_dir):
        for fname in sorted(os.listdir(helpers_dir)):
            if fname.endswith(".rs"):
                fn_name = fname[:-3]
                with open(os.path.join(helpers_dir, fname)) as fh:
                    result[fn_name] = fh.read().strip()
    return result


def oracle_commands(version: str) -> dict[str, str]:
    """Return {cmd_name: spec_text} from specs/{version}/*_spec.rs."""
    specs_dir = os.path.join(BASE_DIR, "specs", version)
    result = {}
    if os.path.exists(specs_dir):
        for fname in sorted(os.listdir(specs_dir)):
            if fname.endswith("_spec.rs"):
                cmd_name = fname[:-8].upper()  # rmi_data_create_spec → RMI_DATA_CREATE
                with open(os.path.join(specs_dir, fname)) as fh:
                    result[cmd_name] = fh.read().strip()
    return result


# ---------------------------------------------------------------------------
# Section extraction (reuses extract_sections.py logic)
# ---------------------------------------------------------------------------

def load_extractor():
    """Import extract_sections from same directory."""
    import importlib.util
    spec = importlib.util.spec_from_file_location(
        "extract_sections",
        os.path.join(BASE_DIR, "extract_sections.py")
    )
    mod = importlib.util.load_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


# ---------------------------------------------------------------------------
# Assembly
# ---------------------------------------------------------------------------

VERUS_OPEN  = "use vstd::prelude::*;\n\nverus! {\n\n"
VERUS_CLOSE = "\n} // verus!\n"


def assemble(
    layer1_text: str,
    type_defs: dict[str, str],
    helper_stubs: dict[str, str],
    cmd_specs: dict[str, str],
    preamble_tail: str | None = None,
) -> str:
    """
    Concatenate all parts into a single .rs file.
    layer1_text already contains the verus! { opening.
    """
    parts = [layer1_text.rstrip()]

    # Layer 2: enum/struct definitions
    if type_defs:
        parts.append("\n// --- Layer 2: Type definitions ---")
        for type_name in sorted(type_defs):
            parts.append("\n" + type_defs[type_name])

    # Layer 3: helper function stubs
    if helper_stubs:
        parts.append("\n// --- Layer 3: Helper function stubs ---")
        for fn_name in sorted(helper_stubs):
            parts.append("\n" + helper_stubs[fn_name])

    # Any extra preamble lines (e.g., from golden preamble tail for oracle mode)
    if preamble_tail:
        parts.append("\n" + preamble_tail)

    # Commands
    if cmd_specs:
        parts.append("\n// --- Commands ---")
        for cmd_name in sorted(cmd_specs):
            parts.append("\n" + cmd_specs[cmd_name])

    parts.append("\n} // verus!\n")
    return "\n".join(parts)


# ---------------------------------------------------------------------------
# Main pipeline
# ---------------------------------------------------------------------------

def run_pipeline(args):
    # --- Layer 1: hard-copy boilerplate ---
    if not os.path.exists(args.l1_rs):
        print(f"[ERROR] Layer 1 file not found: {args.l1_rs}", file=sys.stderr)
        sys.exit(1)
    with open(args.l1_rs) as fh:
        layer1_text = fh.read()
    print(f"[L1] Loaded boilerplate from {args.l1_rs}")

    if args.oracle:
        # Oracle mode: use golden files
        print(f"[ORACLE] Using golden files for version: {args.target}")
        type_defs     = oracle_l2(args.target)
        helper_stubs  = oracle_l3(args.target)
        cmd_specs     = oracle_commands(args.target)
        print(f"  L2: {len(type_defs)} types, L3: {len(helper_stubs)} helpers, "
              f"cmds: {len(cmd_specs)} commands")
    else:
        # Real model mode
        if not args.txt:
            print("[ERROR] --txt required when not in oracle mode", file=sys.stderr)
            sys.exit(1)

        extractor = load_extractor()
        cleaned = extractor.preprocess(args.txt)

        # Layer 2: generate type definitions
        l2_model = load_model(args.l2_model)
        type_sections = extractor.extract_types(cleaned)
        type_defs = {}
        print(f"[L2] Generating {len(type_sections)} type definitions...")
        for type_name, section_text in type_sections.items():
            resp = run_model(l2_model, _SYSTEM_TYPES, f"## Type Specification\n\n{section_text}")
            type_defs[type_name] = resp
            print(f"  {type_name}")

        # Layer 3: generate helper stubs
        l3_model = load_model(args.l3_model)
        helper_sections = extractor.extract_helper_fns(cleaned)
        helper_stubs = {}
        print(f"[L3] Generating {len(helper_sections)} helper stubs...")
        for fn_name, section_text in helper_sections.items():
            resp = run_model(l3_model, _SYSTEM_HELPERS,
                             f"## Helper Function Specification\n\n{section_text}")
            helper_stubs[fn_name] = resp
            print(f"  {fn_name}")

        # Commands: build preamble context from L1+L2+L3
        cmd_model = load_model(args.cmd_model)
        cmd_sections = extractor.extract_commands(cleaned)
        context = layer1_text + "\n".join(type_defs.values()) + "\n".join(helper_stubs.values())
        cmd_specs = {}
        print(f"[CMD] Generating {len(cmd_sections)} command specs...")
        for cmd_title, section_text in cmd_sections.items():
            user_content = (
                "## Context (shared Verus types and helper function signatures)\n\n"
                f"```rust\n{context[-3000:]}\n```\n\n"
                "## Command Specification (from RMM spec PDF)\n\n"
                f"{section_text}"
            )
            resp = run_model(cmd_model, _SYSTEM_COMMANDS, user_content)
            cmd_name = cmd_title.replace(" command", "").replace(" ", "_")
            cmd_specs[cmd_name] = resp
            print(f"  {cmd_name}")

    # Assemble
    assembled = assemble(layer1_text, type_defs, helper_stubs, cmd_specs)
    out_path = args.out or f"{args.target}_generated.rs"
    with open(out_path, "w") as fh:
        fh.write(assembled)
    print(f"\n[DONE] Written to {out_path} ({len(assembled)} chars)")

    # Optional verification
    if args.verify:
        import subprocess
        result = subprocess.run(["verus", out_path], capture_output=True, text=True)
        if result.returncode == 0:
            print("[VERIFY] Verus verification passed!")
        else:
            print("[VERIFY] Verus verification failed:")
            print(result.stderr[:2000])


_SYSTEM_TYPES = (
    "You are a formal specification assistant for Arm CCA RMM. "
    "Given a type specification in ASL pseudocode, generate the Verus/Rust type definition."
)
_SYSTEM_HELPERS = (
    "You are a formal specification assistant for Arm CCA RMM. "
    "Given a helper function specification in ASL pseudocode, "
    "generate the Verus uninterpreted spec function stub (single line ending with ';')."
)
_SYSTEM_COMMANDS = (
    "You are a formal specification assistant for Arm CCA RMM. "
    "Given the specification text for an RMM command and the shared Verus type/function "
    "context, generate the Verus specification function for that command."
)


def main():
    parser = argparse.ArgumentParser(description="RMM spec → Verus pipeline")
    parser.add_argument("--txt",       help="Pre-extracted spec text file (ccaspec/*.txt)")
    parser.add_argument("--target",    required=True, help="Version name (e.g. alp15)")
    parser.add_argument("--l1-rs",     default=os.path.join(BASE_DIR, "boilerplate", "layer1.rs"),
                        help="Layer 1 boilerplate .rs file")
    parser.add_argument("--l2-model",  default="models/layer2/",
                        help="Path to fine-tuned L2 (types) model")
    parser.add_argument("--l3-model",  default="models/layer3/",
                        help="Path to fine-tuned L3 (helpers) model")
    parser.add_argument("--cmd-model", default="models/commands/",
                        help="Path to fine-tuned command model")
    parser.add_argument("--out",       help="Output .rs file path")
    parser.add_argument("--oracle",    action="store_true",
                        help="Oracle mode: use golden files instead of models")
    parser.add_argument("--verify",    action="store_true",
                        help="Run verus on output file after generation")
    args = parser.parse_args()
    run_pipeline(args)


if __name__ == "__main__":
    main()
