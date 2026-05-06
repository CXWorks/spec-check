```verus
pub open spec fn RMI_FEATURES_spec(s: S, index: u64, result: RmiCommandReturnCode, value: u64) -> bool {
  result == RmiCommandReturnCode::SUCCESS && value == RmiFeatureRegisterEncode(s, index as int)
}
```