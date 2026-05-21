pub open spec fn rsi_vsmmu_activate_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, base: Address, top: Address, new_base: Address) -> bool {
    (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT)
    && (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && (
      (AddrIsGranuleAligned(base) && AddrIsGranuleAligned(top) && UInt(top) > UInt(base) && AddrRangeIsProtected(base, top, CurrentRealm(old_s))
       && RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_VSMMU)
      ==> (
        result == RSI_SUCCESS
        && (base == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).reg_base && new_base != VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).reg_top
          ==> VsmmuAt(new_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).state == VSMMU_ACTIVATING)
        && (new_base == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).reg_top
          ==> VsmmuAt(new_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).state == VSMMU_ACTIVE)
      )
    )
}