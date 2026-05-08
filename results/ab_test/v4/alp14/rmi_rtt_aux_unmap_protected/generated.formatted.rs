pub open spec fn RMI_RTT_AUX_UNMAP_PROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: Result<(), RmiStatusCode>,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL as int, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(ipa) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    realm.rtt_tree_per_plane == FEATURE_FALSE || index as int == RMM_RTT_TREE_PRIMARY
        || index as int > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    walk.rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && top == walk_top))
        && (result.is_Ok() ==> (walk.rtte.state == UNASSIGNED && top == walk_top && new_s
        == old_s)))
}