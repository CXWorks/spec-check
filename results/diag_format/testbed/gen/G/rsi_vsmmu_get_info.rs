pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
  (AddrIsAligned(old_s, addr, RMM_GRANULE_SIZE_ORDER as int) ==> result == RSI_SUCCESS)
  && (AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_SUCCESS)
  && (RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU ==> result == RSI_SUCCESS)
  && (addr == RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr ==> result == RSI_SUCCESS)
  && ((!(AddrIsAligned(old_s, addr, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(AddrIsProtected(old_s, addr, CurrentRealm(old_s))))
    ==> result == RSI_ERROR_INPUT)
  && (RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU
    ==> result == RSI_ERROR_INPUT)
  && (RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr != addr
    ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS
    ==> top == RttWalk_(new_s, CurrentRealm(new_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && ((AddrIsAligned(old_s, addr, RMM_GRANULE_SIZE_ORDER as int) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU) &&
       RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU &&
       RttWalk_(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == addr)
    ==> result == RSI_SUCCESS)
}