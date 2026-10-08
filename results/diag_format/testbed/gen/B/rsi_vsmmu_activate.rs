pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
    && (top <= base ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, base, top, old_s.CurrentRealm()) ==> result == RSI_ERROR_INPUT)
    && (let walk = RttWalk(old_s, old_s.CurrentRealm(), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY); walk.rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (forall a: Address | AddrIsWithin(a, base, new_base) ==> RttWalk(old_s, old_s.CurrentRealm(), a, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas == DEV))
    && (result == RSI_SUCCESS ==> (let vsmmu = VsmmuAt(old_s, walk.rtte.addr); (base == vsmmu.reg_base && new_base != vsmmu.reg_top) ==> vsmmu.state == VSMMU_ACTIVATING))
    && (result == RSI_SUCCESS ==> (let vsmmu = VsmmuAt(old_s, walk.rtte.addr); new_base == vsmmu.reg_top ==> vsmmu.state == VSMMU_ACTIVE))
}