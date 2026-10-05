pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqual(new_s, lower, RsiVersionHighestBelow(new_s, req)) && VersionEqual(new_s, higher, RsiVersionHighest(new_s)))
  && (!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req) && RsiVersionHigherIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqual(new_s, lower, higher) && VersionEqual(new_s, higher, RsiVersionHighest(new_s)))
  && (result == RSI_SUCCESS) ==> VersionEqual(new_s, lower, req)
  && (result == RSI_SUCCESS) ==> VersionEqual(new_s, higher, RsiVersionHighest(new_s))
  && ((!(RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req))) &&
       (RsiVersionIsSupported(old_s, req) || !RsiVersionLowerIsSupported(old_s, req) || !RsiVersionHigherIsSupported(old_s, req)))
    ==> result == RSI_SUCCESS
  && result != RSI_SUCCESS
    ==> VersionEqual(new_s, lower, RsiVersionHighestBelow(new_s, req))
  && result != RSI_SUCCESS
    ==> VersionEqual(new_s, higher, RsiVersionHighest(new_s))
  && result != RSI_SUCCESS
    ==> VersionEqual(new_s, lower, higher)
  && result != RSI_SUCCESS
    ==> VersionEqual(new_s, higher, RsiVersionHighest(new_s))
  && (result == RSI_SUCCESS)
    ==> result == RSI_SUCCESS
}