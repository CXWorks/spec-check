pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base != addr ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (top == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top)
}