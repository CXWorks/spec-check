pub open spec fn rsi_vsmmu_activate_spec(base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT)
    && (top <= base ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(old_s, old_s, base, top) ==> result == RSI_ERROR_INPUT)
    && (RttWalk_(old_s, old_s, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> (RipasToRmi(GranuleAt(new_s, base).state) == DEV && forall|addr: Address| AddrInRange(new_s, addr, base, top) ==> RipasToRmi(GranuleAt(new_s, addr).state) == DEV))
    && (result == RSI_SUCCESS ==> ((new_base == VsmmuAt(new_s, RttWalk_(old_s, old_s, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_base) && (new_base != VsmmuAt(new_s, RttWalk_(old_s, old_s, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top) ==> VsmmuAt(new_s, RttWalk_(old_s, old_s, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVATING))
    && (result == RSI_SUCCESS ==> (new_base == VsmmuAt(new_s, RttWalk_(old_s, old_s, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).reg_top ==> VsmmuAt(new_s, RttWalk_(old_s, old_s, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == VSMMU_ACTIVE))
}