pub open spec fn rsi_plane_reg_read_spec(plane_idx: UInt64, encoding: Bits64, result: RsiCommandReturnCode, value: Bits64, old_s: S, new_s: S) -> bool {
  (plane_idx > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> value == PlaneRegValue(new_s, CurrentRealm(new_s), plane_idx as int, encoding))
  && ((!(plane_idx > CurrentRealm(old_s).num_aux_planes) &&
       PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding))
    ==> result == RSI_SUCCESS)
}
