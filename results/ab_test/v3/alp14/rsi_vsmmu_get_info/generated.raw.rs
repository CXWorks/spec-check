pub open spec fn rsi_vsmmu_get_info_spec(result: RsiCommandReturnCode, top: Address, addr: Address, old_s: S, new_s: S) -> bool {
  let realm = old_s;
  let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
  (!AddrIsGranuleAligned(addr) ==> result == RSI_ERROR_INPUT)
  && (!AddrIsProtected(addr, realm) ==> result == RSI_ERROR_INPUT)
  && (walk.rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
  && (addr != VsmmuAt(old_s, walk.rtte.addr).reg_base ==> result == RSI_ERROR_INPUT)
  && (
    (AddrIsGranuleAligned(addr)
      && AddrIsProtected(addr, realm)
      && walk.rtte.state == ASSIGNED_VSMMU
      && addr == VsmmuAt(old_s, walk.rtte.addr).reg_base)
    ==> (result == RSI_SUCCESS && top == VsmmuAt(old_s, walk.rtte.addr).reg_top && new_s == old_s)
  )
}