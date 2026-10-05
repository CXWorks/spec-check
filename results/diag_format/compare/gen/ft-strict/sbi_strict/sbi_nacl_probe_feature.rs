pub open spec fn sbi_nacl_probe_feature_spec(feature_id: UInt32, result: SbiErrorCode, value: UInt, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS ==> IsNaclFeatureAvailable(old_s, feature_id) ==> value == 1)
  && (result == SBI_SUCCESS ==> !IsNaclFeatureAvailable(old_s, feature_id) ==> value == 0)
  && ((!(result == SBI_SUCCESS)) ==> value == 0)
}