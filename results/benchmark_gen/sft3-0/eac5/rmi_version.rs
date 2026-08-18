pub open spec fn rmi_version_spec(req: RmiInterfaceVersion, result: Result<(), RmiStatusCode>, lower: RmiInterfaceVersion, higher: RmiInterfaceVersion, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> lower == req)
  && (result.is_Ok() ==> higher == RmiInterfaceVersionHighestSupported(new_s))
  && (result.is_Err()
    ==> lower == RmiInterfaceVersionHighestSupported(new_s))
  && (result.is_Err()
    ==> higher == RmiInterfaceVersionHighestSupported(new_s))
}