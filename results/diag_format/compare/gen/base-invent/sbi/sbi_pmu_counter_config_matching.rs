pub open spec fn sbi_pmu_counter_config_matching_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let counter_idx_base = old_s.cmd_input_counter_idx_base;
    let counter_idx_mask = old_s.cmd_input_counter_idx_mask;
    let config_flags = old_s.cmd_input_config_flags;
    let event_idx = old_s.cmd_input_event_idx;
    let event_data = old_s.cmd_input_event_data;

    // Failure condition: Reserved bits (8:(XLEN-1)) must be zero
    ((config_flags & ((1u64 << 8) - 1)) == 0 ==> result.is_Ok())
    && (result.is_Err() ==> (config_flags & ((1u64 << 8) - 1)) != 0)
    && (result.is_Ok() ==> (new_s.pmu_counters == old_s.pmu_counters))
}