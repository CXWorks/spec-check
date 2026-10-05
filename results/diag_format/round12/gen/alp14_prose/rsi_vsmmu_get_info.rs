pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
  ((addr % GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && (!IsProtectedIpa(old_s, CurrentRealm(old_s), addr) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
  && (result == RSI_ERROR_INPUT ==> RttWalk(new_s, CurrentRealm(new_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state)
  && (result == RSI_SUCCESS && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED_VSMMU) ==> RttWalk(new_s, CurrentRealm(new_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state)
  && ((!( (addr % GRANULE_SIZE) != 0) &&
       IsProtectedIpa(old_s, CurrentRealm(old_s), addr) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state != ASSIGNED_VSMMU))
    ==> result == RSI_SUCCESS)
}