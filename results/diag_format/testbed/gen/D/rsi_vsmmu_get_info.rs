pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    (!AddrIsGranuleAligned(old_s, realm, addr) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, realm, addr) ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (walk.state != ASSIGNED_VSMMU ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (addr != walk.rtte.s2ap_indirect.base_index ==> ResultEqual(result, RSI_ERROR_INPUT))
    && (result == RSI_SUCCESS ==> top == walk.rtte.s2ap_indirect.base_index)
}