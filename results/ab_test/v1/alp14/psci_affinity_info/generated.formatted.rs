```rust
pub open spec fn PSCI_AFFINITY_INFO_spec(
    s: S,
    target_affinity: u64,
    lowest_affinity_level: u32,
) -> (result: u64) {
    // Failure condition: target_bound
    if lowest_affinity_level != 0 {
        result == PSCI_INVALID_PARAMETERS()
    }
    // Failure condition: target_match
    else if !MpidrIsUsed(s, target_affinity) {
        result == PSCI_INVALID_PARAMETERS()
    }
    // Success condition: runnable
    else if RecFromMpidr(s, target_affinity).flags.runnable == RUNNABLE() {
        result == PSCI_SUCCESS()
    }
    // Success condition: not_runnable
    else if RecFromMpidr(s, target_affinity).flags.runnable == NOT_RUNNABLE() {
        result == PSCI_OFF()
    }
    // Default case (should not occur if preconditions are met)
    else {
        false
    }
}
```

The specification captures:
- **Inputs**: `target_affinity` (X1, u64), `lowest_affinity_level` (X2[31:0], u32)
- **Output**: `result` (X0, u64) as a `PsciReturnCode`
- **Failure conditions** (ordered first, short-circuit evaluation):
  - `target_bound`: Returns `PSCI_INVALID_PARAMETERS` if `lowest_affinity_level != 0`
  - `target_match`: Returns `PSCI_INVALID_PARAMETERS` if the MPIDR is not used
- **Success conditions**:
  - `runnable`: Returns `PSCI_SUCCESS` if the target REC is runnable
  - `not_runnable`: Returns `PSCI_OFF` if the target REC is not runnable