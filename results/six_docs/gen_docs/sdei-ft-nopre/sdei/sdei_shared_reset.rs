pub open spec fn sdei_shared_reset_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
}