# Baseline 1: Reproducing SCOPE's Original Bug Findings

**Goal**: reproduce, using SCOPE's own two automated detection methods, the bugs
SCOPE originally reported on the old ARM CCA RMM spec versions (`eac5`, `rel0`),
as documented in the SCOPE paper (ASPLOS'26), Appendix D. This is the advisor's
Baseline 1 proposal — establishing that our environment/toolchain can reproduce
SCOPE's own numbers before comparing our pipeline against them.

Two of SCOPE's methods were reproduced directly (no involvement of our own LLM
pipeline or the `ensures false` self-contradiction sweep, which is a structurally
different, narrower method — see "Relation to our own pipeline" below):

- **`--mode reason`**: cross-references each command's Verus-formalized model
  against separately-authored architecture summary tables via
  `proof fn ... requires <spec_fn> { assert(...); }` queries, checked with Verus.
- **`--mode rule`**: purely structural — footprint check (declared footprint vs.
  actual state touched) + dangling-output check (output values missing from any
  postcondition). No SMT solving involved.

## Setup

Run on remote GPU server `grimlock.gtisc.gatech.edu`, under
`/mnt/md0/zhushan/scope-baseline/`:

- `scope/` — fresh clone of [`islet-project/scope`](https://github.com/islet-project/scope)
- `verus-scope/` — fresh clone of `verus-lang/verus`, pinned to commit
  `bec74a67d9281a4f51a7e1855760c5d16d8f63ff` (the exact commit the paper's
  reproducibility artifact requires), built with `vargo build --release`.
  ```
  Verus
    Version: 0.2025.01.11.bec74a6
    Profile: release
    Platform: linux_x86_64
    Toolchain: 1.79.0-x86_64-unknown-linux-gnu
  ```
  Kept fully separate from the server's other, newer Verus install
  (`v0.2026.04.12`) used elsewhere in our pipeline.
- Z3 4.12.5 (via `scope`'s `get-z3.sh`).
- `venv/` — Python venv (`pypdf`, `networkx`, `openai`) to work around the
  system pip's PEP 668 `externally-managed-environment` restriction.
- Spec PDFs: `DEN0137_1.0-eac5_rmm-arch_external.pdf` and
  `DEN0137_1.0-rel0_rmm-arch_external.pdf` (ARM's site blocks scripted
  downloads of these two with a 403; obtained via manual browser download).
  `alp11`/`alp12` PDFs were obtainable directly via ARM's CDN zip permalinks.

Commands (same for both versions, `X` = `eac5` or `rel0`):

```bash
export PATH=/mnt/md0/zhushan/scope-baseline/venv/bin:$PATH
cd /mnt/md0/zhushan/scope-baseline/scope

./scope --target X --input-type pdf --mode reason > X.rs
patch -p0 < ./patch/X.patch
/mnt/md0/zhushan/scope-baseline/verus-scope/source/target-verus/release/verus X.rs

./scope --target X --input-type pdf --mode rule > X_rule.txt
patch -p0 < ./patch/X_rule.patch
```

## Results

### `--mode reason` (formal reasoning against summary tables)

**eac5** — matches paper Appendix D.6.1 exactly: **9 total violations (3 TP + 6 FP)**.
Verus's default single-error-per-function reporting initially shows only 8 of the
9 (masking one behind another failing assert in the same function). Per the
paper's reproduction steps, commenting out the first-reported assert in
`rmi_rtt_init_ripas_rule` (line 1577) reveals the masked 9th violation (line 1578).

| Line | Function | Verdict |
|---|---|---|
| 1530 | `rmi_data_destroy_rule` | **TP** |
| 1545 | `rmi_rtt_create_rule` | FP |
| 1556 | `rmi_rtt_destroy_rule` | **TP** |
| 1566 | `rmi_rtt_fold_rule` | FP |
| 1567 | `rmi_rtt_fold_rule` | FP |
| 1575 | `rmi_rtt_init_ripas_rule` | **TP** |
| 1577 | `rmi_rtt_init_ripas_rule` | FP |
| 1578 | `rmi_rtt_init_ripas_rule` (masked, revealed in step 2) | FP |
| 1589 | `rmi_rtt_set_ripas_rule` | FP |

**rel0** — matches paper Appendix D.6.1 exactly: **8 total violations (2 TP + 6 FP)**,
no masking/reveal step needed.

| Line | Function | Verdict |
|---|---|---|
| 1581 | `rmi_data_destroy_rule` | **TP** |
| 1596 | `rmi_rtt_create_rule` | FP |
| 1607 | `rmi_rtt_destroy_rule` | **TP** |
| 1617 | `rmi_rtt_fold_rule` | FP |
| 1618 | `rmi_rtt_fold_rule` | FP |
| 1628 | `rmi_rtt_init_ripas_rule` | FP |
| 1629 | `rmi_rtt_init_ripas_rule` | FP |
| 1640 | `rmi_rtt_set_ripas_rule` | FP |

Notably, `rmi_rtt_init_ripas_rule` drops from 1 TP (in eac5) to 0 TP (in rel0) —
consistent with the rel0 PDF's own release notes, which list:

> RMI_RTT_INIT_RIPAS: correct inconsistency between text and command definition (FENIMORE-864)

confirming this specific bug (SCOPE's original motivating-example bug) was fixed
between eac5 and rel0.

### `--mode rule` (footprint + dangling-output check)

**eac5** and **rel0** both match paper Appendix D.6.2 exactly: **12 violating
commands = 10 pure TP + 1 pure FP + 1 mixed command**.

| Check | Command | Field(s) | Verdict |
|---|---|---|---|
| Dangling output | `RMI_RTT_READ_ENTRY` | `walk_level` | TP |
| Dangling output | `RMI_RTT_READ_ENTRY` | `desc` | FP |
| Dangling output | `RMI_VERSION` | `lower`, `higher` | TP |
| Dangling output | `RSI_ATTESTATION_TOKEN_CONTINUE` | `len` | TP |
| Dangling output | `RSI_ATTESTATION_TOKEN_INIT` | `size` | TP |
| Dangling output | `RSI_IPA_STATE_GET` | `ripas` (+ `out_top` in rel0 only) | TP |
| Dangling output | `RSI_MEASUREMENT_READ` | `value_0`..`value_7` | TP |
| Dangling output | `RSI_VERSION` | `lower`, `higher` | TP |
| Dangling output | `PSCI_VERSION` | `result` | TP |
| Footprint | `RSI_REALM_CONFIG` | `cfg.ipa_width`, `cfg.hash_algo` | FP |
| Footprint | `PSCI_CPU_ON` | `target_rec.pc` | TP |
| Footprint | `PSCI_SYSTEM_OFF` | `realm.state` | TP |
| Footprint | `PSCI_SYSTEM_RESET` | `realm.state` | TP |

`RSI_IPA_STATE_GET`'s dangling-output set differs slightly between versions
(rel0 additionally flags `out_top`) — a minor spec-formalization difference
between the two PDFs, not a TP/FP classification change.

## Summary

| Version | reason-mode | rule-mode |
|---|---|---|
| eac5 | 9 violations (3 TP + 6 FP) | 12 violations (10 TP + 1 FP + 1 mixed) |
| rel0 | 8 violations (2 TP + 6 FP) | 12 violations (10 TP + 1 FP + 1 mixed) |

Both fully match the paper's Appendix D expected results (D.6.1, D.6.2). This
confirms our environment (pinned Verus commit, Z3 version, SCOPE checkout, spec
PDFs) faithfully reproduces SCOPE's original tool and findings on the spec
versions it was originally run against.

## Relation to our own pipeline

Our own `training/inconsistency_analysis_rmm.py` `ensures false` sweep is a
**different, narrower method**: it only checks whether a single command's own
pre-/post-conditions are jointly satisfiable, without cross-referencing summary
tables (`reason`-mode) or checking footprint/output completeness (`rule`-mode).
This is why the two bugs we found independently on alp14
(`RMI_PDEV_STOP`, `RSI_ATTESTATION_TOKEN_CONTINUE`) don't overlap with SCOPE's
Table 7 list — they were found by a structurally different check, not
by re-running SCOPE's own methods.

## Remaining / not yet done

- `--mode reason --is-coverage --no-dependency` on eac5/rel0/alp11/alp12 (patches
  already present: `patch/*_coverage.patch`) — reproduces the paper's Table 9
  robustness numbers. Not yet run.
- Checking whether **our LLM-generated** Verus code (rather than SCOPE's own
  parser output) can also be evaluated with SCOPE's footprint/dangling-output
  rule checks — this would demonstrate our pipeline, not just a re-run of SCOPE,
  can independently surface known bugs. Not yet started.
- Baseline 2 (comparison against a general model/agent) — on hold per advisor
  discussion, scope not yet finalized.
