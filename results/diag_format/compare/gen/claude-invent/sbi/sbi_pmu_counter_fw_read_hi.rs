pub open spec fn sbi_pmu_counter_fw_read_hi_spec(counter_idx: u64, result: SbiRet, old_s: S, new_s: S) -> bool {
    (!IsFirmwareCounter(old_s, counter_idx) ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && (IsXlenAtLeast64(old_s) ==> result.value == 0)
    && (IsFirmwareCounter(old_s, counter_idx) ==> (
        result.error == SBI_SUCCESS
        && (!IsXlenAtLeast64(old_s) ==> result.value == ((FirmwareCounterValue(old_s, counter_idx) >> 32u64) & 0xFFFF_FFFFu64))
        && new_s == old_s
    ))
}
