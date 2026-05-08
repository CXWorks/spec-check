```verus
pub open spec fn PSCI_CPU_SUSPEND_spec(
    s: S,
    fid: u64,
    power_state: u32,
    entry_point_address: Address,
    context_id: u64,
) -> (result: RecExit) {
    // PSCI_CPU_SUSPEND causes a REC exit due to PSCI
    // The entry_point_address and context_id arguments are ignored by RMM
    // Control does not return to the caller
    RecExit::Psci
}
```