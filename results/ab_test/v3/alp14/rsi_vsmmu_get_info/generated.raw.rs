pub open spec fn rsi_vsmmu_get_info_spec(result: RsiCommandReturnCode, old_s: S, new_s: S, addr: Address, top: Address) -> bool {
    let realm = CurrentRealm();
    let walk = RttWalk(realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    
    (!AddrIsGranuleAligned(addr) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (!AddrIsProtected(addr, realm) ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (walk.rtte.state != ASSIGNED_VSMMU ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (addr != VsmmuAt(walk.rtte.addr).reg_base ==> result.is_Err() && result.get_Err_0() == RSI_ERROR_INPUT)
    && (
        (AddrIsGranuleAligned(addr)
        && AddrIsProtected(addr, realm)
        && walk.rtte.state == ASSIGNED_VSMMU
        && addr == VsmmuAt(walk.rtte.addr).reg_base)
        ==> (result.is_Ok() && top == VsmmuAt(walk.rtte.addr).reg_top)
    )
}