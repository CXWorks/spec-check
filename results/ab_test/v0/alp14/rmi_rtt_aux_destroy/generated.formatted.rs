pub open spec fn RMI_RTT_AUX_DESTROY_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(s, rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(s, rd).state != RD && ResultEqual(result, RMI_ERROR_INPUT);
    let level_bound_fail = (!RttLevelIsValid(s, realm, level) || RttLevelIsStarting(
        s,
        realm,
        level,
    )) && ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_align_fail = !AddrIsRttLevelAligned(s, ipa, level - 1) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let ipa_bound_fail = !AddrIsProtected(s, ipa, realm) && ResultEqual(result, RMI_ERROR_INPUT);
    let index_bound_fail = (realm.rtt_tree_per_plane == FEATURE_FALSE || index
        == RMM_RTT_TREE_PRIMARY || index as int > realm.num_aux_planes) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let rtt_walk_fail = walk.level < level - 1 && result.is_Err() && top == walk_top;
    let rtte_state_fail = walk.rtte.state != TABLE && result.is_Err() && top == walk_top;
    let rtt_live_fail = RttIsLive(s, RttAt(s, walk.rtte.addr)) && result.is_Err() && top == ipa;

    // Success conditions
    let rtte_state_success = walk.rtte.state == AUX_DESTROYED;
    let ripas_success = walk.rtte.ripas == DESTROYED;
    let rtt_state_success = GranuleAt(s, walk.rtte.addr).state == DELEGATED;
    let rtt_success = rtt == walk.rtte.addr;
    let top_success = top == walk_top;

    // Check if any failure condition applies
    let has_failure = rd_align_fail || rd_bound_fail || rd_state_fail || level_bound_fail
        || ipa_align_fail || ipa_bound_fail || index_bound_fail || rtt_walk_fail || rtte_state_fail
        || rtt_live_fail;

    // If no failure, check success conditions
    let success = !has_failure && rtte_state_success && ripas_success && rtt_state_success
        && rtt_success && top_success && result.is_Ok();

    has_failure || success
}