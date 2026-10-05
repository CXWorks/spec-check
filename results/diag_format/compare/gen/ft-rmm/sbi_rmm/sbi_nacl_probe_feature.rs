pub open spec fn sbi_nacl_probe_feature_spec(feature_id: UInt32, error: SbiReturnCode, value: UInt64, old_s: S, new_s: S) -> bool {
  (ResultEqual(error, SBI_SUCCESS) ==> value == 1)
  && (!IsNaclFeatureAvailable(old_s, feature_id) ==> value == 0)
  && ((!(ResultEqual(error, SBI_SUCCESS))) ==> value == 0)
}