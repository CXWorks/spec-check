pub open spec fn sbi_nacl_probe_feature_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == 0 ==> true)
    && (result == 1 ==> true)
}