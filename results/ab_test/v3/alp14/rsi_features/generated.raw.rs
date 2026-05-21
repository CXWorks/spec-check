pub open spec fn rsi_features_spec(result: RsiCommandReturnCode, value: u64, realm: RmmRealm, index: u64, old_s: S, new_s: S) -> bool {
    (result == RsiCommandReturnCode::Success)
    && (value == RsiFeatureRegisterEncode(old_s, index as int))
    && (old_s == new_s)
}