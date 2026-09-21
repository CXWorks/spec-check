pub open spec fn 3.6.2.8_clock_rate_get_spec(clock_id: UInt32, result: RsiCommandReturnCode, rate: [UInt32; 2], old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> rate[0] == 0)
  && (result == RSI_SUCCESS ==> rate[1] == 0)
}