pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqual(old_s, lower, RsiVersionHighestBelow(old_s, req)) && VersionEqual(old_s, higher, RsiVersionHighest(old_s)))
  && (!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req) && RsiVersionHigherIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqual(old_s, lower, higher) && VersionEqual(old_s, higher, RsiVersionHighest(old_s)))
  && (result == RSI_SUCCESS) ==> VersionEqual(new_s, lower, req)
  && (result == RSI_SUCCESS) ==> VersionEqual(new_s, higher, RsiVersionHighest(new_s))
  && ((!(RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req))) &&
       (RsiVersionIsSupported(old_s, req) || !RsiVersionLowerIsSupported(old_s, req) || !RsiVersionHigherIsSupported(old_s, req)))
    ==> result == RSI_SUCCESS
  && (result != RSI_SUCCESS)
    ==> VersionEqual(new_s, lower, lower)
  && (result != RSI_SUCCESS)
    ==> VersionEqual(new_s, higher, higher)
}