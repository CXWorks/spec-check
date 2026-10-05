pub open spec fn sbi_pmu_counter_fw_read_hi_spec(error: long, value: unsigned long, counter_idx: unsigned long, old_s: S, new_s: S) -> bool {
    (IsHardwareCounter(counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsValidCounter(counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(error, SBI_SUCCESS) ==> value == FirmwareCounterValue(counter_idx)[63:32])
    && (XLEN >= 64 ==> (ResultEqual(error, SBI_SUCCESS) ==> value == 0))
}