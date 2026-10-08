pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (result == RSI_SUCCESS ==> lower == req)
  && (result == RSI_SUCCESS ==> higher == RsiVersionHighest(new_s))
  && ((!(RsiVersionHigherIsSupported(old_s, req)) && RsiVersionHighestBelow(old_s, req).major < req.major || (RsiVersionHigherIsSupported(old_s, req) && !(RsiVersionIsSupported(old_s, req)))) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_INPUT && RsiVersionHighestBelow(old_s, req).major < req.major ==> lower == RsiVersionHighestBelow(old_s, req))
  && (result == RSI_ERROR_INPUT && !(RsiVersionHigherIsSupported(old_s, req)) && RsiVersionHighestBelow(old_s, req).major >= req.major ==> lower == higher)
  && (result == RSI_ERROR_INPUT && !(RsiVersionHigherIsSupported(old_s, req)) && RsiVersionHighestBelow(old_s, req).major >= req.major ==> higher == RsiVersionHighest(old_s))
  && ((result != RSI_SUCCESS)
    ==> lower == RsiVersionHighest(old_s))
  && ((result != RSI_SUCCESS)
    ==> higher == RsiVersionHighest(old_s))
}