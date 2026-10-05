pub open spec fn sbi_pmu_counter_fw_read_hi_spec(error: SbiErrorCode, value: UInt, old_s: S, new_s: S) -> bool {
    (IsHardwareCounter(old_s, counter_idx(old_s)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsValidCounter(old_s, counter_idx(old_s)) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(error, SBI_SUCCESS) ==> (
        (Xlen() == 32) ==> (value == Bits(FirmwareCounterValue(counter_idx(old_s)), 63, 32))
        && (Xlen() >= 64) ==> (value == 0)
    ))
    && (old_s == new_s)
}