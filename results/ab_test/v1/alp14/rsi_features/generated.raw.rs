# Verus Specification for RSI_FEATURES Command

```verus
pub open spec fn RSI_FEATURES_spec(s: S, realm: RmmRealm, index: u64) -> (result: RsiCommandReturnCode, value: u64) {
    let value = RsiFeatureRegisterEncode(s, realm, index as int);
    (RsiCommandReturnCode::RSI_SUCCESS, value)
}
```

Or, if you want a more explicit version that captures both output values with more detail:

```verus
pub open spec fn RSI_FEATURES_spec(s: S, realm: RmmRealm, index: u64) -> (x0: RsiCommandReturnCode, x1: u64)
    ensures
        x0 == RsiCommandReturnCode::RSI_SUCCESS,
        x1 == RsiFeatureRegisterEncode(s, realm, index as int),
{
    (RsiCommandReturnCode::RSI_SUCCESS, RsiFeatureRegisterEncode(s, realm, index as int))
}
```

**Key points:**
- **Inputs**: `realm` (current realm context), `index` (feature register index as `u64`)
- **Outputs**: 
  - `x0`: Always `RSI_SUCCESS` (no failure conditions)
  - `x1`: Feature register value computed by `RsiFeatureRegisterEncode`
- **No state changes**: The command has no footprint
- **No failure cases**: Documentation explicitly states no failure conditions exist