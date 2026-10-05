pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
  ((base % GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && ((top % GRANULE_SIZE) != 0 ==> result == RSI_ERROR_INPUT)
  && (top <= base ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas[0 as int] != RIPAS_PROTECTED ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> (RttWalk(new_s, CurrentRealm(new_s),new_base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas[0 as int] == RIPAS_DEV))
  && (result == RSI_SUCCESS && base == VsmmuAt(RttWalk(new_s, CurrentRealm(new_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).register_region_base && new_base != VsmmuAt(RttWalk(new_s, CurrentRealm(new_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).register_region_top ==> VsmmuAt(RttWalk(new_s, CurrentRealm(new_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVATING)
  && (result == RSI_SUCCESS && new_base == VsmmuAt(RttWalk(new_s, CurrentRealm(new_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).register_region_top ==> VsmmuAt(RttWalk(new_s, CurrentRealm(new_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVE)
  && ((!( (base % GRANULE_SIZE) != 0) &&
       !( (top % GRANULE_SIZE) != 0) &&
       !(top <= base) &&
       !(RttWalk(old_s, CurrentRealm(old_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas[0 as int] != RIPAS_PROTECTED) &&
       !(RttWalk(old_s, CurrentRealm(old_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> RttWalk(new_s, CurrentRealm(new_s),new_base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas[0 as int] == RttWalk(old_s, CurrentRealm(old_s),new_base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas[0 as int])
  && (result != RSI_SUCCESS
    ==> VsmmuAt(RttWalk(new_s, CurrentRealm(new_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VsmmuAt(RttWalk(old_s, CurrentRealm(old_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
  && (result != RSI_SUCCESS
    ==> VsmmuAt(RttWalk(new_s, CurrentRealm(new_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VsmmuAt(RttWalk(old_s, CurrentRealm(old_s),base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr).state)
}