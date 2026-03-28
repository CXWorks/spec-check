pub open spec fn rsi_plane_reg_write_spec(plane_idx: UInt64, encoding: Bits64, value: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (plane_idx > CurrentRealm(old_s).num_aux_planes ==> result == RSI_ERROR_INPUT)
  && (!PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> PlaneRegValue(new_s, CurrentRealm(new_s), plane_idx as int, encoding) == value)
  && ((!(plane_idx > CurrentRealm(old_s).num_aux_planes) &&
       PlaneRegIsValid(old_s, CurrentRealm(old_s), encoding))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> PlaneRegValue(new_s, CurrentRealm(new_s), plane_idx as int, encoding) == PlaneRegValue(old_s, CurrentRealm(old_s), plane_idx as int, encoding))
}
