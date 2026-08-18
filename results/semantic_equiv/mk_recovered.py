import json, pathlib, sys
# For each command where sampling recovered a pass that greedy missed, take the
# first PASSING sampled spec. If pass@k headroom is real, these should be
# semantically equivalent to gold at a rate like the greedy ones. If they are
# mostly wrong, the headroom is just extra chances to compile something.
for f in sys.argv[1:]:
    d = json.loads(pathlib.Path(f).read_text())
    out = []
    for r in d["results"]:
        if not r.get("samples") or r["pass"] or not r.get("any_pass"):
            continue
        hit = next((s for s in r["samples"] if s["pass"]), None)
        if hit:
            out.append({"command": r["command"], "pass": True, "generated": hit["generated"]})
    name = pathlib.Path(f).stem + "-recovered"
    pathlib.Path(f"{name}.json").write_text(json.dumps({"summary": {}, "results": out}))
    print(f"{name}: {len(out)} sampling-recovered specs")
