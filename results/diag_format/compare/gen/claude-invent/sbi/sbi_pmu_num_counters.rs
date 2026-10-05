pub open spec fn sbi_pmu_num_counters_spec(error: i64, value: u64, old_s: S, new_s: S) -> bool {
    (error == SBI_SUCCESS)
    && (value == PmuNumCounters(old_s))
    && (new_s == old_s)
}
