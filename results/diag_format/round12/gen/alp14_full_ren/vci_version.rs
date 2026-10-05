pub open spec fn vci_version_spec(req: UInt64, lower: UInt64, higher: UInt64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> lower == req)
  && (result == RSI_SUCCESS ==> higher == VciVersionHighest(old_s))
  && ((!(result == RSI_SUCCESS))
    ==> lower == VciVersionHighestBelow(old_s, req))
  && ((!(result == RSI_SUCCESS))
    ==> higher == VciVersionHighest(old_s))
  && (result != RSI_SUCCESS
    ==> lower == VciVersionHighest(old_s))
  && (VciVersionHighest(old_s) < req
    ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS
    ==> VciVersionHighest(old_s) >= req)
  && ((!(result == RSI_SUCCESS))
    ==> VciVersionHighest(old_s) < req)
  && (result == RSI_SUCCESS
    ==> VciVersionHighest(old_s) >= req)
}