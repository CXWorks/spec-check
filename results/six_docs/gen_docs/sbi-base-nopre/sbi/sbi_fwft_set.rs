pub open spec fn sbi_fwft_set_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.code == 0 ==> (new_s.firmware_features == old_s.firmware_features))
    && (result.code == 0 ==> (new_s.firmware_feature_values == old_s.firmware_feature_values))
    && (result.code == 0 ==> (new_s.firmware_feature_flags == old_s.firmware_feature_flags))
    && (result.code == 0 ==> (new_s.firmware_feature_locks == old_s.firmware_feature_locks))
    && (result.code != 0 ==> (result.code == -1))
    && (result.code != 0 ==> (new_s.firmware_features == old_s.firmware_features))
    && (result.code != 0 ==> (new_s.firmware_feature_values == old_s.firmware_feature_values))
    && (result.code != 0 ==> (new_s.firmware_feature_flags == old_s.firmware_feature_flags))
    && (result.code != 0 ==> (new_s.firmware_feature_locks == old_s.firmware_feature_locks))
}