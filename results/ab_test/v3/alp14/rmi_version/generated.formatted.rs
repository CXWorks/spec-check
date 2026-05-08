pub open spec fn RMI_VERSION_spec(
    s: S,
    req: RmiInterfaceVersion,
    result: Result<(), RmiStatusCode>,
    lower: RmiInterfaceVersion,
    higher: RmiInterfaceVersion,
) -> bool {
    let old_s = s;
    let new_s = s;

    // Failure case 1: incompat_lower
    ((!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req)) ==> (
    ResultEqual(result, RMI_ERROR_INPUT) && VersionEqual(lower, RmiVersionHighestBelow(old_s, req))
        && VersionEqual(
        higher,
        RmiVersionHighest(old_s),
    )))
    // Failure case 2: incompat_higher
     && ((!RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req)
        && RmiVersionHigherIsSupported(old_s, req)) ==> (ResultEqual(result, RMI_ERROR_INPUT)
        && VersionEqual(lower, higher) && VersionEqual(
        higher,
        RmiVersionHighest(old_s),
    )))
    // Success case: compatible version
     && ((RmiVersionIsSupported(old_s, req)) ==> (result.is_Ok() && VersionEqual(lower, req)
        && VersionEqual(higher, RmiVersionHighest(old_s))))
}