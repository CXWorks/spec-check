pub open spec fn rsi_plane_enter_spec(plane_idx: UInt64, run_ptr: Address, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  ((plane_idx == 0 || plane_idx > CurrentRealm(old_s).num_aux_planes) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsRsiGranuleAligned(old_s, run_ptr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, run_ptr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), run_ptr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), run_ptr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.enter.pstate[3] == '1' ==> result == RSI_ERROR_INPUT)
  && ((!((plane_idx == 0 || plane_idx > CurrentRealm(old_s).num_aux_planes)) &&
       AddrIsRsiGranuleAligned(old_s, run_ptr) &&
       AddrIsProtected(old_s, run_ptr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), run_ptr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_EMPTY) &&
       !(RttWalk(old_s, CurrentRealm(old_s), run_ptr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.enter.pstate[3] == '1'))
    ==> result == RSI_SUCCESS)
}