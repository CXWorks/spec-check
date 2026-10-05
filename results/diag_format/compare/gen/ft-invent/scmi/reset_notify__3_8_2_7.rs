pub open spec fn reset_notify__3_8_2_7_spec(domain_id: UInt32, notify_enable: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS && (notify_enable & 1) == 1 ==> true)
  && (result == RSI_SUCCESS && (notify_enable & 1) == 0 ==> true)
  && ((!(result == RSI_SUCCESS) || ((notify_enable & 1) == 0 && (notify_enable & !1) == 0))
    ==> true)
}