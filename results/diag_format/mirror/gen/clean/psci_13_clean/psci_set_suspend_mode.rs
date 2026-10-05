pub open spec fn psci_set_suspend_mode_spec(mode: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INVALID_PARAMETERS ==> mode != 0 && mode != 1)
  && (result == RSI_SUCCESS ==> true)
}