pub open spec fn rsi_plane_sysreg_write_spec(plane_idx: UInt64, addr: RsiSysregAddress, value_low: Bits64, value_high: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (plane_idx > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!PlaneSysregValid(old_s, CurrentRec(old_s), addr, RMM_WRITE) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS && addr.d128 == RSI_TRUE ==> PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx, addr)[127:64] == value_high)
  && (result == RSI_SUCCESS && addr.d128 == RSI_FALSE ==> PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx, addr)[63:0] == value_low)
  && ((!(plane_idx > CurrentRealm(old_s).num_aux_planes) &&
       PlaneSysregValid(old_s, CurrentRec(old_s), addr, RMM_WRITE))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx, addr)[63:0] == PlaneSysregValue(old_s, CurrentRec(old_s), plane_idx, addr)[63:0])
  && (result != RSI_SUCCESS
    ==> PlaneSysregValue(new_s, CurrentRec(new_s), plane_idx, addr)[127:64] == PlaneSysregValue(old_s, CurrentRec(old_s), plane_idx, addr)[127:64])
}