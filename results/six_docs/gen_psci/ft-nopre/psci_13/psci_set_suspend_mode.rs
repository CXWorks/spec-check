pub open spec fn psci_set_suspend_mode_spec(mode: UInt, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT ==> (mode != 0 && mode != 1))
  && ((!(AllCoresInCorrectState(old_s)) && result == RSI_SUCCESS) ==> result == RSI_DENIED)
  && ((result == RSI_SUCCESS) ==> PsciCurrentCoordinationMode(new_s) == mode)
  && ((result != RSI_SUCCESS) ==> PsciCurrentCoordinationMode(new_s) == PsciCurrentCoordinationMode(old_s))
}