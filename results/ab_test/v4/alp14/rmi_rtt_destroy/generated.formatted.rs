pub open spec fn RMI_RTT_DESTROY_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    // Precondition checks for failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let level_bound_fail = !RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(
        old_s,
        realm,
        level,
    );
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level - 1);
    let ipa_bound_fail = (ipa as int) >= (1 << realm.ipa_width);
    let rtt_walk_fail = walk.level < level - 1;
    let rtte_state_fail = walk.rtte.state != TABLE;
    let rtt_live_fail = RttIsLive(old_s, RttAt(old_s, walk.rtte.addr));
    let aux_ref_fail = AddrIsAuxRef(ipa, realm);

    // Failure condition postconditions
    (rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rd_bound_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (rd_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (level_bound_fail
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (ipa_align_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (ipa_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rtt_walk_fail ==> (
    result.is_Err() && result.get_Err_0().0 == RMI_ERROR_RTT && result.get_Err_0().1 == walk.level
        && top == walk_top)) && (rtte_state_fail ==> (result.is_Err() && result.get_Err_0().0
        == RMI_ERROR_RTT && result.get_Err_0().1 == walk.level && top == walk_top)) && (
    rtt_live_fail ==> (result.is_Err() && result.get_Err_0().0 == RMI_ERROR_RTT
        && result.get_Err_0().1 == level && top == ipa)) && (aux_ref_fail ==> (result.is_Err()
        && result.get_Err_0().0 == RMI_ERROR_RTT && result.get_Err_0().1 == walk.level))
        &&
    // Success conditions (when no precondition failures)
    (!rd_align_fail && !rd_bound_fail && !rd_state_fail && !level_bound_fail && !ipa_align_fail
        && !ipa_bound_fail && !rtt_walk_fail && !rtte_state_fail && !rtt_live_fail && !aux_ref_fail
        ==> ((result.is_Ok()) && (AddrIsProtected(old_s, ipa, realm) ==> (RttEntryAt(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        entry_idx,
    ).state == UNASSIGNED)) && (AddrIsProtected(old_s, ipa, realm) ==> (RttEntryAt(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        entry_idx,
    ).ripas == DESTROYED)) && (!AddrIsProtected(old_s, ipa, realm) ==> (RttEntryAt(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        entry_idx,
    ).state == UNASSIGNED_NS)) && (GranuleAt(new_s, walk.rtte.addr).state == DELEGATED) && (rtt
        == walk.rtte.addr) && (top == walk_top)))
}