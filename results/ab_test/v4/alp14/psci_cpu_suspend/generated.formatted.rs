pub open spec fn PSCI_CPU_SUSPEND_spec(
    old_s: S,
    new_s: S,
    fid: u64,
    power_state: u32,
    entry_point_address: Address,
    context_id: u64,
) -> bool {
    fid == 0xC4000001
}