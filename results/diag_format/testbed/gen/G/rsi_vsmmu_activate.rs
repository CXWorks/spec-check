pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
  (AddrIsAligned(old_s, base, RMM_GRANULE_SIZE_ORDER as int) ==> result == RSI_SUCCESS)
  && (AddrIsAligned(old_s, top, RMM_GRANULE_SIZE_ORDER as int) ==> result == RSI_SUCCESS)
  && ((top) <= (base) ==> result == RSI_SUCCESS)
  && (RttWalk_(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS ==> GranulesAllState(new_s, base, new_base, DEV))
  && ((result == RSI_SUCCESS && base == VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base && new_base != VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top) ==> VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVATING)
  && (result == RSI_SUCCESS && new_base == VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top ==> VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVE)
  && ((!(AddrIsAligned(old_s, base, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(AddrIsAligned(old_s, top, RMM_GRANULE_SIZE_ORDER as int)) ||
       ((top) <= (base)) ||
       !(RttWalk_(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU))
    ==> result == RSI_ERROR_INPUT)
  && (result != RSI_SUCCESS
    ==> GranulesAllState(new_s, base, new_base, UNDELEGATED))
  && (result != RSI_SUCCESS
    ==> VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VsmmuAt(old_s, RttWalk_(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
  && (!(result == RSI_SUCCESS && (base == VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base && new_base != VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top)) ==> VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VsmmuAt(old_s, RttWalk_(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
}