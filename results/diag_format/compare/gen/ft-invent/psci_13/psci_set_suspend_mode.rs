pub open spec fn psci_set_suspend_mode_spec(mode: UInt, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT ==> mode != 0 && mode != 1)
  && (result == RSI_SUCCESS ==> true)
  && ((!(AllCoresInCorrectState(old_s)) || (result == RSI_SUCCESS)) ==> result == RSI_SUCCESS)
  && ((!(AllCoresInCorrectState(old_s)) || (result == RSI_SUCCESS)) ==> result != RSI_SUCCESS)
  && (result != RSI_SUCCESS && result != RSI_SUCCESS ==> result == RSI_SUCCESS)
}