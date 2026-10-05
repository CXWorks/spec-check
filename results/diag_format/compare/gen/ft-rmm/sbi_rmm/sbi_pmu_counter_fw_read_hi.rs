pub open spec fn sbi_pmu_counter_fw_read_hi_spec(counter_idx: unsigned long, error: long, value: unsigned long, old_s: S, new_s: S) -> bool {
  (IsHardwareCounter(old_s, counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (!IsValidCounter(old_s, counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (ResultEqual(error, SBI_SUCCESS) ==> ResultEqual(error, SBI_SUCCESS))
  && (ResultEqual(error, SBI_SUCCESS) ==> value == FirmwareCounterValue(new_s, counter_idx)[63:32])
  && (ResultEqual(error, SBI_SUCCESS) && (64) <= XLEN ==> value == 0)
  && ((!(IsHardwareCounter(old_s, counter_idx)) &&
       IsValidCounter(old_s, counter_idx))
    ==> ResultEqual(error, SBI_SUCCESS))
}