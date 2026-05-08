pub open spec fn RSI_PLANE_SYSREG_WRITE_spec(
    s: S,
    realm: RmmRealm,
    rec: RmmRec,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
) -> (result: RsiCommandReturnCode, s_prime: S)
{
    let s_prime = s;
    
    // Failure condition: idx_bound
    if plane_idx > realm.num_aux_planes {
        (result == RSI_ERROR_INPUT, s_prime)
    }
    // Failure condition: sysreg_valid
    else if !PlaneSysregValid(rec, addr, RMM_WRITE) {
        (result == RSI_ERROR_INPUT, s_prime)
    }
    // Success conditions
    else {
        // Success: value_low written
        let s_prime = s_prime; // Would update PlaneSysregValue(rec, plane_idx, addr)[63:0] == value_low
        
        // Success: value_high (conditional on addr.d128 == RSI_TRUE)
        let s_prime = if addr.d128 == RSI_TRUE {
            s_prime // Would update PlaneSysregValue(rec, plane_idx, addr)[127:64] == value_high
        } else {
            s_prime
        };
        
        (result == RSI_SUCCESS, s_prime)
    }
}
```

Or as a more explicit specification:

```rust
pub open spec fn RSI_PLANE_SYSREG_WRITE_spec(
    s: S,
    realm: RmmRealm,
    rec: RmmRec,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
) -> RsiCommandReturnCode
{
    if plane_idx > realm.num_aux_planes {
        RSI_ERROR_INPUT
    } else if !PlaneSysregValid(rec, addr, RMM_WRITE) {
        RSI_ERROR_INPUT
    } else {
        RSI_SUCCESS
    }
}

pub open spec fn RSI_PLANE_SYSREG_WRITE_ensures(
    s: S,
    realm: RmmRealm,
    rec: RmmRec,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
    result: RsiCommandReturnCode,
) -> bool
{
    if result == RSI_SUCCESS {
        && PlaneSysregValue(rec, plane_idx, addr)[63:0] == value_low
        && (addr.d128 == RSI_TRUE ==> PlaneSysregValue(rec, plane_idx, addr)[127:64] == value_high)
    } else {
        true
    }
}