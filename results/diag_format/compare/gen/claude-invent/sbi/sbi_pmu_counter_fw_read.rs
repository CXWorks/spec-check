pub open spec fn sbi_pmu_counter_fw_read_spec(counter_idx: UInt64, ret: SbiRet, old_s: S, new_s: S) -> bool {
    (!IsFirmwareCounter(old_s, counter_idx) ==> (ret.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && (IsFirmwareCounter(old_s, counter_idx) ==> (
        ret.error == SBI_SUCCESS
        && (!IsRv32(old_s) ==> ret.value == FirmwareCounterValue(old_s, counter_idx))
        && (IsRv32(old_s) ==> ret.value == (FirmwareCounterValue(old_s, counter_idx) & 0xFFFF_FFFFu64))
        && new_s == old_s
    ))
}
