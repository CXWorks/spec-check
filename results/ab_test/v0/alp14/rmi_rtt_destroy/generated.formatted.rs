pub open spec fn RMI_RTT_DESTROY_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);

    let rd_align_ok = AddrIsGranuleAligned(rd);
    let rd_bound_ok = PaIsDelegable(rd);
    let rd_state_ok = GranuleAt(s, rd).state == RD;
    let level_bound_ok = RttLevelIsValid(s, realm, level) && !RttLevelIsStarting(s, realm, level);
    let ipa_align_ok = AddrIsRttLevelAligned(ipa, level - 1);
    let ipa_bound_ok = UInt(ipa) < (1 << realm.ipa_width);
    let rtt_walk_ok = walk.level >= level - 1;
    let rtte_state_ok = walk.rtte.state == TABLE;
    let rtt_live_ok = !RttIsLive(s, RttAt(s, walk.rtte.addr));
    let aux_ref_ok = !AddrIsAuxRef(ipa, realm);

    if !rd_align_ok || !rd_bound_ok || !rd_state_ok {
        result == RmiCommandReturnCode::Error(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !level_bound_ok || !ipa_bound_ok {
        result == RmiCommandReturnCode::Error(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !rtt_walk_ok || !rtte_state_ok {
        result == RmiCommandReturnCode::Error(RmiStatusCode::RMI_ERROR_RTT(walk.level)) && top
            == walk_top
    } else if !rtte_state_ok {
        result == RmiCommandReturnCode::Error(RmiStatusCode::RMI_ERROR_RTT(walk.level)) && top
            == walk_top
    } else if !rtt_live_ok {
        result == RmiCommandReturnCode::Error(RmiStatusCode::RMI_ERROR_RTT(level)) && top == ipa
    } else if !aux_ref_ok {
        result == RmiCommandReturnCode::Error(RmiStatusCode::RMI_ERROR_RTT(walk.level))
    } else {
        result == RmiCommandReturnCode::Success && rtt == walk.rtte.addr && top == walk_top && ((
        AddrIsProtected(ipa, realm) && walk.rtte.state == UNASSIGNED && walk.rtte.ripas
            == DESTROYED) || (!AddrIsProtected(ipa, realm) && walk.rtte.state == UNASSIGNED_NS))
            && GranuleAt(s, walk.rtte.addr).state == DELEGATED
    }
}