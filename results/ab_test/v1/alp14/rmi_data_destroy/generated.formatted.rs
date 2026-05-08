pub open spec fn RMI_DATA_DESTROY_spec(
    s: S,
    rd: Address,
    ipa: Address,
) -> (result: RmiCommandReturnCode, data: Address, top: Address)
{
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    let rtte = RttEntryAt(s, RttAt(s, walk.rtt_addr), entry_idx);
    
    // Failure conditions (ordered by precedence)
    if !AddrIsGranuleAligned(rd) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !PaIsDelegable(rd) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if GranuleAt(s, rd).state != RD {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !AddrIsGranuleAligned(ipa) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if !AddrIsProtected(s, ipa, realm) {
        (RMI_ERROR_INPUT, 0, 0)
    } else if walk.level < RMM_RTT_PAGE_LEVEL {
        (RMI_ERROR_RTT + walk.level, 0, walk_top)
    } else if rtte.state != ASSIGNED {
        (RMI_ERROR_RTT + walk.level, 0, walk_top)
    } else if AddrIsAuxLive(s, ipa, realm) {
        (RMI_ERROR_RTT_AUX, 0, 0)
    } else {
        // Success case
        // Post-conditions:
        // - GranuleAt(rtte.addr).state == DELEGATED
        // - rtte.state == UNASSIGNED
        // - if rtte.ripas == RAM: rtte.ripas == DESTROYED, else rtte.ripas unchanged
        // - data == rtte.addr
        // - top == walk_top
        (RMI_SUCCESS, rtte.addr, walk_top)
    }
}