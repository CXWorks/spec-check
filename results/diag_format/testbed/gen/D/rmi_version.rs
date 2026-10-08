pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
    (!VersionEqualRmi(req, higher) && req > lower ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!VersionEqualRmi(req, higher) && req < higher ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ResultEqual(result, RMI_SUCCESS) ==> lower == req)
    && (ResultEqual(result, RMI_SUCCESS) ==> higher == RmiVersionHighest(old_s))
    && (ResultEqual(result, RMI_ERROR_INPUT) ==> lower == RmiVersionHighestBelow(old_s, req))
    && (ResultEqual(result, RMI_ERROR_INPUT) ==> higher == RmiVersionHighest(old_s))
}