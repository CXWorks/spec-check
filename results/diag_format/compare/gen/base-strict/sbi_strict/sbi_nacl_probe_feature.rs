pub open spec fn sbi_nacl_probe_feature_spec(result: SbiErrorCode, value: UInt, feature_id: UInt32, old_s: S, new_s: S) -> bool {
    (ResultEqual(result, SBI_SUCCESS) && (IsNaclFeatureAvailable(feature_id) ==> value == 1) && (!IsNaclFeatureAvailable(feature_id) ==> value == 0))
    && (old_s == new_s)
}