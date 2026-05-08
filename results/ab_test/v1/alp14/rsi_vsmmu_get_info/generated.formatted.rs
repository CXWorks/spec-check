pub open spec fn RSI_VSMMU_GET_INFO_spec(s: S, addr: Address) -> (result: RsiCommandReturnCode, top: Address) {
    let realm = CurrentRealm();
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    if !AddrIsGranuleAligned(addr) {
        (RSI_ERROR_INPUT, 0int as Address)
    } else if !AddrIsProtected(addr, realm) {
        (RSI_ERROR_INPUT, 0int as Address)
    } else if walk.rtte.state != ASSIGNED_VSMMU {
        (RSI_ERROR_INPUT, 0int as Address)
    } else if addr != VsmmuAt(s, walk.rtte.addr).reg_base {
        (RSI_ERROR_INPUT, 0int as Address)
    } else {
        (RSI_SUCCESS, VsmmuAt(s, walk.rtte.addr).reg_top)
    }
}