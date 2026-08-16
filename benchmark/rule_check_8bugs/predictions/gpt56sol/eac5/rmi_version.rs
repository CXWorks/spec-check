pub open spec fn rmi_version_spec(
    req: RmiInterfaceVersion,
    result: Result<(), RmiStatusCode>,
    lower: RmiInterfaceVersion,
    higher: RmiInterfaceVersion,
    old_s: S,
    new_s: S,
) -> bool {
    true
}