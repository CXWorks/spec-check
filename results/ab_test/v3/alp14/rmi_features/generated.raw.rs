```verus
pub open spec fn RMI_FEATURES_spec(old_s: S, new_s: S, index: u64, result: RmiCommandReturnCode, value: u64) -> bool {
    result.is_Success() ==> value == RmiFeatureRegisterEncode(old_s, index as int)
}
```