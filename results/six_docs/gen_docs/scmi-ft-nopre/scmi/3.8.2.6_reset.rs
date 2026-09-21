pub open spec fn 3.8.2.6_reset_spec(domain_id: UInt32, flags: UInt32, reset_state: UInt32, result: Result<(), RsiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
  && (result == RSI_ERROR_NOT_FOUND)
  && (result == RSI_ERROR_INVALID_PARAMETERS)
  && (result == RSI_ERROR_GENERIC_ERROR)
  && (result == RSI_ERROR_DENIED)
  && ((!(flags & 0x7) == 0) ==> result == RSI_ERROR_INVALID_PARAMETERS)
  && ((flags & 0x4) != 0 && (flags & 0x1) == 0 ==> result == RSI_ERROR_INVALID_PARAMETERS)
}