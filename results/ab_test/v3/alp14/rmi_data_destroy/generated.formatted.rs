pub open spec fn RMI_DATA_DESTROY_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    result: RmiCommandReturnCode,
    data: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    // Failure conditions with ordering
    (
    // rd_align failure
    (!AddrIsGranuleAligned(old_s, rd) ==> (result.is_Err() && result.get_Err_0()
        == RMI_ERROR_INPUT))
        &&
    // rd_bound failure (ordered before rtt_walk, rtte_state, aux_live)
    (!PaIsDelegable(old_s, rd) ==> (result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT))
        &&
    // rd_state failure (ordered before rtt_walk, rtte_state, aux_live)
    (GranuleAt(old_s, rd).state != RD ==> (result.is_Err() && result.get_Err_0()
        == RMI_ERROR_INPUT)) &&
    // ipa_align failure
    (!AddrIsGranuleAligned(old_s, ipa) ==> (result.is_Err() && result.get_Err_0()
        == RMI_ERROR_INPUT))
        &&
    // ipa_bound failure (ordered before rtt_walk, rtte_state, aux_live)
    (!AddrIsProtected(old_s, ipa, realm) ==> (result.is_Err() && result.get_Err_0()
        == RMI_ERROR_INPUT)) &&
    // rtt_walk failure
    (walk.level < RMM_RTT_PAGE_LEVEL ==> (result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT
        && top == walk_top)) &&
    // rtte_state failure
    (walk.rtte.state != ASSIGNED ==> (result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT && top
        == walk_top)) &&
    // aux_live failure
    (AddrIsAuxLive(old_s, ipa, realm) ==> (result.is_Err() && result.get_Err_0()
        == RMI_ERROR_RTT_AUX))
        &&
    // Success conditions (when all preconditions are satisfied)
    ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state
        == RD && AddrIsGranuleAligned(old_s, ipa) && AddrIsProtected(old_s, ipa, realm)
        && walk.level == RMM_RTT_PAGE_LEVEL && walk.rtte.state == ASSIGNED && !AddrIsAuxLive(
        old_s,
        ipa,
        realm,
    )) ==> (result.is_Ok() && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED && walk.rtte.state
        == UNASSIGNED && (walk.rtte.ripas == RAM ==> walk.rtte.ripas == DESTROYED) && data
        == walk.rtte.addr && top == walk_top)))
}