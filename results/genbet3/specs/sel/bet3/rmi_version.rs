pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(RmiInterfaceVersion, RmiInterfaceVersion), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req)) ==> (result.status == RMI_ERROR_INPUT && VersionEqual(lower, RmiVersionHighestBelow(old_s, req)) && VersionEqual(higher, RmiVersionHighest(old_s)))
  && (!RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req) && RmiVersionHigherIsSupported(old_s, req)) ==> (result.status == RMI_ERROR_INPUT && VersionEqual(lower, higher) && VersionEqual(higher, RmiVersionHighest(old_s)))
  && (result.is_Ok() && RmiVersionIsSupported(old_s, req)) ==> VersionEqual(lower, req)
  && (result.is_Ok() && RmiVersionIsSupported(old_s, req)) ==> VersionEqual(higher, RmiVersionHighest(old_s))
  && ((!(RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req))) &&
       (RmiVersionIsSupported(old_s, req) || !(RmiVersionLowerIsSupported(old_s, req) && RmiVersionHigherIsSupported(old_s, req))))
    ==> result.is_Ok()
}