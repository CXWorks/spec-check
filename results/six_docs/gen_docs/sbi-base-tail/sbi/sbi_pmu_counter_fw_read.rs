pub open spec fn sbi_pmu_counter_fw_read_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == SBI_SBI_ERR_INVALID_PARAM ==> (old_s as int) == (new_s as int))
    && (result == SBI_SBI_SUCCESS ==> (new_s as int) == (old_s as int))
}