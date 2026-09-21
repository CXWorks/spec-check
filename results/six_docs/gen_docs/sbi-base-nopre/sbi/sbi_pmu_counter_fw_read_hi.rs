pub open spec fn sbi_pmu_counter_fw_read_hi_spec(result: SbiRet, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (result.value == 0 && (new_s == old_s)))
    && (result.error == SBI_SUCCESS ==> (result.value == 0))
}