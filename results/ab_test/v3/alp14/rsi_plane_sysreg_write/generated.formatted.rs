pub open spec fn rsi_plane_sysreg_write_spec(
    result: RsiCommandReturnCode,
    plane_idx: u64,
    addr: RsiSysregAddress,
    value_low: u64,
    value_high: u64,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = CurrentRealm(old_s);
    let rec = CurrentRec(old_s);
    (plane_idx > realm.num_aux_planes ==> result == RSI_ERROR_INPUT) && (!PlaneSysregValid(
        rec,
        addr,
        RMM_WRITE,
    ) ==> result == RSI_ERROR_INPUT) && ((plane_idx <= realm.num_aux_planes && PlaneSysregValid(
        rec,
        addr,
        RMM_WRITE,
    )) ==> (result == RSI_SUCCESS && PlaneSysregValue(rec, plane_idx, addr).low == value_low && (
    addr.d128 == RSI_TRUE ==> PlaneSysregValue(rec, plane_idx, addr).high == value_high)))
}