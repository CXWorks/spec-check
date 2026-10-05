pub open spec fn reset__3_8_2_6_spec(domain_id: UInt32, flags: UInt32, reset_state: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_ERROR_NOT_FOUND)
  && (result == RSI_ERROR_INVALID_PARAMETERS)
  && (result == RSI_ERROR_GENERIC_ERROR)
  && (result == RSI_ERROR_DENIED)
  && ((!(result == RSI_SUCCESS) &&
       !(result == RSI_ERROR_NOT_FOUND) &&
       !(result == RSI_ERROR_INVALID_PARAMETERS) &&
       !(result == RSI_ERROR_GENERIC_ERROR) &&
       !(result == RSI_ERROR_DENIED))
    ==> result == RSI_SUCCESS)
}