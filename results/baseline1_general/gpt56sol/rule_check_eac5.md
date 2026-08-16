# Dangling-output rule check: GPT gpt-5.6-sol (high) — eac5 only, round 1

## eac5

- Commands checked: **41**
- SCOPE rule-mode TPs rediscovered: **7/8**
- Extra flags (not in SCOPE's TP list): **1**

| SCOPE TP command | Expected fields | Flagged in our run | Rediscovered |
|---|---|---|---|
| `PSCI_VERSION` | result | result | yes |
| `RMI_RTT_READ_ENTRY` | walk_level | — | **no** |
| `RMI_VERSION` | lower, higher | lower, higher | yes |
| `RSI_ATTESTATION_TOKEN_CONTINUE` | len | len | yes |
| `RSI_ATTESTATION_TOKEN_INIT` | size | size | yes |
| `RSI_IPA_STATE_GET` | ripas | ripas | yes |
| `RSI_MEASUREMENT_READ` | value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7 | value_0, value_1, value_2, value_3, value_4, value_5, value_6, value_7 | yes |
| `RSI_VERSION` | lower, higher | lower, higher | yes |

### Extra flags — need source-verified verdicts

| Command | Output | Conditions table mentions it? | Verdict |
|---|---|---|---|
| `RSI_FEATURES` | `value` | yes | TODO |
