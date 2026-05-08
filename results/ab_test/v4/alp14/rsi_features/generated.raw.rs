pub open spec fn RSI_FEATURES_spec(old_s: S, new_s: S, realm: RmmRealm, index: u64, result: RsiCommandReturnCode, value: u64) -> bool {
    result == RsiCommandReturnCode::Success() &&
    value == RsiFeatureRegisterEncode(old_s, realm, index as int)
}