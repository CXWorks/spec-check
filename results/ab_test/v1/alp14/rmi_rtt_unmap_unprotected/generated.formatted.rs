pub open spec fn RMI_RTT_UNMAP_UNPROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
) -> (result: RmiCommandReturnCode, top: Address)
{
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    let rtte = RttEntryAt(s, RttAt(s, walk.rtt_addr), entry_idx);

    // Failure condition: rd_align
    if !AddrIsGranuleAligned(rd) {
        (RMI_ERROR_INPUT, 0)
    }
    // Failure condition: rd_bound
    else if !PaIsDelegable(rd) {
        (RMI_ERROR_INPUT, 0)
    }
    // Failure condition: rd_state
    else if GranuleAt(s, rd).state != RD {
        (RMI_ERROR_INPUT, 0)
    }
    // Failure condition: level_bound
    else if !RttLevelIsValid(s, realm, level) || level < 1 {
        (RMI_ERROR_INPUT, 0)
    }
    // Failure condition: ipa_align
    else if !AddrIsRttLevelAligned(ipa, level) {
        (RMI_ERROR_INPUT, 0)
    }
    // Failure condition: ipa_bound
    else if UInt(ipa) >= (1 << realm.ipa_width) || AddrIsProtected(ipa, realm) {
        (RMI_ERROR_INPUT, 0)
    }
    // Failure condition: rtt_walk
    else if walk.level < level {
        (RmiMakeErrorCode(RMI_ERROR_RTT, walk.level), walk_top)
    }
    // Failure condition: rtte_state
    else if rtte.state != ASSIGNED_NS {
        (RmiMakeErrorCode(RMI_ERROR_RTT, walk.level), walk_top)
    }
    // Success condition
    else {
        (RMI_SUCCESS, walk_top)
    }
}