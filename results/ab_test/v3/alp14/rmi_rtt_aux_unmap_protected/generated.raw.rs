pub open spec fn rmi_rtt_aux_unmap_protected_spec(
    result: RmiCommandReturnCode,
    top: Address,
    rd: Address,
    ipa: Address,
    index: u64,
    old_s: S,
    new_s: S
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && top == walk_top))
    && (walk.rtte.state == ASSIGNED ==> (result == RMI_SUCCESS && walk.rtte.state == UNASSIGNED && top == walk_top))
}