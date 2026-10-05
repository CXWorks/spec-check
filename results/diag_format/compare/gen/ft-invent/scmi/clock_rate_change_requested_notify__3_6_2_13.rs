pub open spec fn clock_rate_change_requested_notify__3_6_2_13_spec(clock_id: UInt32, notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_STATE)
  && (result == RSI_INCOMPLETE)
  && (result == RSI_ERROR_UNKNOWN)
  && ((notify_enable & 1) == 0 ==> result == RSI_ERROR_INPUT)
}