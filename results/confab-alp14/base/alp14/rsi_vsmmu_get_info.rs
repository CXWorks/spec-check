pub open spec fn rsi_vsmmu_get_info_spec(result: RsiCommandReturnCode, addr: Address, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, addr) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && (addr != VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(old_s, addr)
         && AddrIsProtected(old_s, addr, CurrentRealm(old_s))
         && RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_VSMMU
         && addr == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base)
        ==> (result == RSI_SUCCESS
             && top == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top
             && new_s == old_s))
}