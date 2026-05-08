pub open spec fn RMI_VERSION_spec(
    s: S,
    req: RmiInterfaceVersion,
    result: Result<(), RmiStatusCode>,
    lower: RmiInterfaceVersion,
    higher: RmiInterfaceVersion,
) -> bool {
    let success_lower = VersionEqual(lower, req);
    let success_higher = VersionEqual(higher, RmiVersionHighest(s));

    let incompat_lower_pre = !RmiVersionIsSupported(s, req) && RmiVersionLowerIsSupported(s, req);
    let incompat_lower_post = ResultEqual(result, RMI_ERROR_INPUT) && VersionEqual(
        lower,
        RmiVersionHighestBelow(s, req),
    ) && VersionEqual(higher, RmiVersionHighest(s));

    let incompat_higher_pre = !RmiVersionIsSupported(s, req) && !RmiVersionLowerIsSupported(s, req)
        && RmiVersionHigherIsSupported(s, req);
    let incompat_higher_post = ResultEqual(result, RMI_ERROR_INPUT) && VersionEqual(lower, higher)
        && VersionEqual(higher, RmiVersionHighest(s));

    // Success case: requested version is supported
    (RmiVersionIsSupported(s, req) ==> (result.is_Ok() && success_lower
        && success_higher))
    // Failure case: incompatible lower
     && (incompat_lower_pre
        ==> incompat_lower_post)
    // Failure case: incompatible higher
     && (incompat_higher_pre ==> incompat_higher_post)
}