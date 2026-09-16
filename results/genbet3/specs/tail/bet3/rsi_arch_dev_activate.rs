pub open spec fn rsi_arch_dev_activate_spec(base: Address, dev_type: RsiArchDevType, result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (!AddrIsRsiGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, base, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,falsefalse).rtte.state != RTTE_ARCH_DEV ==> result == RSI_ERROR_DEVICE)
  && (VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,falsefalse).rtte.addr).state == VSMMU_ACTIVE ==> result == RSI_ERROR_DEVICE)
  && (result == RSI_SUCCESS ==> VsmmuAt(new_s, RttWalk(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,falsefalse).rtte.addr).state == VSMMU_ACTIVE)
  && ((AddrIsRsiGranuleAligned(old_s, base) &&
       AddrIsProtected(old_s, base, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,falsefalse).rtte.state != RTTE_ARCH_DEV) &&
       !(VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,falsefalse).rtte.addr).state == VSMMU_ACTIVE))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> VsmmuAt(new_s, RttWalk(new_s, CurrentRealm(new_s), base,RMM_RTT_PAGE_LEVEL as int,falsefalse).rtte.addr).state == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base,RMM_RTT_PAGE_LEVEL as int,falsefalse).rtte.addr).state)
}