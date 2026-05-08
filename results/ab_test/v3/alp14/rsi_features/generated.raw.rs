```verus
pub open spec fn RSI_FEATURES_spec(old_s: S, new_s: S, index: u64, result: RsiCommandReturnCode, value: u64) -> bool {
    result == RsiCommandReturnCode::Success() ==> value == RsiFeatureRegisterEncode(old_s, index as int)
}
```