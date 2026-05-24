pub open spec fn rsi_plane_sysreg_write_spec(result: RsiCommandReturnCode, plane_idx: u64, addr: RsiSysregAddress, value_low: u64, value_high: u64, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm();
    let rec = CurrentRec();
    
    (plane_idx > realm.num_aux_planes ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (!PlaneSysregValid(rec, addr, RMM_WRITE) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (plane_idx <= realm.num_aux_planes && PlaneSysregValid(rec, addr, RMM_WRITE) ==>
        (result.is_Ok()
        && PlaneSysregValue(rec, plane_idx, addr)[63:0] == value_low
        && (addr.d128 == RSI_TRUE ==> PlaneSysregValue(rec, plane_idx, addr)[127:64] == value_high)))
}