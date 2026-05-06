```verus
pub open spec fn RMI_RTT_AUX_UNMAP_UNPROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: Result<(), RmiStatusCode>,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    (
        // rd_align failure
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // rd_bound failure
        (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // rd_state failure
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // ipa_align failure
        (!AddrIsRttLevelAligned(ipa, realm.rtt_level_start) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // ipa_bound failure
        ((UInt(ipa) >= (1 << realm.ipa_width) || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // index_bound failure
        ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        // Success case: all preconditions met
        (
            (AddrIsGranuleAligned(rd)
            && PaIsDelegable(rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsRttLevelAligned(ipa, realm.rtt_level_start)
            && UInt(ipa) < (1 << realm.ipa_width)
            && !AddrIsProtected(old_s, ipa, realm)
            && !(realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes))
            ==>
            (result.is_Ok()
            && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED_NS
            && top == walk_top)
        )
    )
}
```