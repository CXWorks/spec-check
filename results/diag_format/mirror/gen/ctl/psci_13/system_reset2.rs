pub open spec fn system_reset2_spec(reset_type: UInt64, cookie: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INVALID_PARAMETERS && (reset_type != 0))
  && ((!(result == RSI_SUCCESS) &&
       !(result == RSI_ERROR_INVALID_PARAMETERS))
    ==> result == RSI_SUCCESS)
}