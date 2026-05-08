pub open spec fn RMI_RTT_AUX_UNMAP_PROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: Result<(Address), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, index as int);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);

    let rd_align_cond = !AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_cond = !PaIsDelegable(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_cond = GranuleAt(s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_align_cond = !AddrIsGranuleAligned(s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_bound_cond = !AddrIsProtected(s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let index_bound_cond = (realm.rtt_tree_per_plane == FEATURE_FALSE || index
        == RMM_RTT_TREE_PRIMARY as u64 || index > realm.num_aux_planes as u64) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let rtte_state_cond = walk.rtte.state != ASSIGNED ==> (result.is_Err() && result.get_Err_0()
        == RMI_ERROR_RTT_AUX && result.get_ok_1() == walk_top);

    let rtte_state_success = walk.rtte.state == ASSIGNED ==> walk.rtte.state == UNASSIGNED;
    let top_success = walk.rtte.state == ASSIGNED ==> (result.is_Ok() && result.get_Ok_0().1
        == walk_top);

    rd_align_cond && rd_bound_cond && rd_state_cond && ipa_align_cond && ipa_bound_cond
        && index_bound_cond && rtte_state_cond && rtte_state_success && top_success
}