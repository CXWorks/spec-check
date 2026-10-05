pub open spec fn sbi_nacl_probe_feature_spec(result: u64, old_s: S, new_s: S) -> bool {
    (result == 0) || (result == 1)
}