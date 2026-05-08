pub open spec fn RSI_VERSION_spec(old_s: S, new_s: S, req: RsiInterfaceVersion, result: RsiCommandReturnCode, lower: RsiInterfaceVersion, higher: RsiInterfaceVersion) -> bool {
    ((!RsiVersionIsSupported(old_s, req) && RsiVersionLowerIsSupported(old_s, req)) ==> 
        (result == RSI_ERROR_INPUT && VersionEqual(lower, RsiVersionHighestBelow(old_s, req)) && VersionEqual(higher, RsiVersionHighest(old_s)))) &&
    
    ((!RsiVersionIsSupported(old_s, req) && !RsiVersionLowerIsSupported(old_s, req) && RsiVersionHigherIsSupported(old_s, req)) ==>
        (result == RSI_ERROR_INPUT && VersionEqual(lower, higher) && VersionEqual(higher, RsiVersionHighest(old_s)))) &&
    
    (RsiVersionIsSupported(old_s, req) ==>
        (result == RSI_SUCCESS && VersionEqual(lower, req) && VersionEqual(higher, RsiVersionHighest(old_s)))) &&
    
    old_s == new_s
}