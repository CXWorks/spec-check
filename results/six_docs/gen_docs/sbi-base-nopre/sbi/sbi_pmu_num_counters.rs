pub open spec fn sbi_pmu_num_counters_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_SUCCESS)
    && (result.value >= 0)
}