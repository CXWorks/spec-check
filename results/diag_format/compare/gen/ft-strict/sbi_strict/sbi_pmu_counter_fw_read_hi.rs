pub open spec fn sbi_pmu_counter_fw_read_hi_spec(counter_idx: UInt, error: SbiErrorCode, value: UInt, old_s: S, new_s: S) -> bool {
  (IsHardwareCounter(old_s, counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (!IsValidCounter(old_s, counter_idx) ==> ResultEqual(error, SBI_ERR_INVALID_PARAM))
  && (ResultEqual(error, SBI_SUCCESS) ==> (Xlen() == 32) ==> (value == Bits(FirmwareCounterValue(old_s, counter_idx), 63, 32)))
  && (ResultEqual(error, SBI_SUCCESS) ==> (Xlen() >= 64) ==> (value == 0))
  && ((!(IsHardwareCounter(old_s, counter_idx)) &&
       IsValidCounter(old_s, counter_idx))
    ==> ResultEqual(error, SBI_SUCCESS))
}