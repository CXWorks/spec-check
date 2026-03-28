pub open spec fn rsi_plane_sysreg_write_spec(plane_idx: UInt64, addr: RsiSysregAddress, value_low: Bits64, value_high: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (plane_idx > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!RecSysregValid(old_s, CurrentRec(old_s), addr, true) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> RecSysregValue(new_s, CurrentRec(new_s), plane_idx as int, addr).0)
  && (result == RSI_SUCCESS ==>  == value_low)
  && (result == RSI_SUCCESS && addr.d128 == RSI_TRUE ==> RecSysregValue(new_s, CurrentRec(new_s), plane_idx as int, addr).1)
  && (result == RSI_SUCCESS ==>  == value_high)
  && ((!(plane_idx > CurrentRealm(old_s).num_aux_planes) &&
       RecSysregValid(old_s, CurrentRec(old_s), addr, true))
    ==> result == RSI_SUCCESS)
}
