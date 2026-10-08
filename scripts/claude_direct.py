#!/usr/bin/env python3.13
"""Claude as the spec writer, on exactly the prompt the 9B gets.

    python3.13 scripts/claude_direct.py --sections <dir with <v>/ subdirs> \
        --versions psci_13 psci_13_rmm --out <dir> [--jobs 8]

The baseline the 9B has to beat in the doc -> intermediate form -> code pipeline.
The system prompt and user message come from eval_checkpoint.build_prompt, the
same function gen_specs.py uses (v3.1, no preamble), so the only difference from
the 9B runs is who answers. Output goes to <out>/<v>/<command lower>.rs, the
layout the 9B runs write, after the same strip_output.
"""
import argparse
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "prompt_engineering"))
from confab_probe import call_claude  # noqa: E402
from eval_checkpoint import build_prompt, strip_output  # noqa: E402
from prompt_engineering_v3 import get_v3_prompt  # noqa: E402


class Sample:
    def __init__(self, command, section_text):
        self.command, self.section_text = command, section_text


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--sections", required=True)
    ap.add_argument("--versions", nargs="+", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--model", default="claude-opus-5-5")
    ap.add_argument("--effort", default="high")
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--system-file", default=None, help="as in gen_specs.py")
    ap.add_argument("--gold-signature", default=None,
                    help="specs dir holding gold <cmd>_spec.rs; puts gold's parameter "
                         "list in the Signature line, as gen_specs.py --gold-signature")
    ap.add_argument("--preamble", default=None,
                    help="preamble file shown before the section, as gen_specs.py "
                         "--with-preamble --preamble-mode full does")
    args = ap.parse_args()

    v3 = get_v3_prompt("v3.1")
    if args.system_file:
        v3 = type(v3)(v3.name, Path(args.system_file).read_text(), v3.user_template)
    pre = Path(args.preamble).read_text().strip() if args.preamble else None
    jobs = []
    for v in args.versions:
        for f in sorted((Path(args.sections) / v).glob("*_command.txt")):
            if not f.name.startswith("._"):
                jobs.append((v, Sample(f.name[: -len("_command.txt")],
                                       f.read_text(errors="replace").strip())))

    def one(job):
        v, s = job
        dst = Path(args.out) / v / f"{s.command.lower()}.rs"
        if dst.exists():
            return v, s.command, "cached"
        msgs = build_prompt(s, v3, pre)
        if args.gold_signature:
            from verify_generated_verus import extract_fn_block
            g = Path(args.gold_signature) / f"{s.command.lower()}_spec.rs"
            _, params, _ = extract_fn_block(g.read_text(errors="replace"))
            stub = f"{s.command.lower()}_spec(...)"
            assert stub in msgs[1]["content"] and params
            msgs[1] = dict(msgs[1], content=msgs[1]["content"].replace(stub, f"{s.command.lower()}_spec{params}", 1))
        sys_msg, user_msg = (m["content"] for m in msgs)
        for _ in range(2):
            try:
                text = strip_output(call_claude(sys_msg, user_msg, args.model, args.effort))
                break
            except Exception as e:  # noqa: BLE001
                text, err = None, e
        if text is None:
            return v, s.command, f"failed: {err}"
        dst.parent.mkdir(parents=True, exist_ok=True)
        dst.write_text(text + "\n")
        return v, s.command, "ok"

    with ThreadPoolExecutor(args.jobs) as ex:
        res = list(ex.map(one, jobs))
    bad = [r for r in res if r[2].startswith("failed")]
    for r in bad:
        print("[claude-direct] FAILED", *r, flush=True)
    print(f"[claude-direct] {len(res) - len(bad)}/{len(res)} written -> {args.out}", flush=True)


if __name__ == "__main__":
    main()
