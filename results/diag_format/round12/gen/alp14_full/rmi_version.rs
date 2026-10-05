pub open spec fn rmi_version_spec(req: UInt64, lower: UInt64, higher: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) ==> lower <= higher)
  && (result.is_Ok() ==> lower == req)
  && (result.is_Ok() ==> higher == RmiVersionHighestBelow(old_s, req))
  && ((!(result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT)) &&
       result.is_Ok())
    ==> higher == RmiVersionHighestBelow(old_s, req))
  && (result.is_Err()
    ==> RmiVersionHighestBelow(old_s, req) == higher)
}