```verus
pub open spec fn RSI_VERSION_spec(s: S, realm: RmmRealm, req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion) -> bool {
  let req_supported = RsiVersionIsSupported(s, req);
  let lower_supported = RsiVersionLowerIsSupported(s, req);
  let higher_supported = RsiVersionHigherIsSupported(s, req);
  let highest = RsiVersionHighest(s);
  let highest_below = RsiVersionHighestBelow(s, req);
  
  if req_supported {
    VersionEqual(result, req) && VersionEqual(lower, req) && VersionEqual(higher, highest)
  } else if lower_supported {
    result == RSI_ERROR_INPUT && VersionEqual(lower, highest_below) && VersionEqual(higher, highest)
  } else if higher_supported {
    result == RSI_ERROR_INPUT && VersionEqual(lower, highest) && VersionEqual(higher, highest)
  } else {
    false
  }
}
```