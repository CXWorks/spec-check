pub open spec fn rsi_plane_sysreg_read_spec(result: RsiCommandReturnCode, plane_idx: u64, addr: RsiSysregAddress, value_low: u64, value_high: u64, old_s: S, new_s: S) -> bool {
    (plane_idx > old_s.current_realm().num_aux_planes ==> result == RSI_ERROR_INPUT)
    && (!PlaneSysregValid(old_s.current_rec(), addr, RMM_READ) ==> result == RSI_ERROR_INPUT)
    && (addr.d128 == RSI_FALSE && plane_idx <= old_s.current_realm().num_aux_planes && PlaneSysregValid(old_s.current_rec(), addr, RMM_READ) ==> result == RSI_SUCCESS && (Zeros(64) :: value_low) == PlaneSysregValue(old_s.current_rec(), plane_idx, addr))
    && (addr.d128 == RSI_TRUE && plane_idx <= old_s.current_realm().num_aux_planes && PlaneSysregValid(old_s.current_rec(), addr, RMM_READ) ==> result == RSI_SUCCESS && (value_high :: value_low) == PlaneSysregValue(old_s.current_rec(), plane_idx, addr))
    && (old_s == new_s)
}