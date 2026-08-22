pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
    && ((top as int) <= (base as int) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s)) ==> result == RSI_ERROR_INPUT)
    && (RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && ((AddrIsGranuleAligned(old_s, base)
            && AddrIsGranuleAligned(old_s, top)
            && (top as int) > (base as int)
            && AddrRangeIsProtected(old_s, base, top, CurrentRealm(old_s))
            && RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_VSMMU)
        ==> (result == RSI_SUCCESS
            && RttEntriesInRangeRipas(new_s, RttWalk(new_s, CurrentRealm(new_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt, RMM_RTT_PAGE_LEVEL as int, base, new_base, DEV)
            && ((base == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).reg_base
                    && new_base != VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).reg_top)
                ==> VsmmuAt(new_s, RttWalk(new_s, CurrentRealm(new_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).state == VSMMU_ACTIVATING)
            && (new_base == VsmmuAt(old_s, RttWalk(old_s, CurrentRealm(old_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).reg_top
                ==> VsmmuAt(new_s, RttWalk(new_s, CurrentRealm(new_s), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).state == VSMMU_ACTIVE)))
}