pub open spec fn sbi_pmu_counter_fw_read_hi_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (result == SBI_SBI_ERR_INVALID_PARAM))
    && (result == SBI_SBI_SUCCESS ==> (result == SBI_SBI_SUCCESS))
    && (result == SBI_SBI_SUCCESS ==> (result == 0))
}