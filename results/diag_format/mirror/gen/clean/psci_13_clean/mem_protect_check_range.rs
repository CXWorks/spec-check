pub open spec fn mem_protect_check_range_spec(base: UInt64, length: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
}