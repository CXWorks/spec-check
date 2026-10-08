pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
    (ResultEqual(result, RMI_ERROR_INPUT) ==> (RmiVersionHighestBelow(old_s, req) == lower))
    && (ResultEqual(result, RMI_ERROR_INPUT) ==> (RmiVersionHighest(old_s) == higher))
    && (result.is_Ok() ==> (lower == req))
    && (result.is_Ok() ==> (higher == RmiVersionHighest(old_s)))
    && (lower.major == higher.major)
    && (lower.minor <= higher.minor)
}