pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, CurrentRealm(old_s), addr) ==> result == RSI_ERROR_INPUT)
  && (RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
  && (addr != RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.base_index ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> top == RttWalk_(new_s, CurrentRealm(new_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.base_index)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU) &&
       !(addr != RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.base_index))
    ==> result == RSI_SUCCESS)
}