pub open spec fn RMI_RTT_AUX_UNMAP_UNPROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: RmiCommandReturnCode,
    top: Address
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, realm.rtt_level_start, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     (!AddrIsRttLevelAligned(ipa, realm.rtt_level_start) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     ((ipa as int >= (1 << realm.ipa_width) || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
     ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY as u64 || index > realm.num_aux_planes as u64) ==> ResultEqual(result, RMI_ERROR_INPUT))) &&
    (result.is_Ok() ==> (walk.rtte.state == UNASSIGNED_NS && top == walk_top))
}