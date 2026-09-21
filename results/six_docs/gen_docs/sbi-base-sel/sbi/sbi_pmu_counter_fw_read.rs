pub open spec fn sbi_pmu_counter_fw_read_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (counter_idx < 0 || counter_idx >= 16))
    && (result == SBI_SBI_SUCCESS ==> (new_s.pmu_counter_fw_read_value == old_s.pmu_counter_fw_read_value))
}