pub open spec fn rsi_plane_sysreg_read_spec(plane_idx: UInt64, addr: RsiSysregAddress, result: RsiCommandReturnCode, value_low: Bits64, value_high: Bits64, old_s: S, new_s: S) -> bool {
  (plane_idx > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!PlaneSysregValid(old_s, CurrentRec(old_s), addr, RMM_READ) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> (value_high == PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx as int, addr).high && value_low == PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx as int, addr).low))
  && ((!(plane_idx > CurrentRealm(old_s).num_aux_planes) && PlaneSysregValid(old_s, CurrentRec(old_s), addr, RMM_READ))
    ==> (result == RSI_SUCCESS
      ==> (value_high == PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx as int, addr).high && value_low == PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx as int, addr).low))
    && (result != RSI_SUCCESS
      ==> (value_high == 0 && value_low == 0)))
}