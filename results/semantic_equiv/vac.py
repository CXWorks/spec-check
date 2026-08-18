import pathlib, json
g = pathlib.Path("training-dataset/specs/alp14")
vac, tiny = [], []
for f in sorted(g.glob("*_spec.rs")):
    t = f.read_text(errors="ignore")
    b = t[t.find("{") + 1: t.rfind("}")].strip()
    if b in ("true", "true,"):
        vac.append(f.stem)
    elif len(b) < 40:
        tiny.append((f.stem, b[:40]))
print(f"  vacuous (exactly true): {len(vac)}/98")
for v in vac:
    print("     ", v)
print(f"  very short (<40 chars): {len(tiny)}")
for n, b in tiny[:6]:
    print(f"      {n}: {b}")
sp = pathlib.Path("training-dataset/dataset_clean/splits.json")
if sp.exists():
    held = set(json.load(open(sp))["cmd_test"])
    vac_up = {x.upper().replace("_SPEC", "") for x in vac}
    print("  vacuous AND held-out:", sorted(vac_up & held))
