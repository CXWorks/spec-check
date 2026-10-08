pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (result == RSI_ERROR_INPUT ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> lower.major == req.major && lower.minor == req.minor)
  && (result == RSI_SUCCESS ==> higher.major == RsiVersionHighest(new_s).major && higher.minor == RsiVersionHighest(new_s).minor)
  && ((!(result == RSI_ERROR_INPUT))
    ==> result == RSI_SUCCESS)
}