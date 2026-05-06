pub open spec fn PSCI_CPU_SUSPEND_spec(
    old_s: S,
    new_s: S,
    fid: u64,
    power_state: u32,
    entry_point_address: Address,
    context_id: u64,
) -> bool {
    // No failure conditions
    // No success conditions
    // Control does not return to caller
    // Command causes REC exit due to PSCI
    true
}