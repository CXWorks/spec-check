pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, ipa, realm.rtt_level_start as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, realm.rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= pow2(realm.ipa_width as int) || AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok() ==> (RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).state == UNASSIGNED_NS))
    && (result.is_Ok() ==> (top == walk_top))
    && (result.is_Ok() ==> (RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED_NS))
}