pub open spec fn sbi_pmu_counter_start_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error != 0 ==> (
        (old_s.counter_idx_base as int < 0) ||
        (old_s.counter_idx_mask as int < 0) ||
        (old_s.counter_idx_base as int > (old_s.counter_idx_mask as int)) ||
        (old_s.start_flags & SBI_PMU_START_SET_INIT_VALUE != 0) && (old_s.start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT != 0) ||
        (old_s.start_flags & SBI_PMU_START_SET_INIT_VALUE == 0) && (old_s.start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT == 0) && (old_s.start_flags & SBI_PMU_START_SET_INIT_VALUE != 0)
    ))
    && (result.error == 0 ==> (
        (old_s.counter_idx_base as int >= 0) &&
        (old_s.counter_idx_mask as int >= 0) &&
        (old_s.counter_idx_base as int <= (old_s.counter_idx_mask as int)) &&
        ((old_s.start_flags & SBI_PMU_START_SET_INIT_VALUE != 0) || (old_s.start_flags & SBI_PMU_START_FLAG_INIT_SNAPSHOT != 0))
    ))
}