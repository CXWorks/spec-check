pub open spec fn sbi_nacl_probe_feature_spec(feature_id: UInt32, ret_error: i64, ret_value: UInt64, old_s: S, new_s: S) -> bool {
    (ret_error == SBI_SUCCESS)
    && (!NaclFeatureAvailable(old_s, feature_id) ==> ret_value == 0)
    && (NaclFeatureAvailable(old_s, feature_id) ==> ret_value == 1)
    && (new_s == old_s)
}
