pub open spec fn system_reset2_spec(reset_type: UInt64, cookie: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT && (reset_type != 0 && (reset_type & 0x80000000) == 0) ==> true)
  && (result == RSI_SUCCESS ==> true)
  && ((!(result == RSI_ERROR_INPUT && (reset_type != 0 && (reset_type & 0x80000000) == 0)))
    ==> result == RSI_SUCCESS)
}