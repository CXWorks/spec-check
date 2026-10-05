pub open spec fn hci_version_spec(req: UInt64, lower: UInt64, higher: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> lower == req)
  && (result == RSI_SUCCESS ==> higher == HciVersionHighest(old_s))
  && ((!(result == RSI_SUCCESS))
    ==> higher == HciVersionHighest(old_s))
  && (result != RSI_SUCCESS
    ==> higher == HciVersionHighest(old_s))
  && (result == RSI_SUCCESS
    ==> (new_s == old_s))
  && (result != RSI_SUCCESS
    ==> (new_s == old_s))
}