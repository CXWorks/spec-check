#!/usr/bin/env python3
"""Run gen_specs.py over many versions on a fixed set of GPUs, one process per GPU.

    queue_gen.py --gpus 0 1 2 --model ft --versions scmi_rmm sbi_rmm --tag ft-rmm

Each version is split into chunks of at most --chunk commands (written as
sections/<v>__k<i>/), so a 153-command document does not pin one GPU while the
others sit idle. Chunks go to whichever GPU frees first. A chunk whose process
exits non-zero, or that writes fewer files than it has commands, is retried
once. When everything is done the chunk outputs are merged into
<out>/<tag>/<v>/ and a line `QUEUE_DONE <tag> ...` is printed -- completion is
read from that line and from file counts, never from the process table.
"""
import argparse, os, shutil, subprocess, sys, time
from pathlib import Path

HOME = Path.home()
REPO = HOME / "spec-check"
DATA = REPO / "training-dataset"
AD = (HOME / ".cache/huggingface/hub/models--jisenli--spec-check-ckpt/snapshots"
      / "7b59ac04600bb54d60a4fc30e1e0c0dc25675992")

ap = argparse.ArgumentParser()
ap.add_argument("--gpus", nargs="+", required=True)
ap.add_argument("--model", choices=["ft", "base"], required=True)
ap.add_argument("--versions", nargs="+", required=True)
ap.add_argument("--tag", required=True)
ap.add_argument("--chunk", type=int, default=12)
ap.add_argument("--out", default=str(HOME / "diag" / "gen3"))
ap.add_argument("--system-file", default=None)
a = ap.parse_args()

out = Path(a.out) / a.tag
logs = HOME / "diag" / "logs" / a.tag
out.mkdir(parents=True, exist_ok=True); logs.mkdir(parents=True, exist_ok=True)

chunks = []                                   # (chunk version, n commands, parent)
for v in a.versions:
    names = sorted(n for n in os.listdir(DATA / "sections" / v) if not n.startswith("._"))
    for i in range(0, len(names), a.chunk):
        # The tag is part of the name: two queues sharing chunk directories
        # delete each other's inputs on cleanup (base-rmm4 lost all of SBI).
        cv = f"{v}__{a.tag}_k{i // a.chunk}"
        sd, pd = DATA / "sections" / cv, DATA / "specs" / cv
        shutil.rmtree(sd, ignore_errors=True); sd.mkdir(parents=True)
        pd.mkdir(parents=True, exist_ok=True)
        shutil.copy(DATA / "specs" / v / "preamble.rs", pd / "preamble.rs")
        for n in names[i:i + a.chunk]:
            shutil.copy(DATA / "sections" / v / n, sd / n)
        chunks.append((cv, len(names[i:i + a.chunk]), v))
print(f"[queue] {a.tag}: {len(chunks)} chunks over gpus {a.gpus}", flush=True)

env = dict(os.environ, HF_HUB_OFFLINE="1", TRANSFORMERS_OFFLINE="1")
model = (["--adapter", str(AD), "--subfolder", "sft3-2/final"] if a.model == "ft" else [])
cmd0 = [str(REPO / ".venv/bin/python"), "scripts/gen_specs.py", "--base", "Qwen/Qwen3.5-9B",
        *model, "--prompt-variant", "v3.1", "--no-gold", "--out-dir", str(out / "_chunks"),
        *(["--system-file", a.system_file] if a.system_file else [])]

todo = [(c, n, p, 0) for c, n, p in chunks]
running = {}                                   # gpu -> (proc, chunk, n, parent, attempt)
failed = []
while todo or running:
    for g in a.gpus:
        if g not in running and todo:
            c, n, p, att = todo.pop(0)
            lf = open(logs / f"{c}.log", "w")
            proc = subprocess.Popen(cmd0 + ["--versions", c], cwd=REPO, stdout=lf,
                                    stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL,
                                    env=dict(env, CUDA_VISIBLE_DEVICES=g))
            running[g] = (proc, c, n, p, att)
            print(f"[queue] gpu{g} start {c} ({n})", flush=True)
    time.sleep(10)
    for g, (proc, c, n, p, att) in list(running.items()):
        if proc.poll() is None:
            continue
        del running[g]
        got = len(list((out / "_chunks" / c).glob("*.rs"))) if (out / "_chunks" / c).exists() else 0
        if proc.returncode == 0 and got >= n:
            print(f"[queue] gpu{g} done  {c} {got}/{n}", flush=True)
        elif att == 0:
            print(f"[queue] gpu{g} RETRY {c} rc={proc.returncode} {got}/{n}", flush=True)
            todo.append((c, n, p, 1))
        else:
            print(f"[queue] gpu{g} FAILED {c} rc={proc.returncode} {got}/{n}", flush=True)
            failed.append(c)

summary = []
for v in a.versions:
    dst = out / v; dst.mkdir(parents=True, exist_ok=True)
    for c, n, p in chunks:
        if p == v and (out / "_chunks" / c).exists():
            for f in (out / "_chunks" / c).glob("*.rs"):
                shutil.copy(f, dst / f.name)
    total = len(os.listdir(DATA / "sections" / v))
    summary.append(f"{v}={len(list(dst.glob('*.rs')))}/{total}")
for c, n, p in chunks:                         # chunk inputs are scaffolding only
    shutil.rmtree(DATA / "sections" / c, ignore_errors=True)
    shutil.rmtree(DATA / "specs" / c, ignore_errors=True)
print(f"QUEUE_DONE {a.tag} {' '.join(summary)} failed={failed}", flush=True)
