pub open spec fn rsi_vsmmu_get_info_spec(
    result: RsiCommandReturnCode,
    top: Address,
    old_s: S,
    new_s: S,
    addr: Address,
) -> bool {
    let realm = CurrentRealm();
    let walk = RttWalk(realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let vsmmu = VsmmuAt(walk.rtte.addr);

    // Failure condition: addr_align
    (!AddrIsGranuleAligned(addr) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: addr_bound
     && (!AddrIsProtected(addr, realm) ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: rtte_state
     && (walk.rtte.state != ASSIGNED_VSMMU ==> result
        == RSI_ERROR_INPUT)
    // Failure condition: vsmmu_base
     && (addr != vsmmu.reg_base ==> result
        == RSI_ERROR_INPUT)
    // Success condition: vsmmu_base
     && ((AddrIsGranuleAligned(addr) && AddrIsProtected(addr, realm) && walk.rtte.state
        == ASSIGNED_VSMMU && addr == vsmmu.reg_base) ==> (result == RSI_SUCCESS && top
        == vsmmu.reg_top))
    // No state changes
     && (new_s == old_s)
}