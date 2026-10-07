#!/usr/bin/env python3
"""Write the intermediate form (a strict RMM-layout condition table) for the gold testbed.

    # Claude, locally (needs the claude CLI)
    python3 scripts/ir_testbed.py claude --sections training-dataset/sections/alp14_prose \
        --out <root> --version alp14_ir_claude [--jobs 8]
    # a 9B, where the GPUs are
    python3 scripts/ir_testbed.py ninebee --model ft|base --sections ... --out <root> \
        --version alp14_ir_ft [--shard 0 --nshards 4]

The testbed is alp14's 49 held-out commands with their conditions rewritten as
PSCI-style prose, so the input reads like an unseen document while gold still
exists. The question is how much the final spec loses when the 9B, instead of
Claude, writes the intermediate form. So both writers get exactly the same
prompt, built by build_prompt() below:

  - the RMM-layout instructions and strict pre:/post: rules of restyle_mirror.py;
  - an example command from the TRAINING split (RMI_REALM_ACTIVATE). The example
    used before, RMI_GRANULE_DELEGATE, is one of the 49, and showing it would
    hand over one answer;
  - the full alp14 preamble, so the table can use the names gold uses;
  - the prose section.

Every table is checked (strict grammar, RMM headings, post: rows, return codes,
command name) but nothing is dropped: the comparison is on the final spec, so
each arm keeps all 49 commands. Claude gets one rewrite when the check fails; the
9B decodes greedily, so a rewrite would repeat itself and it gets one attempt.

Output: <out>/sections/<version>/<CMD>_command.txt and <out>/specs/<version>/
(preamble.rs plus gold copies), the layout gen_specs.py --versions reads, and
<out>/<version>.check[.<shard>].json with the check result for each command.
"""
import argparse
import json
import re
import shutil
import sys
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT / "prompt_engineering"))
from restyle_conditions import CODE, SYSTEM  # noqa: E402
from restyle_mirror import RMM, STRICT_RULES, strict_violations  # noqa: E402

DATA = ROOT / "training-dataset"
GOLD = DATA / "specs" / "alp14"
EXAMPLE = "RMI_REALM_ACTIVATE"

DICT_BLOCK = """
Declarations available in this specification -- functions, types and constants.
Whenever one of them expresses a condition, use it by exactly this name; introduce
a new helper only when none of them fits.

----- declarations -----
{dictionary}
----- end of declarations -----

Section:
"""


def build_prompt(section):
    example = (DATA / "sections" / "alp14" / f"{EXAMPLE}_command.txt").read_text()
    dictionary = (GOLD / "preamble.rs").read_text().strip()
    head = RMM.format(example=example)
    head = head[: head.rstrip().rfind("Section:")].rstrip() + "\n"
    return SYSTEM, head + STRICT_RULES + DICT_BLOCK.format(dictionary=dictionary) + section


def check(cmd, orig, new):
    bad = list(strict_violations(new))
    if cmd not in new:
        bad.append("command name lost")
    if not re.search(r'\bpost:', new):
        bad.append("no post: rows")
    for h in ("Input values", "Output values", "Failure conditions", "Success conditions"):
        if h not in new:
            bad.append(f"missing heading: {h}")
    have = set(CODE.findall(new))
    lost = {c for c in CODE.findall(orig) if not any(h.startswith(c) for h in have)}
    if lost:
        bad.append(f"return codes lost: {sorted(lost)[:4]}")
    return bad


def commands(sections):
    return sorted(f.name[: -len("_command.txt")] for f in Path(sections).glob("*_command.txt")
                  if not f.name.startswith("._"))


def write(out, version, cmd, text):
    sd, pd = Path(out) / "sections" / version, Path(out) / "specs" / version
    sd.mkdir(parents=True, exist_ok=True)
    pd.mkdir(parents=True, exist_ok=True)
    if not (pd / "preamble.rs").exists():
        shutil.copy(GOLD / "preamble.rs", pd / "preamble.rs")
    (sd / f"{cmd}_command.txt").write_text(text.strip() + "\n")
    shutil.copy(GOLD / f"{cmd.lower()}_spec.rs", pd / f"{cmd.lower()}_spec.rs")


def clean(text):
    text = re.sub(r"<think>.*?</think>\s*", "", text, flags=re.S)
    m = re.search(r"```\w*\n(.*?)```", text, re.S)
    return (m.group(1) if m else text).strip()


def cmd_claude(a):
    from confab_probe import call_claude

    def one(cmd):
        sec = (Path(a.sections) / f"{cmd}_command.txt").read_text()
        system, user = build_prompt(sec)
        new, why, tries = "", ["not run"], 0
        for tries in (1, 2):
            try:
                new = clean(call_claude(system, user, a.model, a.effort))
            except Exception as e:  # noqa: BLE001
                why = [f"call failed: {e}"]
                continue
            why = check(cmd, sec, new)
            if not why:
                break
        if new:
            write(a.out, a.version, cmd, new)
        return cmd, {"attempts": tries, "problems": why, "written": bool(new)}

    with ThreadPoolExecutor(a.jobs) as ex:
        res = dict(ex.map(one, commands(a.sections)))
    (Path(a.out) / f"{a.version}.check.json").write_text(json.dumps(res, indent=1))
    ok = sum(not v["problems"] for v in res.values())
    print(f"[ir-claude] {sum(v['written'] for v in res.values())}/{len(res)} written, "
          f"{ok} pass the check", flush=True)


def cmd_ninebee(a):
    import os
    import torch
    from transformers import AutoModelForCausalLM, AutoTokenizer
    from eval_checkpoint import render_generation_prompt

    os.environ.setdefault("HF_HUB_OFFLINE", "1")
    tok = AutoTokenizer.from_pretrained("Qwen/Qwen3.5-9B")
    model = AutoModelForCausalLM.from_pretrained("Qwen/Qwen3.5-9B", dtype=torch.bfloat16,
                                                 device_map="auto")
    if a.model == "ft":
        from peft import PeftModel
        model = PeftModel.from_pretrained(model, a.adapter, subfolder="sft3-2/final")
    model.eval()

    cmds = commands(a.sections)[a.shard::a.nshards]
    res = {}
    for i, cmd in enumerate(cmds, 1):
        sec = (Path(a.sections) / f"{cmd}_command.txt").read_text()
        system, user = build_prompt(sec)
        msgs = [{"role": "system", "content": system}, {"role": "user", "content": user}]
        text = render_generation_prompt(tok, msgs)
        raw = tok(text, return_tensors="pt", add_special_tokens=False)
        ids = (raw["input_ids"] if hasattr(raw, "keys") else raw).to(model.device)
        with torch.no_grad():
            g = model.generate(ids, max_new_tokens=a.max_new_tokens, do_sample=False,
                               pad_token_id=tok.eos_token_id)
        new = clean(tok.decode(g[0][ids.shape[1]:], skip_special_tokens=True))
        write(a.out, a.version, cmd, new or "(empty)")
        res[cmd] = {"attempts": 1, "problems": check(cmd, sec, new), "written": True,
                    "prompt_tokens": int(ids.shape[1])}
        print(f"[ir-{a.model}] {i}/{len(cmds)} {cmd} problems={len(res[cmd]['problems'])}",
              flush=True)
    (Path(a.out) / f"{a.version}.check.{a.shard}.json").write_text(json.dumps(res, indent=1))
    print(f"IR_DONE {a.version} shard {a.shard}", flush=True)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("cmd", choices=["claude", "ninebee", "prompt"])
    ap.add_argument("--sections", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--version", required=True)
    ap.add_argument("--jobs", type=int, default=8)
    ap.add_argument("--model", default="claude-opus-5-5",
                    help="claude: model id; ninebee: ft or base")
    ap.add_argument("--effort", default="medium")
    ap.add_argument("--adapter", default=None, help="local adapter snapshot (ninebee ft)")
    ap.add_argument("--shard", type=int, default=0)
    ap.add_argument("--nshards", type=int, default=1)
    ap.add_argument("--max-new-tokens", type=int, default=4096)
    a = ap.parse_args()
    if a.cmd == "prompt":                       # print one prompt, for inspection
        system, user = build_prompt((Path(a.sections) / f"{commands(a.sections)[0]}_command.txt").read_text())
        print(system, "\n=====\n", user)
        return
    {"claude": cmd_claude, "ninebee": cmd_ninebee}[a.cmd](a)


if __name__ == "__main__":
    main()
