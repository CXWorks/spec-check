pub open spec fn base_notify_errors__3_2_2_10_spec(notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS && notify_enable != 0 ==> true)
  && (result == RSI_SUCCESS && notify_enable == 0 ==> true)
  && ((!(result == RSI_SUCCESS)) ==> true)
}