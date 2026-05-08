pub open spec fn RMI_VERSION_spec(
    s: S,
    req: RmiInterfaceVersion,
    result: Result<(), RmiStatusCode>,
    lower: RmiInterfaceVersion,
    higher: RmiInterfaceVersion,
) -> bool {
    (
    // Success condition: requested version is supported
    (RmiVersionIsSupported(s, req) && result.is_Ok() && lower == req && higher == RmiVersionHighest(
        s,
    ))) || (
    // Failure condition: incompat_lower
    (!RmiVersionIsSupported(s, req) && RmiVersionLowerIsSupported(s, req) && ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_INPUT,
    ) && lower == RmiVersionHighestBelow(s, req) && higher == RmiVersionHighest(s))) || (
    // Failure condition: incompat_higher
    (!RmiVersionIsSupported(s, req) && !RmiVersionLowerIsSupported(s, req)
        && RmiVersionHigherIsSupported(s, req) && ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_INPUT,
    ) && lower == higher && higher == RmiVersionHighest(s)))
}