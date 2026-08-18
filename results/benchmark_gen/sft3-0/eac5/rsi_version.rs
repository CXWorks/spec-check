pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> lower == req)
  && (result == RSI_SUCCESS ==> higher == RsiInterfaceVersion::MAX)
  && ((!(result == RSI_SUCCESS) &&
       !(result == RSI_SUCCESS))
    ==> lower == higher)
}