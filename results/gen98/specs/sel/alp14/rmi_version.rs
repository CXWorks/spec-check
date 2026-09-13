pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req)) ==> (ResultEqual(result, RMI_ERROR_INPUT) && VersionEqual(lower, RmiVersionHighestBelow(old_s, req)) && VersionEqual(higher, RmiVersionHighest(old_s)))
  && (!RmiVersionIsSupported(old_s, req) && !RmiVersionLowerIsSupported(old_s, req) && RmiVersionHigherIsSupported(old_s, req)) ==> (ResultEqual(result, RMI_ERROR_INPUT) && VersionEqual(lower, higher) && VersionEqual(higher, RmiVersionHighest(old_s)))
  && (result.is_Ok()) ==> VersionEqual(lower, req)
  && (result.is_Ok()) ==> VersionEqual(higher, RmiVersionHighest(new_s))
  && ((!(RmiVersionIsSupported(old_s, req) && RmiVersionLowerIsSupported(old_s, req))) &&
       (RmiVersionIsSupported(old_s, req) ||
        !(RmiVersionHigherIsSupported(old_s, req))))
    ==> result.is_Ok()
  && (result.is_Err())
    ==> VersionEqual(lower, higher)
}