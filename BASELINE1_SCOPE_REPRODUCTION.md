# Baseline 1: Reproducing SCOPE's Original Bug Findings

**Goal** (advisor's Baseline 1 proposal): use **our own pipeline** — the
fine-tuned Qwen model — to generate Verus
code for the old ARM CCA RMM spec versions (`eac5`, `rel0`) SCOPE was
originally run against, then check whether SCOPE-style detection methods
applied to *our* generated code rediscover the bugs SCOPE originally found
(documented in the SCOPE paper, ASPLOS'26, Table 7 / Appendix D).

SCOPE's own known rule-mode TPs on eac5/rel0 (ground truth this report checks
against — reproduced by re-running SCOPE's own tool directly, matching the
paper's Appendix D.6.1/D.6.2 exactly):

| Check | Command | Field(s) |
|---|---|---|
| Dangling output | `RMI_RTT_READ_ENTRY` | `walk_level` |
| Dangling output | `RMI_VERSION` | `lower`, `higher` |
| Dangling output | `RSI_ATTESTATION_TOKEN_CONTINUE` | `len` |
| Dangling output | `RSI_ATTESTATION_TOKEN_INIT` | `size` |
| Dangling output | `RSI_IPA_STATE_GET` | `ripas` (+ `out_top` in rel0) |
| Dangling output | `RSI_MEASUREMENT_READ` | `value_0`..`value_7` |
| Dangling output | `RSI_VERSION` | `lower`, `higher` |
| Dangling output | `PSCI_VERSION` | `result` |

---

## Our Qwen pipeline generating eac5/rel0

### How

Used the same V3-prompt pipeline as the alp14 work (`run_qwen_baseline1_eac5_rel0.py`,
model `item_split_v4_best` — the retrained checkpoint from
[`prompt_engineering/RESULTS_V3.md`](prompt_engineering/RESULTS_V3.md) Iteration 7,
47/98 Verus pass rate on alp14) to generate Verus spec functions directly from
the eac5/rel0 PDF sections, with no reference to SCOPE's parser or gold specs
at generation time. Results: `results/baseline1_eac5_rel0/v4_qwen/{eac5,rel0}/`.

**Important caveat**: eac5 and rel0 are in the model's *training* split
(`dataset_loader.TRAIN_VERSIONS`), unlike alp14 (held-out test split) — the
model saw gold answers for these exact commands during fine-tuning. This is
a weaker test of generalization than alp14; treat it as "can our trained
pipeline's output be used to rediscover known bugs," not "can it generalize
to an unseen spec."

### CodeBLEU

| Version | Commands | CodeBLEU Best@1 |
|---|---|---|
| eac5 | 41 | 0.8139 |
| rel0 | 41 | 0.8056 |

(For reference, alp14 test-split CodeBLEU was 0.8389 in Iteration 7 — eac5/rel0
score slightly lower despite being training-split versions, since CodeBLEU is
measured against the un-formatted oracle text, not exact-match.)

### Method: dangling-output check applied to our own generated code

SCOPE's `--mode rule` doesn't accept external Verus code — it re-parses the
PDF into its own tables and never touches any file we generate. Instead,
reused SCOPE's PDF-derived outputs/footprint tables (`scope --target X --mode
raw`, generator-independent spec metadata) and re-implemented the
dangling-output check against our own generated spec functions
(`training/scope_rule_check_ourcode.py --raw-file ... --gen-dir ...`).
(The footprint check is excluded here — as with alp14, it produces mostly
noise due to our code's explicit `old_s`/`new_s` state-threading not matching
SCOPE's bound-variable PDF text style.)

### Results: do we rediscover SCOPE's known bugs?

**Important methodological correction**: the dangling-output check only tells
you "our generated code never mentions this output name" — it does **not**
by itself tell you *why*. A command can fail that check for three very
different reasons, which need to be told apart by reading the underlying PDF
text, not assumed:

1. The spec's own structured Success Conditions table genuinely never defines
   the output anywhere — the same reason SCOPE's own PDF-table parser flags
   it. This is a real "same root cause" reproduction of SCOPE's finding.
2. The spec *does* define the output, but only in free-text narrative outside
   the structured table (SCOPE's parser only reads the structured table, so
   it correctly can't see this) — and our model correctly extracted it, just
   using its own accessor-function naming (e.g. `rsi_version_lower(new_s)`)
   that a naive literal-name check doesn't recognize as "establishing" the
   output. This is a **false positive of the check itself**, not a finding.
3. The spec defines the output (either in the table or the narrative), but
   our model simply failed to translate it — a pure generation defect,
   unrelated to the specific gap SCOPE reported.

Checked each flagged command's actual PDF section text (`training-dataset/sections/eac5/`)
and generated code to classify it correctly:

| Command (output) | Spec structured table defines it? | Our code defines it (any naming)? | Verdict |
|---|---|---|---|
| `PSCI_VERSION` (`result`) | No — table says "does not have any success conditions" | No | **Same root cause as SCOPE** |
| `RSI_MEASUREMENT_READ` (`value_0..7`) | No — table says "does not have any success conditions" | No | **Same root cause as SCOPE** |
| `RSI_ATTESTATION_TOKEN_CONTINUE` (`len`) | Table exists but never mentions `len` | No | **Same root cause as SCOPE** |
| `RSI_ATTESTATION_TOKEN_INIT` (`size`) | Table exists but never mentions `size` | No | **Same root cause as SCOPE** |
| `RMI_RTT_READ_ENTRY` (`walk_level`) | Table exists but never mentions `walk_level` | No | **Same root cause as SCOPE** |
| `RSI_VERSION` (`lower`, `higher`) | No (table empty), but narrative prose spells out the exact a/b/c logic | **Yes** — via `rsi_version_lower(...)`/`rsi_version_higher(...)` | **False positive of our own checker — not a real finding** |
| `RMI_VERSION` (`lower`, `higher`) | No (table empty), narrative prose near-identical to `RSI_VERSION`'s | No — model didn't extract it here despite doing so for `RSI_VERSION` | **Inconsistent generation gap — not the same root cause as SCOPE** |
| `RMI_RTT_FOLD` (`rtt`) | **Yes** — table explicitly states `rtt == walk.rtte.addr` | No — parameter dropped entirely from the generated signature | **Not a spec bug at all — pure generation defect** (same category as the alp14 "missing from signature" findings, not a new SCOPE-style bug) |

### Bottom line

Using only our own trained model's output, with no reference to SCOPE's
parser or gold specs: **5 of SCOPE's 8 known rule-mode bugs on eac5/rel0 are
genuinely rediscovered with the same root cause** (`PSCI_VERSION`,
`RSI_MEASUREMENT_READ`, `RSI_ATTESTATION_TOKEN_CONTINUE`,
`RSI_ATTESTATION_TOKEN_INIT`, `RMI_RTT_READ_ENTRY`) — the spec's structured
Success Conditions text genuinely never defines these values, so neither
SCOPE's parser nor our model could produce something that isn't there. The
other 3 flagged commands are **not** valid reproductions once checked against
the source text: one (`RSI_VERSION`) is a false positive of our own
literal-name checker (the model did define it), one (`RMI_VERSION`) is a
generation inconsistency unrelated to SCOPE's finding, and one (`RMI_RTT_FOLD`)
is a plain generation defect where the spec clearly provides the answer and
our model dropped it — not a rediscovered spec bug at all.

### Not yet done

- **`reason`-mode equivalent**: SCOPE's cross-reference-with-summary-table
  checks use hand-written `assert` queries specific to SCOPE's own generated
  function signatures (`patch/eac5.patch`, `patch/rel0.patch`). These would
  need to be manually re-authored against our own generated signatures to
  test whether we also rediscover the reason-mode TPs (`RMI_DATA_DESTROY`,
  `RMI_RTT_DESTROY`, `RMI_RTT_INIT_RIPAS`). Not started.
- `--mode reason --is-coverage --no-dependency` (Table 9 robustness numbers) —
  not run.
- Baseline 2 (comparison against a general model/agent) — on hold.
