pub open spec fn rsi_plane_sysreg_read_spec(result: RsiCommandReturnCode, value_low: u64, value_high: u64, old_s: S, new_s: S, plane_idx: u64, addr: RsiSysregAddress) -> bool {
    let realm = old_s;
    let rec = old_s;
    (plane_idx > realm.num_aux_planes ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (!PlaneSysregValid(rec, addr, RMM_READ) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && ((addr.d128 == RSI_FALSE && result.is_Ok()) ==> (Zeros(64) :: value_low) == PlaneSysregValue(rec, plane_idx, addr))
    && ((addr.d128 == RSI_TRUE && result.is_Ok()) ==> (value_high :: value_low) == PlaneSysregValue(rec, plane_idx, addr))
}