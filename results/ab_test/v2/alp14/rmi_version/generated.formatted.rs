pub open spec fn rmi_version_spec(
    result: RmiCommandReturnCode,
    lower: RmiInterfaceVersion,
    higher: RmiInterfaceVersion,
    old_s: S,
    new_s: S,
    req: RmiInterfaceVersion,
) -> bool {
    // Failure condition: incompat_lower
    ((!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req)) ==> (
    ResultEqual(result, RMI_ERROR_INPUT) && VersionEqual(lower, RmiVersionHighestBelow(old_s, req))
        && VersionEqual(
        higher,
        RmiVersionHighest(old_s),
    )))
    // Failure condition: incompat_higher
     && ((!RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req)
        && RmiVersionHigherIsSupported(old_s, req)) ==> (ResultEqual(result, RMI_ERROR_INPUT)
        && VersionEqual(lower, higher) && VersionEqual(
        higher,
        RmiVersionHighest(old_s),
    )))
    // Success condition: lower
     && (RmiVersionIsSupported(old_s, req) ==> VersionEqual(
        lower,
        req,
    ))
    // Success condition: higher
     && (RmiVersionIsSupported(old_s, req) ==> VersionEqual(
        higher,
        RmiVersionHighest(old_s),
    ))
    // State unchanged (no footprint)
     && old_s == new_s
}