pub open spec fn rmi_version_spec(result: Result<(), RmiStatusCode>, req: RmiInterfaceVersion, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
    ((!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req))
        ==> (ResultEqual(result, RMI_ERROR_INPUT)
            && VersionEqualRmi(lower, RmiVersionHighestBelow(old_s, req))
            && VersionEqualRmi(higher, RmiVersionHighest(old_s))))
    && ((!RmiVersionIsSupported(old_s, req)
            && !RmiVersionLowerIsSupported(old_s, req)
            && RmiVersionHigherIsSupported(old_s, req))
        ==> (ResultEqual(result, RMI_ERROR_INPUT)
            && VersionEqualRmi(lower, higher)
            && VersionEqualRmi(higher, RmiVersionHighest(old_s))))
    && (RmiVersionIsSupported(old_s, req)
        ==> (result.is_Ok()
            && VersionEqualRmi(lower, req)
            && VersionEqualRmi(higher, RmiVersionHighest(old_s))))
    && new_s == old_s
}