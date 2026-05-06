pub open spec fn RSI_PLANE_SYSREG_WRITE_spec(
    old_s: S,
    new_s: S,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
    result: RsiCommandReturnCode
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    
    (plane_idx > realm.num_aux_planes ==> result == RSI_ERROR_INPUT) &&
    (!PlaneSysregValid(rec, addr, RMM_WRITE) ==> result == RSI_ERROR_INPUT) &&
    (
        (plane_idx <= realm.num_aux_planes &&
         PlaneSysregValid(rec, addr, RMM_WRITE)) ==>
        (
            PlaneSysregValue(rec, plane_idx, addr)[63:0] == value_low &&
            (addr.d128 == RSI_TRUE ==> PlaneSysregValue(rec, plane_idx, addr)[127:64] == value_high) &&
            result == RSI_SUCCESS &&
            new_s == old_s
        )
    )
}