```verus
pub open spec fn RSI_PLANE_SYSREG_WRITE_spec(
    s: S,
    realm: RmmRealm,
    rec: RmmRec,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
    result: RsiCommandReturnCode
) -> bool {
    (plane_idx > realm.num_aux_planes as u64 ==> result == RSI_ERROR_INPUT) &&
    (!PlaneSysregValid(rec, addr, RMM_WRITE) ==> result == RSI_ERROR_INPUT) &&
    (result == RSI_SUCCESS ==> (
        PlaneSysregValue(rec, plane_idx, addr).low == value_low &&
        (addr.d128 == RSI_TRUE ==> PlaneSysregValue(rec, plane_idx, addr).high == value_high)
    ))
}
```