pub open spec fn rmi_features_spec(index: UInt64, result: Result<(), RmiStatusCode>, value: Bits64, old_s: S, new_s: S) -> bool {
  (result.is_Ok() && index != 0 ==> value == 0)
  && ((!(result.is_Ok()))
    ==> value == 0)
}