pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> lower == req)
  && (result.is_Ok() ==> higher == RmiVersionHighest(new_s))
  && ((!(RmiVersionSupported(new_s, req)) && RmiVersionHighest(new_s) < req) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(RmiVersionSupported(new_s, req)) && !(RmiVersionHighest(new_s) < req)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err() ==> lower == RmiVersionHighest(new_s))
  && (result.is_Err() && RmiVersionHighest(new_s) < req ==> higher == RmiVersionHighest(new_s))
  && ((result.is_Ok()) ==> higher == RmiVersionHighest(new_s))
}