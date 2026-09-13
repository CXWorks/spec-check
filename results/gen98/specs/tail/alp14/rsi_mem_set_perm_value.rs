pub open spec fn rsi_mem_set_perm_value_spec(plane_index: UInt64, perm_index: UInt64, value: Bits64, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((plane_index == 0 || plane_index > CurrentRealm(old_s).num_aux_planes) ==> result == RSI_ERROR_INPUT)
  && (perm_index >= RMM_NUM_PERM_OVERLAY_INDICES ==> result == RSI_ERROR_INPUT)
  && (CurrentRealm(old_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED ==> result == RSI_ERROR_INPUT)
  && (!MemPermLabelSupported(old_s, value) ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> CurrentRealm(new_s).overlay_perms[plane_index as int].values[perm_index as int] == value)
  && ((!((plane_index == 0 || plane_index > CurrentRealm(old_s).num_aux_planes)) &&
       !(perm_index >= RMM_NUM_PERM_OVERLAY_INDICES) &&
       !(CurrentRealm(old_s).overlay_locked[perm_index as int] == MEM_PERM_LOCKED) &&
       MemPermLabelSupported(old_s, value))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> CurrentRealm(new_s).overlay_perms[plane_index as int].values[perm_index as int] == CurrentRealm(old_s).overlay_perms[plane_index as int].values[perm_index as int])
}