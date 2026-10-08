pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (RmiVersionIsSupported(old_s, req) ==> ResultEqual(result, RMI_SUCCESS))
  && (!RmiVersionIsSupported(old_s, req) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> lower.major == req.major && lower.minor == req.minor)
  && (result.is_Ok() ==> higher.major == RmiVersionHighest(old_s).major && higher.minor == RmiVersionHighest(old_s).minor)
  && ((!(RmiVersionIsSupported(old_s, req)))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
}