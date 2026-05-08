pub open spec fn RMI_RTT_AUX_UNMAP_PROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: Result<(), RmiStatusCode>,
    top: Address,
) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(realm, ipa, RMM_RTT_PAGE_LEVEL, index);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(RttAt(walk.rtt_addr), walk.level, ipa);

    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_state
    (GranuleAt(rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: ipa_align
    (!AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: ipa_bound
    (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: index_bound
    ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rtte_state (ordered after rd_bound, rd_state, ipa_bound, index_bound)
    (walk.rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && top == walk_top))
        &&
    // Success condition: rtte_state
    (result.is_Ok() ==> walk.rtte.state == UNASSIGNED)
        &&
    // Success condition: top
    (result.is_Ok() ==> top == walk_top)
}