```verus
pub open spec fn rsi_features_spec(result: RsiCommandReturnCode, value: u64, old_s: S, new_s: S, realm: RmmRealm, index: u64) -> bool {
    result == RsiCommandReturnCode::Success &&
    value == RsiFeatureRegisterEncode(old_s, index as int) &&
    new_s == old_s
}
```