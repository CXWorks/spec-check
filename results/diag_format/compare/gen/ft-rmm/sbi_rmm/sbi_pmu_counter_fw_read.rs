pub open spec fn sbi_pmu_counter_fw_read_spec(counter_idx: unsigned long, error: long, value: unsigned long, old_s: S, new_s: S) -> bool {
  (!IsValidCounter(old_s, counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (IsHardwareCounter(old_s, counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (ResultEqual(error, SBI_SUCCESS) ==> value == FirmwareCounterValue(new_s, counter_idx))
  && (ResultEqual(error, SBI_SUCCESS) && IsRV32(new_s) ==> value == FirmwareCounterValue(new_s, counter_idx)[31:0])
  && ((IsValidCounter(old_s, counter_idx) &&
       !IsHardwareCounter(old_s, counter_idx))
    ==> ResultEqual(error, SBI_SUCCESS))
}