pub open spec fn rmi_features_spec(index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Ok() ==> new_s.value == RmiFeatureRegisterEncode(index as int))
  && (!result.is_Ok() ==> new_s.value == new_s.value)
}