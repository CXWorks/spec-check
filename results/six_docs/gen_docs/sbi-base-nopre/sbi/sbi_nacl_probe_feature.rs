pub open spec fn sbi_nacl_probe_feature_spec(result: i32, old_s: S, new_s: S) -> bool {
    (result == 0 ==> !FeatureAvailable(old_s, feature_id))
    && (result == 1 ==> FeatureAvailable(old_s, feature_id))
    && (result != 0 ==> result != 1 ==> true)
    && (result == 0 ==> new_s == old_s)
    && (result == 1 ==> new_s == old_s)
}