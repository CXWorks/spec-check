pub open spec fn rmi_version_spec(
    req: RmiInterfaceVersion,
    result: Result<(), RmiStatusCode>,
    lower: RmiInterfaceVersion,
    higher: RmiInterfaceVersion,
    old_s: S,
    new_s: S,
) -> bool {
    ((!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req)) ==> (
    ResultEqual(result, RMI_ERROR_INPUT) && VersionEqualRmi(
        lower,
        RmiVersionHighestBelow(new_s, req),
    ) && VersionEqualRmi(higher, RmiVersionHighest(new_s)))) && ((!RmiVersionIsSupported(old_s, req)
        && !RmiVersionLowerIsSupported(old_s, req) && RmiVersionHigherIsSupported(old_s, req)) ==> (
    ResultEqual(result, RMI_ERROR_INPUT) && VersionEqualRmi(lower, higher) && VersionEqualRmi(
        higher,
        RmiVersionHighest(new_s),
    ))) && (result.is_Ok() ==> VersionEqualRmi(lower, req)) && (result.is_Ok() ==> VersionEqualRmi(
        higher,
        RmiVersionHighest(new_s),
    )) && ((!((!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req))) && !((
    !RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req)
        && RmiVersionHigherIsSupported(old_s, req)))) ==> result.is_Ok())
}