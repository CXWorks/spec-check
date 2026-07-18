pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
  && (RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
  && (addr != VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int).rtte.addr).reg_base ==> result == RSI_ERROR_INPUT)
  && (result == RSI_SUCCESS ==> top == VsmmuAt(new_s, RttWalk(new_s, CurrentRealm(new_s), addr,RMM_RTT_PAGE_LEVEL as int).rtte.addr).reg_top)
  && ((AddrIsGranuleAligned(old_s, addr) &&
       AddrIsProtected(old_s, addr, CurrentRealm(old_s)) &&
       !(RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int).rtte.state != ASSIGNED_VSMMU) &&
       !(addr != VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr,RMM_RTT_PAGE_LEVEL as int).rtte.addr).reg_base))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> top == top)
}