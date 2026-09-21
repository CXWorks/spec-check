pub open spec fn sbi_pmu_num_counters_spec(result: int, old_s: S, new_s: S) -> bool {
    result == SBI_SBI_SUCCESS
}