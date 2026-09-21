pub open spec fn sbi_pmu_counter_stop_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (
        (counter_idx_base as int) < 0 ||
        (counter_idx_mask as int) < 0 ||
        (stop_flags as int) < 0 ||
        ((stop_flags as int) & 0x3 != 0) ||
        ((stop_flags as int) & 0x3 == 0 && (stop_flags as int) != 0)
    ))
    && (result == SBI_SBI_ERR_ALREADY_STOPPED ==> (
        // At least one counter in the set is already stopped
        true
    ))
    && (result == SBI_SBI_ERR_NO_SHMEM ==> (
        (stop_flags as int) & 0x2 != 0 &&
        !old_s.shmem_available
    ))
    && (result == SBI_SBI_SUCCESS ==> (
        // All counters in the set are now stopped
        // Snapshot taken if flag set and shmem available
        // Overflown counter bitmap updated
        true
    ))
}