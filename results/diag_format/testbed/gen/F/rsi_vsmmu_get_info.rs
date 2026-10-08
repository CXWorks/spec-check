pub open spec fn rsi_vsmmu_get_info_spec(addr: Address, result: RsiCommandReturnCode, top: Address, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, addr, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    (!((addr as int) % (RMM_GRANULE_SIZE as int) != 0) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsProtected(old_s, addr, realm) ==> result == RSI_ERROR_INPUT)
    && (!RttEntryStateToRmi(old_s, RttEntryAt(old_s, RttAt(old_s, addr), walk.index as int)).state == RMI_ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && (!AddrIsWithin(old_s, addr, RttAt(old_s, addr), walk.rtt_addr, RttAt(old_s, addr), walk.rtt_addr + (RttLevelSize(old_s, RMM_RTT_PAGE_LEVEL as int) as int)) ==> result == RSI_ERROR_INPUT)
    && (result == RSI_SUCCESS ==> top == RttAt(old_s, addr).reg_top)
    && (result == RSI_SUCCESS ==> AddrIsWithin(old_s, addr, RttAt(old_s, addr), walk.rtt_addr, RttAt(old_s, addr), walk.rtt_addr + (RttLevelSize(old_s, RMM_RTT_PAGE_LEVEL as int) as int)))
}