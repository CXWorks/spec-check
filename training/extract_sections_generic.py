#!/usr/bin/env python3
"""Split a spec PDF into one text file per command.

    python3 training/extract_sections_generic.py sdei
    python3 training/extract_sections_generic.py --all
    python3 training/extract_sections_generic.py --selftest    # rebuild psci_13, diff

Modelled on training/extract_sections_psci.py, which does the same job for one
document with its patterns hard-coded. The documents in specs/ differ only in
three things -- what a command heading looks like, what the running headers and
footers are, and where the command chapter ends -- so those are config here and
the splitting logic is shared.

WHY --selftest EXISTS. A section splitter fails silently. It does not crash on a
bad boundary; it emits a file that looks entirely normal and is quietly wrong.
Extracting DEN0137 2.0 BET3 produced a 3554-line PSCI_VERSION -- the true length
is 32 -- because that command is the last of its chapter, so "cut at the next
command heading" ran on into the following types section and past it. Nothing
reported an error. It was caught only because the line count stood out.

`--selftest` rebuilds psci_13 through this code and diffs against the 22 section
files already committed from the original extractor. Run it before trusting a
new document's output.
"""
import argparse, re, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPECS = ROOT / "specs"
OUT_ROOT = ROOT / "training-dataset" / "sections"
IDENT = re.compile(r'[A-Z][A-Z0-9_]*')

# Noise every Arm PDF repeats on each page. Per-document extras go in `noise`.
COMMON_NOISE = [
    r'Copyright.*(?:ARM|Arm|Limited)',
    r'^\d+\.\d+\s+Non-confidential\s*$',
    r'^Non-Confidential',
    r'^Page\s+\d+\s+of\s+\d+',
    r'\.{4,}\s*\d+\s*$',          # table-of-contents dot leaders
    r'^\d+\s*$',                  # a lone page number
    r'^Chapter \d+\.',            # running header
]

DOCS = {
    "sdei": dict(
        pdf="ARM_DEN0054C_Software_Delegated_Exception_Interface.pdf",
        # "5.1.2   SDEI_EVENT_REGISTER" -- the name must end the line, which is
        # what separates a body heading from its contents-page twin.
        heading=r'(?m)^\s*(\d+\.\d+(?:\.\d+)?)\s+(SDEI_[A-Za-z0-9_/ ]*[A-Z0-9_])\s*$',
        stop=r'(?m)^\s*(?:6|7|8|9|Appendix|Glossary)\b',
        noise=[r'^DEN0054', r'^Software Delegated Exception Interface'],
    ),
    "drtm": dict(
        pdf="DEN0113_DRTM_1.4.pdf",
        heading=r'(?m)^\s*(\d+\.\d+(?:\.\d+)?)\s+(DRTM_[A-Za-z0-9_/ ]*[A-Z0-9_])\s*$',
        stop=r'(?m)^\s*(?:Appendix|Glossary)\b',
        noise=[r'^DEN0113', r'^Dynamic Root of Trust'],
    ),
    "ffa": dict(
        pdf="DEN0077A_Firmware_Framework_Arm_A-profile_1.3_ALP4.pdf",
        heading=r'(?m)^\s*(\d+\.\d+(?:\.\d+)?)\s+(FFA_[A-Za-z0-9_/ ]*[A-Z0-9_])\s*$',
        stop=r'(?m)^\s*(?:Appendix|Glossary)\b',
        # The running footer is "13.14. FFA_ABORT" -- section number, a dot,
        # a command name. A real FF-A heading has no dot after the number
        # ("13.2 FFA_VERSION"), so the dot is what tells them apart. Without
        # this the footer reads as a same-depth boundary and truncates every
        # section: FFA_VERSION came out at 37 lines instead of ~180.
        noise=[r'^DEN0077', r'^Arm Firmware Framework',
               r'^\s*\d+\.\d+\.\s+FFA_[A-Z0-9_]+\s*$'],
    ),
    "scmi": dict(
        pdf="DEN0056F_System_Control_and_Management_Interface_v4.0-bet0.pdf",
        # Four levels: "3.2.2.1   PROTOCOL_VERSION".
        heading=r'(?m)^\s*(\d+(?:\.\d+){2,3})\s+([A-Z][A-Z0-9_]{2,})\s*$',
        stop=None,
        noise=[r'^DEN0056', r'^System Control and Management Interface'],
        # SCMI repeats command names across protocols -- every protocol has its
        # own PROTOCOL_VERSION -- so the bare name is not unique. Qualify with
        # the section number, which is.
        qualify=True,
    ),
    "sbi": dict(
        pdf="riscv-sbi.pdf",
        # "4.1. Function: Get SBI specification version (FID #0)". The real
        # name is the C declaration on the next non-blank line, so the heading
        # only has to be located; name_from does the naming.
        heading=r'(?m)^(\d+\.\d+)\.\s+Function:.*\(FID #\d+\)\s*$',
        stop=None,
        noise=[r'^RISC-V', r'^\s*Chapter \d'],
        name_from=r'\b(sbi_[a-z0-9_]+)\s*\(',
    ),
    # psci_13 is not listed: it keeps its original extractor. It is the fixture
    # --selftest rebuilds, so giving it a second definition here would make the
    # test compare this file against itself.
}


def preprocess(text, extra_noise):
    pats = [re.compile(p, re.I) for p in COMMON_NOISE + list(extra_noise)]
    keep = []
    for raw in text.splitlines(keepends=True):
        line = raw.strip()
        if not line:
            continue
        if any(p.search(line) for p in pats):
            continue
        keep.append(raw)
    return "".join(keep)


def names_in(title):
    """A heading can name more than one command. Two shapes occur:

        "MIGRATE_INFO_TYPE and MIGRATE_INFO_UP_CPU"  -> both
        "PSCI_STAT_RESIDENCY/COUNT"                  -> RESIDENCY and _COUNT

    Taken from extract_sections_psci.py:_parse_cmd_names. Capturing only the
    first identifier silently drops the second, which is how the first draft of
    this file found 18 of psci_13's 22 commands -- the self-test caught it.
    """
    t = title.strip()
    if " and " in t:
        out = [p.strip().rstrip('.,') for p in t.split(" and ") if p.strip()]
    elif "/" in t:
        base, suffix = t.rsplit("/", 1)
        base = base.strip().rstrip('.,')
        out = [base, base.rsplit("_", 1)[0] + "_" + suffix.strip().upper()]
    else:
        out = [t.rstrip('.,')]
    # Splitting on " and " also splits prose titles: "Arguments and return
    # values in PSCI" yielded two bogus commands until this filter. A command
    # name is an all-caps identifier, never an English phrase.
    return [n for n in out if IDENT.fullmatch(n)]


# The trailing dot is optional: Arm writes "3.14 Title", RISC-V SBI writes
# "16.2. Title". Requiring no dot let sbi_steal_time_set_shmem run 346 lines
# into the next chapter, because nothing matched as a same-depth boundary.
ANY_HEADING = re.compile(r'(?m)^\s*(\d+(?:\.\d+)+)\.?\s+\S')


def split(text, heading, stop, name_from=None, qualify=False):
    """{command: section text}, bounded by three things.

    The next command heading is not enough on its own: the last command of a
    chapter then swallows everything after it. DRTM_PARAMETERS came out at 1704
    lines against a 39-line median, running on through
    "3.14 MEMORY_REGION_DESCRIPTOR_TABLE"; FFA_MSG_POLL did the same at 1637.
    DEN0137 2.0 BET3 hit this too, at 3554 lines.

    So each section also stops at the next heading of the SAME DEPTH. Depth, not
    any heading: a command's own subsections ("5.6.1 Intended use") are one
    level deeper and must not terminate it, while the next sibling section
    ("3.14 ...") must.
    """
    heads = list(re.finditer(heading, text))
    stop_pat = re.compile(stop) if stop else None
    out = {}
    for i, m in enumerate(heads):
        hard = heads[i + 1].start() if i + 1 < len(heads) else len(text)
        depth = m.group(1).count(".")
        for h in ANY_HEADING.finditer(text, m.end()):
            if h.group(1).count(".") <= depth:
                hard = min(hard, h.start())
                break
        if stop_pat:
            s = stop_pat.search(text, m.end())
            if s:
                hard = min(hard, s.start())
        body = text[m.start():hard]
        if name_from:
            # The heading is a human title; the identifier lives in the body.
            got = re.search(name_from, body)
            found = [got.group(1)] if got else []
        else:
            found = names_in(m.group(2))
        for nm in found:
            # Suffix, not prefix. Prefixing put the section number first
            # ("3.10.3.13_POWERCAP_DOMAIN_NAME_GET"), so the generated function
            # name began with a digit and was not a legal identifier -- the
            # spec parser then failed on 151 of SCMI's 153 files and the
            # vacuity check silently fell back to scanning whole files, which
            # never equal "true". SCMI looked like the one document where the
            # fine-tuned model was NOT vacuous. It was a naming artifact.
            key = f"{nm}__{m.group(1).replace('.', '_')}" if qualify else nm
            out[key] = body
    return out


def run(name, cfg, out_dir=None, write=True):
    pdf = SPECS / cfg["pdf"]
    if not pdf.exists():
        sys.exit(f"missing {pdf}")
    import subprocess
    raw = subprocess.run(["pdftotext", "-layout", str(pdf), "-"],
                         capture_output=True, text=True, check=True).stdout
    cmds = split(preprocess(raw, cfg.get("noise", [])),
                 cfg["heading"], cfg.get("stop"),
                 cfg.get("name_from"), cfg.get("qualify", False))
    if write:
        d = Path(out_dir) if out_dir else OUT_ROOT / name
        d.mkdir(parents=True, exist_ok=True)
        for c, body in cmds.items():
            (d / f"{c}_command.txt").write_text(body)
    lens = sorted(len(b.splitlines()) for b in cmds.values())
    if cmds:
        print(f"{name:6s} {len(cmds):3d} commands  lines min/median/max = "
              f"{lens[0]}/{lens[len(lens)//2]}/{lens[-1]}")
        # A max far above the median is the overrun signature, not a long command.
        if lens[-1] > 10 * max(lens[len(lens) // 2], 1):
            print(f"       ⚠️  longest is {lens[-1]//max(lens[len(lens)//2],1)}x "
                  f"the median -- likely a section that ran past its end")
    return cmds


def selftest():
    """Rebuild psci_13 here and diff against the committed section files."""
    cfg = dict(
        pdf="DEN0022F.b_Power_State_Coordination_Interface.pdf",
        heading=r'(?m)^\s*(5\.\d+)\s+([A-Z][A-Za-z0-9_/ ]*[A-Z0-9_])\s*$',
        stop=r'(?m)^\s*[6-9]\.\d+(?!\.\d)\s+\S',
        noise=[r'^DEN0022', r'^Power State Coordination Interface'],
    )
    got = run("psci_13", cfg, write=False)
    ref_dir = OUT_ROOT / "psci_13"
    ref = {p.name[:-len("_command.txt")] for p in ref_dir.glob("*_command.txt")}
    mine = set(got)
    print(f"\n[selftest] committed {len(ref)} commands, this code found {len(mine)}")
    if mine == ref:
        print("[selftest] PASS -- same command set")
        return True
    print(f"[selftest] FAIL")
    if ref - mine:
        print(f"  missed : {sorted(ref - mine)}")
    if mine - ref:
        print(f"  extra  : {sorted(mine - ref)}")
    return False


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("doc", nargs="?", choices=sorted(DOCS))
    ap.add_argument("--all", action="store_true")
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--dry-run", action="store_true")
    a = ap.parse_args()
    if a.selftest:
        return 0 if selftest() else 1
    names = sorted(DOCS) if a.all else ([a.doc] if a.doc else [])
    if not names:
        ap.error("give a document, --all, or --selftest")
    for n in names:
        run(n, DOCS[n], write=not a.dry_run)
    return 0


if __name__ == "__main__":
    sys.exit(main())
