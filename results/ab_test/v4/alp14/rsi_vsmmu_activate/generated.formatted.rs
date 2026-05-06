pub open spec fn RSI_VSMMU_ACTIVATE_spec(
    old_s: S,
    new_s: S,
    base: Address,
    top: Address,
    result: RsiCommandReturnCode,
    new_base: Address,
) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let vsmmu = VsmmuAt(old_s, walk.rtte.addr);

    ((!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT) && (!AddrIsGranuleAligned(
        old_s,
        top,
    ) ==> result == RSI_ERROR_INPUT) && (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT) && (
    !AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT) && (
    walk.rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)) || (result == RSI_SUCCESS
        && RttEntriesInRangeRipas(
        new_s,
        RttAt(new_s, walk.rtte.addr),
        RMM_RTT_PAGE_LEVEL as int,
        base,
        new_base,
        DEV as RmmRipas,
    ) && ((base == vsmmu.reg_base && new_base != vsmmu.reg_top) ==> VsmmuAt(
        new_s,
        walk.rtte.addr,
    ).state == VSMMU_ACTIVATING) && (new_base == vsmmu.reg_top ==> VsmmuAt(
        new_s,
        walk.rtte.addr,
    ).state == VSMMU_ACTIVE))
}