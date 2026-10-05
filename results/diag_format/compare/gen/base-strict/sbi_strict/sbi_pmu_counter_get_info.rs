pub open spec fn sbi_pmu_counter_get_info_spec(result: SbiErrorCode, value: CounterInfo, old_s: S, new_s: S) -> bool {
    (result != SBI_SUCCESS ==> true)
    && (result == SBI_SUCCESS ==> (
        Bits(value, XLEN - 1, XLEN - 1) == CounterTypeEncoding(old_s.cmd_input_counter_idx)
        && ((Bits(value, XLEN - 1, XLEN - 1) == 0) == IsHardwareCounter(old_s.cmd_input_counter_idx))
        && ((Bits(value, XLEN - 1, XLEN - 1) == 1) == IsFirmwareCounter(old_s.cmd_input_counter_idx))
        && (Bits(value, XLEN - 1, XLEN - 1) == 0 ==> Bits(value, 11, 0) == CounterCsrNumber(old_s.cmd_input_counter_idx))
        && (Bits(value, XLEN - 1, XLEN - 1) == 0 ==> Bits(value, 17, 12) == CounterBitWidth(old_s.cmd_input_counter_idx) - 1)
    ))
    && (old_s == new_s)
}