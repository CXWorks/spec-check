#!/usr/bin/env python3
"""Does telling a general model not to invent definitions make it find the gap?

    python3 scripts/confab_probe.py --commands RMI_RTT_READ_ENTRY --versions eac5 rel0
    python3 scripts/confab_probe.py --all --versions eac5 rel0 --out-root results/confab

The claim under test, from BASELINE1_GENERAL_MODEL_COMPARISON.md: a capable model
hides a spec gap by confabulating a plausible definition for an output the
document never defines. Both Claude Opus 5 and GPT invented a `walk_level` for
RMI_RTT_READ_ENTRY, on both spec versions, in four independent generations, and
so failed to flag the one command whose gap is exactly that absence.

That is currently read as a property of capable models. It has a cheaper
explanation that nobody has ruled out: the prompt never told them not to. Our
fine-tuned models leave the same output blank, and the 9B does so while being
*more* capable than the 4B -- which is evidence against a capability story and
for a "learned the convention from ~250 examples" story. A general model given
the convention explicitly should then behave the same way.

Two arms, identical in every respect except one added paragraph:

  base          PROMPT_V3_SYSTEM verbatim, the prompt the published rows used
  noinvent      the same, plus NO_INVENT below
  sig           base, but the template carries gold's real parameter list
  noinvent+sig  both

Both arms are run here rather than quoting the published `base` numbers, because
those were produced on another date and a drifted model version would confound
the comparison. The control has to reproduce before the treatment means anything.

NO_INVENT deliberately names no command and no field. Mentioning walk_level, or
even "level", would hand over the answer and test nothing.
"""

import argparse
import json
import os
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "prompt_engineering"))

# Reproduced verbatim from run_baseline1_general.py on the benchmark branch: the
# general models get the preamble because they have never seen this DSL. Keeping
# the template identical is what makes the base arm a control.
TEMPLATE = """Context -- Verus type, constant and helper signatures available to you (copy names from here verbatim):
{context}

Command specification text:
{spec}

Signature: pub open spec fn {cmd_lower}_spec{signature} -> bool
Prefer Bits64/UInt64/UInt32 aliases when present in spec, but do not sacrifice semantic correctness for alias formatting.
Keep unchanged-state constraints when implied by the command behavior."""

NO_INVENT = """

CRITICAL -- do not define outputs the specification leaves undefined: If the specification text does not state what the value of a declared output is, leave that output unconstrained in your specification. Do not infer it, do not derive it from context, and do not carry it over from how similar commands behave. An output that the document never defines must remain unconstrained. Writing a plausible definition for it is worse than omitting it, because it turns a gap in the document into a constraint the document never made."""

# The `sig` arms replace `(...)` in the template above with gold's real parameter
# list. 85% of Claude's eac5 output has a signature gold cannot be compared
# against -- mostly ordering, e.g. result first instead of third. That convention
# carries no semantic content and is not derivable from the PDF, which says
# nothing about Verus signatures, so no instruction can recover it; the only way
# to supply it is to supply it. Legitimate for the question "can a general model
# replace the fine-tune in this pipeline", since gold signatures exist for every
# command the pipeline runs on.

PREAMBLE_TAIL_LINES = 200


def preamble_tail(version):
    p = ROOT / "training-dataset" / "specs" / version / "preamble.rs"
    lines = p.read_text(encoding="utf-8", errors="replace").splitlines(keepends=True)
    return "".join(lines[-PREAMBLE_TAIL_LINES:]).strip()


def strip_output(text):
    """Pull the spec fn out of whatever wrapping the CLI returns."""
    import re
    m = re.search(r"```(?:rust|verus)?\s*(.*?)```", text, re.DOTALL)
    body = m.group(1) if m else text
    i = body.find("pub open spec fn")
    return (body[i:] if i >= 0 else body).strip()


def call_claude(system, user, model, effort, timeout=600):
    argv = ["claude", "-p", "--model", model, "--effort", effort,
            "--tools", "", "--output-format", "text",
            "--system-prompt", system]
    env = dict(os.environ)
    # An inherited API key would bill the API and land the run on a different
    # account than the published rows used.
    env.pop("ANTHROPIC_API_KEY", None)
    p = subprocess.run(argv, input=user, capture_output=True, text=True,
                       timeout=timeout, env=env)
    if p.returncode != 0:
        raise RuntimeError(f"claude exited {p.returncode}: {(p.stderr or '')[:300]}")
    return p.stdout or ""


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--commands", nargs="*", default=["RMI_RTT_READ_ENTRY"])
    ap.add_argument("--all", action="store_true", help="every command in the version")
    ap.add_argument("--versions", nargs="+", default=["eac5", "rel0"])
    ap.add_argument("--arms", nargs="+", default=["base", "noinvent"],
                    choices=["base", "noinvent", "sig", "noinvent+sig"])
    ap.add_argument("--model", default="claude-opus-5")
    ap.add_argument("--effort", default="high")
    ap.add_argument("--out-root", default="results/confab")
    args = ap.parse_args()

    from dataset_loader import load_dataset
    from prompt_engineering_v3 import PROMPT_V3_SYSTEM

    systems = {"base": PROMPT_V3_SYSTEM,
               "noinvent": PROMPT_V3_SYSTEM + NO_INVENT,
               "sig": PROMPT_V3_SYSTEM,
               "noinvent+sig": PROMPT_V3_SYSTEM + NO_INVENT}
    # Arms whose name carries "sig" get gold's real parameter list instead of `(...)`.
    from verify_generated_verus import extract_fn_block

    def signature_for(version, cmd):
        p = ROOT / "training-dataset" / "specs" / version / f"{cmd.lower()}_spec.rs"
        if not p.exists():
            return "(...)"
        _, params, _ = extract_fn_block(p.read_text(encoding="utf-8", errors="replace"))
        return params or "(...)"
    out_root = Path(args.out_root)
    log = []

    for version in args.versions:
        ctx = preamble_tail(version)
        samples = {s.command: s for s in load_dataset(versions=[version], all_commands=True)
                   if getattr(s, "command", None)}
        wanted = sorted(samples) if args.all else [c for c in args.commands if c in samples]
        missing = [] if args.all else [c for c in args.commands if c not in samples]
        for c in missing:
            print(f"[confab] {version}: {c} not in this version, skipping", flush=True)

        for arm in args.arms:
            d = out_root / arm / version
            d.mkdir(parents=True, exist_ok=True)
            for i, cmd in enumerate(wanted, 1):
                dest = d / f"{cmd.lower()}.rs"
                if dest.exists() and dest.stat().st_size > 0:
                    print(f"[confab] {arm}/{version} {i}/{len(wanted)} {cmd}: cached", flush=True)
                    continue
                sig = signature_for(version, cmd) if "sig" in arm else "(...)"
                user = TEMPLATE.format(context=ctx, spec=samples[cmd].section_text,
                                       cmd_lower=cmd.lower(), signature=sig)
                t0 = time.time()
                try:
                    spec = strip_output(call_claude(systems[arm], user, args.model, args.effort))
                except Exception as e:
                    print(f"[confab] {arm}/{version} {cmd}: FAILED {e}", flush=True)
                    log.append({"arm": arm, "version": version, "command": cmd,
                                "error": str(e)[:200]})
                    continue
                dest.write_text(spec)
                print(f"[confab] {arm}/{version} {i}/{len(wanted)} {cmd}: "
                      f"{len(spec)} chars, {time.time()-t0:.0f}s", flush=True)
                log.append({"arm": arm, "version": version, "command": cmd,
                            "chars": len(spec)})

    out_root.mkdir(parents=True, exist_ok=True)
    (out_root / "log.json").write_text(json.dumps(log, indent=2))
    print(f"\n[confab] wrote {out_root}/", flush=True)


if __name__ == "__main__":
    main()
