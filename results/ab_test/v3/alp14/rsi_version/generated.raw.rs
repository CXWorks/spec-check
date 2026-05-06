```verus
pub open spec fn RSI_VERSION_spec(
    old_s: S,
    new_s: S,
    req: RsiInterfaceVersion,
    result: RsiCommandReturnCode,
    lower: RsiInterfaceVersion,
    higher: RsiInterfaceVersion,
) -> bool {
    // Failure condition: incompat_lower
    ((!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req))
        ==> (result == RsiCommandReturnCode::RSI_ERROR_INPUT()
            && VersionEqual(lower, RsiVersionHighestBelow(old_s, req))
            && VersionEqual(higher, RsiVersionHighest(old_s))))
    
    // Failure condition: incompat_higher
    && ((!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req)
            && RsiVersionHigherIsSupported(old_s, req))
        ==> (result == RsiCommandReturnCode::RSI_ERROR_INPUT()
            && VersionEqual(lower, higher)
            && VersionEqual(higher, RsiVersionHighest(old_s))))
    
    // Success condition: supported version
    && ((RsiVersionIsSupported(old_s, req))
        ==> (result == RsiCommandReturnCode::RSI_SUCCESS()
            && VersionEqual(lower, req)
            && VersionEqual(higher, RsiVersionHighest(old_s))))
    
    // Success condition: higher and higher output
    && (VersionEqual(higher, RsiVersionHighest(old_s)))
    
    // State unchanged
    && (old_s == new_s)
}
```