pub open spec fn sbi_pmu_counter_fw_read_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> (result.value == 0))
    && (result.error == SBI_SUCCESS ==> (result.value == old_s.pmu_counter_fw[old_s.pmu_counter_idx]))
    && (result.error != SBI_SUCCESS && result.error != SBI_SUCCESS ==> (result.value == 0))
}