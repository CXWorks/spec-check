pub open spec fn pinctrl_request__3_11_2_9_spec(identifier: UInt32, flags: UInt32, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_NOT_FOUND)
  && (result == RSI_INVALID_PARAMETERS)
  && (result == RSI_DENIED)
  && (result == RSI_IN_USE)
  && ((!(flags & 3) == 0) ==> result == RSI_INVALID_PARAMETERS)
}