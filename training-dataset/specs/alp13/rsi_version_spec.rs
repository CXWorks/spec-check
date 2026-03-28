pub open spec fn rsi_version_spec(req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion, old_s: S, new_s: S) -> bool {
  ((!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqualRsi(lower, RsiVersionHighestBelow(new_s, req)) && VersionEqualRsi(higher, RsiVersionHighest(new_s))))
  && ((!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req) && RsiVersionHigherIsSupported(old_s, req)) ==> (result == RSI_ERROR_INPUT && VersionEqualRsi(lower, higher) && VersionEqualRsi(higher, RsiVersionHighest(new_s))))
  && (result == RSI_SUCCESS ==> VersionEqualRsi(lower, req))
  && (result == RSI_SUCCESS ==> VersionEqualRsi(higher, RsiVersionHighest(new_s)))
  && ((!((!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req))) &&
       !((!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req) && RsiVersionHigherIsSupported(old_s, req))))
    ==> result == RSI_SUCCESS)
}
