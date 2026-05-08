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

import subprocess
import tempfile

from unsloth import FastLanguageModel
import torch

BASE_DIR = os.path.dirname(os.path.abspath(__file__))
VERUSFMT = os.path.join(BASE_DIR, "verusfmt/target/release/verusfmt")


def fmt_code(code: str) -> str:
    """Format a Verus snippet with verusfmt (--verus-only); return original on failure."""
    if not os.path.exists(VERUSFMT):
        return code
    with tempfile.NamedTemporaryFile(suffix=".rs", mode="w", delete=False) as f:
        f.write(f"use vstd::prelude::*;\nverus! {{\n{code}\n}}\n")
        fname = f.name
    try:
        r = subprocess.run([VERUSFMT, "--verus-only", fname],
                           capture_output=True, timeout=10)
        if r.returncode == 0:
            txt = open(fname).read()
            inner = re.search(
                r'verus!\s*\{(.*)\}\s*(?://[^\n]*)?\s*$', txt, re.DOTALL)
            if inner:
                return inner.group(1).strip()
    except Exception:
        pass
    finally:
        os.unlink(fname)
    return code

# ---------------------------------------------------------------------------
# Model interface (stub — implement for GPU inference)
# ---------------------------------------------------------------------------

def load_model(model_path: str):
    """Load a fine-tuned model. Returns a (model, tokenizer) handle."""
    model, tokenizer = FastLanguageModel.from_pretrained(
        model_path, max_seq_length=8192, load_in_4bit=True, dtype=None)
    FastLanguageModel.for_inference(model)
    return model, tokenizer


def run_model(model, system_prompt: str, user_content: str) -> str:
    """Run inference on a loaded model. Returns the assistant response."""
    m, tokenizer = model
    messages = [
        {"role": "system", "content": system_prompt},
        {"role": "user",   "content": user_content},
    ]
    raw = tokenizer.apply_chat_template(
        messages, return_tensors="pt", add_generation_prompt=True
    )
    # apply_chat_template may return a BatchEncoding or a plain tensor
    if hasattr(raw, "input_ids"):
        input_ids = raw.input_ids.to("cuda")
    else:
        input_ids = raw.to("cuda")
    input_len = input_ids.shape[1]
    with torch.no_grad():
        out = m.generate(input_ids, max_new_tokens=2048, temperature=0.1,
                         do_sample=True, pad_token_id=tokenizer.eos_token_id)
    return tokenizer.decode(out[0][input_len:], skip_special_tokens=True).strip()


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
        # Real model mode — use pre-extracted sections dir or raw txt
        if args.sections_dir:
            sdir = args.sections_dir
            type_sections = {
                f[:-4]: open(os.path.join(sdir, "types", f)).read()
                for f in sorted(os.listdir(os.path.join(sdir, "types")))
                if f.endswith(".txt")
            }
            helper_sections = {
                f[:-4]: open(os.path.join(sdir, "helpers", f)).read()
                for f in sorted(os.listdir(os.path.join(sdir, "helpers")))
                if f.endswith(".txt")
            }
            cmd_sections = {
                f[:-12]: open(os.path.join(sdir, f)).read()
                for f in sorted(os.listdir(sdir))
                if f.endswith("_command.txt")
            }
        else:
            if not args.txt:
                print("[ERROR] --txt or --sections-dir required when not in oracle mode",
                      file=sys.stderr)
                sys.exit(1)
            extractor = load_extractor()
            cleaned = extractor.preprocess(args.txt)
            type_sections    = extractor.extract_types(cleaned)
            helper_sections  = extractor.extract_helper_fns(cleaned)
            cmd_sections     = extractor.extract_commands(cleaned)

        # Select system prompts based on spec type
        spec_type = getattr(args, "spec_type", "rmm")
        _TYPES_MAP = {
            "psci": (_SYSTEM_TYPES_PSCI,    _SYSTEM_COMMANDS_PSCI,    "PSCI spec PDF"),
            "sdei": (_SYSTEM_TYPES_SDEI,    _SYSTEM_COMMANDS_SDEI,    "SDEI spec PDF"),
            "drtm": (_SYSTEM_TYPES_DRTM,    _SYSTEM_COMMANDS_DRTM,    "DRTM spec PDF"),
            "scmi": (_SYSTEM_TYPES_SCMI,    _SYSTEM_COMMANDS_SCMI,    "SCMI spec PDF"),
            "ffa":  (_SYSTEM_TYPES_FFA,     _SYSTEM_COMMANDS_FFA,     "FF-A spec PDF"),
            "sbi":  (_SYSTEM_TYPES_SBI,     _SYSTEM_COMMANDS_SBI,     "RISC-V SBI spec"),
            "tdx":  (_SYSTEM_TYPES_TDX,     _SYSTEM_COMMANDS_TDX,     "Intel TDX ABI spec"),
        }
        if spec_type in _TYPES_MAP:
            sys_types, sys_cmds, spec_label = _TYPES_MAP[spec_type]
        else:
            sys_types, sys_cmds, spec_label = _SYSTEM_TYPES, _SYSTEM_COMMANDS, "RMM spec PDF"
        sys_helpers = _SYSTEM_HELPERS

        # Layer 2: generate type definitions
        l2_model = load_model(args.l2_model)
        type_defs = {}
        print(f"[L2] Generating {len(type_sections)} type definitions...")
        for type_name, section_text in type_sections.items():
            resp = run_model(l2_model, sys_types,
                             f"## Type Specification (from {spec_label})\n\n{section_text}")
            type_defs[type_name] = fmt_code(resp)
            print(f"  {type_name}")

        # Layer 3: generate helper stubs (skipped if no helper sections)
        helper_stubs = {}
        if helper_sections:
            l3_model = load_model(args.l3_model)
            print(f"[L3] Generating {len(helper_sections)} helper stubs...")
            for fn_name, section_text in helper_sections.items():
                resp = run_model(l3_model, sys_helpers,
                                 f"## Helper Function Specification (from {spec_label})\n\n{section_text}")
                helper_stubs[fn_name] = fmt_code(resp)
                print(f"  {fn_name}")
        else:
            print(f"[L3] No helper sections found — skipping L3 inference")

        # Commands: build preamble context from L1+L2+L3, last 200 lines
        cmd_model = load_model(args.cmd_model)
        context_full = (layer1_text + "\n".join(type_defs.values()) +
                        "\n".join(helper_stubs.values()))
        context_tail = "\n".join(context_full.splitlines()[-200:])
        # Truncate section text to avoid exceeding model context window.
        # Qwen3-4B has 40960 token context; limit spec text to ~12000 chars
        # (~3000 tokens) to leave room for context_tail and generation.
        MAX_SECTION_CHARS = 12000

        cmd_specs = {}
        print(f"[CMD] Generating {len(cmd_sections)} command specs...")
        for cmd_title, section_text in cmd_sections.items():
            trunc_section = section_text[:MAX_SECTION_CHARS]
            if len(section_text) > MAX_SECTION_CHARS:
                trunc_section += "\n... [TRUNCATED FOR CONTEXT WINDOW]"
            user_content = (
                "## Context (shared Verus types and helper function signatures)\n\n"
                f"```rust\n{context_tail}\n```\n\n"
                f"## Command Specification (from {spec_label})\n\n"
                f"{trunc_section}"
            )
            resp = run_model(cmd_model, sys_cmds, user_content)
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
    "You are a formal specification assistant for Arm CCA (Confidential Compute "
    "Architecture) Realm Management Monitor (RMM). "
    "Given the specification text for an RMM type definition (enumeration, structure, "
    "or fieldset), generate the corresponding Verus/Rust type definition. "
    "Output only the type definition (pub enum or struct block) in valid Verus syntax."
)
_SYSTEM_HELPERS = (
    "You are a formal specification assistant for Arm CCA (Confidential Compute "
    "Architecture) Realm Management Monitor (RMM). "
    "Given the specification text for an RMM helper function in ASL pseudocode, "
    "generate the Verus uninterpreted spec function stub (a single line ending with ';'). "
    "Output only the stub declaration in valid Verus syntax."
)
_SYSTEM_COMMANDS = (
    "You are a formal specification assistant for Arm CCA (Confidential Compute "
    "Architecture) Realm Management Monitor (RMM). "
    "Given the specification text for an RMM command and the shared Verus type/function "
    "context (preamble), generate the Verus specification function for that command. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in "
    "valid Verus syntax."
)

# PSCI-specific system prompts
_SYSTEM_TYPES_PSCI = (
    "You are a formal specification assistant for the ARM Power State Coordination Interface "
    "(PSCI, DEN0022). "
    "Given the specification text for a PSCI data type, generate the corresponding Verus/Rust "
    "type definition. "
    "Output only the type definition (pub enum or struct block) in valid Verus syntax."
)
_SYSTEM_COMMANDS_PSCI = (
    "You are a formal specification assistant for the ARM Power State Coordination Interface "
    "(PSCI, DEN0022). "
    "Given the specification text for a PSCI function and the shared Verus type context, "
    "generate the Verus specification function for that PSCI command. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in "
    "valid Verus syntax, encoding the preconditions and postconditions as boolean implications."
)

# SDEI-specific system prompts
_SYSTEM_TYPES_SDEI = (
    "You are a formal specification assistant for the ARM Software Delegated Exception "
    "Interface (SDEI, DEN0054C). "
    "Given the specification text for an SDEI data type, generate the corresponding Verus/Rust "
    "type definition. Output only the type definition in valid Verus syntax."
)
_SYSTEM_COMMANDS_SDEI = (
    "You are a formal specification assistant for the ARM Software Delegated Exception "
    "Interface (SDEI, DEN0054C). "
    "Given the specification text for an SDEI function and the shared Verus type context, "
    "generate the Verus specification function for that SDEI command. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in "
    "valid Verus syntax, encoding preconditions and postconditions as boolean implications."
)

# DRTM-specific system prompts
_SYSTEM_TYPES_DRTM = (
    "You are a formal specification assistant for the ARM Dynamic Root of Trust for "
    "Measurement (DRTM, DEN0113). "
    "Given the specification text for a DRTM data type, generate the corresponding Verus/Rust "
    "type definition. Output only the type definition in valid Verus syntax."
)
_SYSTEM_COMMANDS_DRTM = (
    "You are a formal specification assistant for the ARM Dynamic Root of Trust for "
    "Measurement (DRTM, DEN0113). "
    "Given the specification text for a DRTM function and the shared Verus type context, "
    "generate the Verus specification function for that DRTM command. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in "
    "valid Verus syntax, encoding preconditions and postconditions as boolean implications."
)

# SCMI-specific system prompts
_SYSTEM_TYPES_SCMI = (
    "You are a formal specification assistant for the ARM System Control and Management "
    "Interface (SCMI, DEN0056F). "
    "Given the specification text for an SCMI data type, generate the corresponding Verus/Rust "
    "type definition. Output only the type definition in valid Verus syntax."
)
_SYSTEM_COMMANDS_SCMI = (
    "You are a formal specification assistant for the ARM System Control and Management "
    "Interface (SCMI, DEN0056F), Base Protocol and Power Domain Protocol. "
    "Given the specification text for an SCMI message and the shared Verus type context, "
    "generate the Verus specification function for that SCMI message. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in "
    "valid Verus syntax, encoding preconditions and postconditions as boolean implications."
)

# FF-A-specific system prompts
_SYSTEM_TYPES_FFA = (
    "You are a formal specification assistant for the ARM Firmware Framework for Arm A-profile "
    "(FF-A, DEN0077A). "
    "Given the specification text for an FF-A data type, generate the corresponding Verus/Rust "
    "type definition. Output only the type definition in valid Verus syntax."
)
_SYSTEM_COMMANDS_FFA = (
    "You are a formal specification assistant for the ARM Firmware Framework for Arm A-profile "
    "(FF-A, DEN0077A). "
    "Given the specification text for an FF-A function and the shared Verus type context, "
    "generate the Verus specification function for that FF-A function. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in "
    "valid Verus syntax, encoding preconditions and postconditions as boolean implications."
)


# RISC-V SBI-specific system prompts
_SYSTEM_TYPES_SBI = (
    "You are a formal specification assistant for the RISC-V Supervisor Binary Interface (SBI). "
    "Given the specification text for an SBI data structure, generate an uninterpreted Verus stub. "
    "Output only the stub declaration in valid Verus syntax."
)
_SYSTEM_COMMANDS_SBI = (
    "You are a formal specification assistant for the RISC-V Supervisor Binary Interface (SBI). "
    "Given the specification text for an SBI function and the shared Verus context, generate "
    "the Verus specification function. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in valid "
    "Verus syntax, encoding preconditions and postconditions as boolean implications."
)

# Intel TDX-specific system prompts
_SYSTEM_TYPES_TDX = (
    "You are a formal specification assistant for Intel TDX Module ABI. "
    "Given the specification text for a TDX data structure, generate the Verus type definition. "
    "Output only the type definition in valid Verus syntax."
)
_SYSTEM_COMMANDS_TDX = (
    "You are a formal specification assistant for Intel TDX Module ABI. "
    "Given the specification text for a TDCALL leaf function and the shared Verus context, generate "
    "the Verus specification function. "
    "The output should be a single `pub open spec fn {cmd}_spec(...)` function body in valid "
    "Verus syntax, encoding preconditions and postconditions as boolean implications."
)


def main():
    parser = argparse.ArgumentParser(description="RMM spec → Verus pipeline")
    parser.add_argument("--txt",         help="Pre-extracted spec text file (ccaspec/*.txt)")
    parser.add_argument("--sections-dir", help="Pre-extracted sections dir (e.g. sections/alp14)")
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
    parser.add_argument("--spec-type", default="rmm",
                        choices=["rmm", "psci", "sdei", "drtm", "scmi", "ffa", "sbi", "tdx"],
                        help="Spec domain — rmm (default), psci, sdei, drtm, scmi, ffa, sbi, tdx")
    parser.add_argument("--oracle",    action="store_true",
                        help="Oracle mode: use golden files instead of models")
    parser.add_argument("--verify",    action="store_true",
                        help="Run verus on output file after generation")
    args = parser.parse_args()
    run_pipeline(args)


if __name__ == "__main__":
    main()
