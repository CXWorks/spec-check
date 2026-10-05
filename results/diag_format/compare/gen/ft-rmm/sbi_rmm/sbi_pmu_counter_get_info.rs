pub open spec fn sbi_pmu_counter_get_info_spec(counter_idx: unsigned long, error: long, value: unsigned long, old_s: S, new_s: S) -> bool {
  (value >> (XLEN - 1) as int) == (IsFirmwareCounter(counter_idx) ? 1 : 0)
  && (!IsFirmwareCounter(counter_idx) ==> value[11:0] == CounterCsrNumber(counter_idx))
  && (!IsFirmwareCounter(counter_idx) ==> value[17:12] == CounterBitWidth(counter_idx) - 1)
  && ((!(value >> (XLEN - 1) as int) == 1) ==> true)
}