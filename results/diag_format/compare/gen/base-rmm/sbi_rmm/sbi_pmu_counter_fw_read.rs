pub open spec fn sbi_pmu_counter_fw_read_spec(error: long, value: unsigned long, counter_idx: unsigned long, old_s: S, new_s: S) -> bool {
    (!IsValidCounter(counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (IsHardwareCounter(counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
    && (ResultEqual(error, SBI_SUCCESS) ==> value == FirmwareCounterValue(counter_idx))
    && (IsRV32() ==> value == FirmwareCounterValue(counter_idx)[31:0])
    && (old_s == new_s)
}