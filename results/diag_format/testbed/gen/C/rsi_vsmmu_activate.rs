pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, base) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (top <= base ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (RttWalk_(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> ResultEqual(result, RSI_ERROR_INPUT))
  && (result == RSI_SUCCESS ==> result == RSI_SUCCESS)
  && (result == RSI_SUCCESS ==> forall (a: Address), AddrIsWithin(a, base, new_base) ==> RttWalk_(new_s, CurrentRealm(new_s), a,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DEV)
  && (result == RSI_SUCCESS && (base == VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base && new_base != VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top) ==> VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVATING)
  && (result == RSI_SUCCESS && new_base == VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top ==> VsmmuAt(new_s, RttWalk_(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVE)
  && ((AddrIsGranuleAligned(old_s, base) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(top <= base) &&
       AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) &&
       !(RttWalk_(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU))
    ==> result == RSI_SUCCESS)
}