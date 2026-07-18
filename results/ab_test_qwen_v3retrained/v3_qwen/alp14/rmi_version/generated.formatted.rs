pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: RmiCommandReturnCode, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (!RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req)) ==> (result == RMI_ERROR_INPUT && VersionEqual(lower, RmiVersionHighestBelow(old_s, req)) && VersionEqual(higher, RmiVersionHighest(old_s)))
  && ((RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req)) ==> (result == RMI_SUCCESS))
  && (result == RMI_ERROR_INPUT ==> lower == higher)
  && (result == RMI_ERROR_INPUT ==> VersionEqual(higher, RmiVersionHighest(old_s)))
  && (result == RMI_SUCCESS ==> VersionEqual(lower, req))
  && (result == RMI_SUCCESS ==> VersionEqual(higher, RmiVersionHighest(old_s)))
  && (result != RMI_SUCCESS ==> VersionEqual(lower, RmiVersionHighestBelow(old_s, req)))
}