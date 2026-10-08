pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> lower == req)
  && (result.is_Ok() ==> higher == RmiVersionHighest(new_s))
  && ((!(RmiVersionIsSupported(old_s, req)) && RmiVersionHighestBelow(old_s, req).major < req.major || (RmiVersionIsSupported(old_s, req) && !(RmiVersionIsSupported(old_s, req)))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() ==> lower == RmiVersionHighestBelow(old_s, req))
  && (result.is_Err() ==> higher == RmiVersionHighest(old_s))
  && (result.is_Err() && (RmiVersionHighestBelow(old_s, req).major >= req.major) ==> lower == higher)
}