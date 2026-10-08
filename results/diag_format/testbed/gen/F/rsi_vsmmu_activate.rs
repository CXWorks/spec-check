pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
    && ((top as u64) <= (base as u64) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, base, top, old_s.CurrentRealm()) ==> result == RSI_ERROR_INPUT)
    && (let walk = RttWalk(old_s, old_s.CurrentRealm(), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int); walk.level != RMM_RTT_PAGE_LEVEL || RttEntryStateToRmi(old_s, RttEntryAt(old_s, walk.rtte.addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int)) != RMI_ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (let walk = RttWalk(old_s, old_s.CurrentRealm(), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int); let rtte = RttEntryAt(old_s, walk.rtte.addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int); let vsmmu = VsmmuAt(old_s, rtte.addr); GranulesAllState(old_s, base, new_base, DEV) && (base == vsmmu.reg_base && new_base != vsmmu.reg_top ==> vsmmu.state == VSMMU_ACTIVATING) && (new_base == vsmmu.reg_top ==> vsmmu.state == VSMMU_ACTIVE)))
}