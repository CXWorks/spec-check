pub open spec fn sbi_nacl_probe_feature_spec(error: SbiReturnCode, value: UInt64, feature_id: UInt32, old_s: S, new_s: S) -> bool {
    (IsNaclFeatureAvailable(feature_id) ==> ResultEqual(error, SBI_SUCCESS) && value == 1)
    && (!IsNaclFeatureAvailable(feature_id) ==> ResultEqual(error, SBI_SUCCESS) && value == 0)
    && (old_s == new_s)
}