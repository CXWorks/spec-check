pub open spec fn sbi_pmu_counter_start_spec(counter_idx_base: UInt64, counter_idx_mask: UInt64, start_flags: UInt64, initial_value: uint64_t, result: sbiret, old_s: S, new_s: S) -> bool {
  true
}