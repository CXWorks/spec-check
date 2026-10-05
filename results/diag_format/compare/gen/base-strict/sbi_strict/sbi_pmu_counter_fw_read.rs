pub open spec fn sbi_pmu_counter_fw_read_spec(error: i64, value: u64, counter_idx: u64, old_s: S, new_s: S) -> bool {
    (IsHardwareCounter(counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (!IsValidCounter(counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(error, SBI_SUCCESS) ==> (
        (!IsRv32() ==> value == FirmwareCounterValue(counter_idx))
        && (IsRv32() ==> value == Bits(FirmwareCounterValue(counter_idx), 31, 0))
    ))
    && (old_s == new_s)
}