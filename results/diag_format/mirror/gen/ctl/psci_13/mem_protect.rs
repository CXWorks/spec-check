pub open spec fn mem_protect_spec(enable: bool, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> result == RSI_SUCCESS)
  && ((!(result == RSI_SUCCESS))
    ==> result == RSI_SUCCESS)
}