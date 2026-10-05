pub open spec fn power_state_change_requested_notify__3_3_2_9_spec(domain_id: UInt32, notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_STATE)
  && (result == RSI_INCOMPLETE)
  && (result == RSI_ERROR_UNKNOWN)
  && ((notify_enable & 1) == 0 ==> result == RSI_ERROR_INPUT)
}