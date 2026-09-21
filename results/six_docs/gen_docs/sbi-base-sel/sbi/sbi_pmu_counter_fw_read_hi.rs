pub open spec fn sbi_pmu_counter_fw_read_hi_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (counter_idx < 0 || counter_idx >= old_s.pmu_counter_fw_read_hi_max))
    && (result == SBI_SBI_SUCCESS ==> (result == 0))
    && (result == SBI_SBI_ERR_INVALID_PARAM ==> (new_s == old_s))
    && (result == SBI_SBI_SUCCESS ==> (new_s == old_s))
}