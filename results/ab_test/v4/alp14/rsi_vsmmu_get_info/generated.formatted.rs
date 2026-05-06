pub open spec fn RSI_VSMMU_GET_INFO_spec(
    old_s: S,
    new_s: S,
    addr: Address,
    realm: RmmRealm,
    walk: RmmRttWalkResult,
    result: RsiCommandReturnCode,
    top: Address,
) -> bool {
    let addr_align_fail = !AddrIsGranuleAligned(addr);
    let addr_bound_fail = !AddrIsProtected(old_s, addr, realm);
    let rtte_state_fail = walk.rtte.state != ASSIGNED_VSMMU;
    let vsmmu_base_fail = addr != VsmmuAt(old_s, walk.rtte.addr).reg_base;
    let success = AddrIsGranuleAligned(addr) && AddrIsProtected(old_s, addr, realm)
        && walk.rtte.state == ASSIGNED_VSMMU && addr == VsmmuAt(old_s, walk.rtte.addr).reg_base;

    (addr_align_fail ==> result == RSI_ERROR_INPUT) && (addr_bound_fail ==> result
        == RSI_ERROR_INPUT) && (rtte_state_fail ==> result == RSI_ERROR_INPUT) && (vsmmu_base_fail
        ==> result == RSI_ERROR_INPUT) && (success ==> result == RSI_OK && top == VsmmuAt(
        old_s,
        walk.rtte.addr,
    ).reg_top) && old_s == new_s
}