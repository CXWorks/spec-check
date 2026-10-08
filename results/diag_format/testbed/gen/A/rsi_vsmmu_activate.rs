pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
    let old_realm = CurrentRealm(old_s);
    let old_walk = RttWalk(old_s, old_realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let realm = CurrentRealm(new_s);
    let walk = RttWalk(new_s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let vsmmu = VsmmuAt(new_s, walk.rtte.addr);
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
    && (top <= base ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, base, top, old_realm) ==> result == RSI_ERROR_INPUT)
    && (old_walk.rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(old_s, base)
        && AddrIsGranuleAligned(old_s, top)
        && top > base
        && AddrRangeIsProtected(old_s, base, top, old_realm)
        && old_walk.rtte.state == ASSIGNED_VSMMU)
        ==> (result == RSI_SUCCESS
            && RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level, base, new_base, DEV)
            && ((base == vsmmu.reg_base && new_base != vsmmu.reg_top) ==> vsmmu.state == VSMMU_ACTIVATING)
            && (new_base == vsmmu.reg_top ==> vsmmu.state == VSMMU_ACTIVE)))
}
