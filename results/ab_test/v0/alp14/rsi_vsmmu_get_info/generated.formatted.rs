pub open spec fn RSI_VSMMU_GET_INFO_spec(
    s: S,
    realm: RmmRealm,
    addr: Address,
    result: RsiCommandReturnCode,
    top: Address,
) -> bool {
    let walk = RttWalk(s, realm, addr, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);

    // Failure conditions
    let addr_align_fails = !AddrIsGranuleAligned(addr) && result == RSI_ERROR_INPUT;
    let addr_bound_fails = !AddrIsProtected(addr, realm) && result == RSI_ERROR_INPUT;
    let rtte_state_fails = walk.rtte.state != ASSIGNED_VSMMU && result == RSI_ERROR_INPUT;
    let vsmmu_base_fails = addr != VsmmuAt(walk.rtte.addr).reg_base && result == RSI_ERROR_INPUT;

    // Success condition
    let vsmmu_base_succeeds = addr == VsmmuAt(walk.rtte.addr).reg_base && AddrIsGranuleAligned(addr)
        && AddrIsProtected(addr, realm) && walk.rtte.state == ASSIGNED_VSMMU && top == VsmmuAt(
        walk.rtte.addr,
    ).reg_top && result == RSI_SUCCESS;

    // Either a failure condition is met or the success condition is met
    addr_align_fails || addr_bound_fails || rtte_state_fails || vsmmu_base_fails
        || vsmmu_base_succeeds
}