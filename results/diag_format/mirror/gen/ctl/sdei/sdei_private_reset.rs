pub open spec fn sdei_private_reset_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS)
}