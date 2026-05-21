pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    ipa: Address,
    index: u64,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(
        ipa,
        realm.rtt_level_start,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((UInt(ipa) >= (2 ^ realm.ipa_width)
        || AddrIsProtected(ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((AddrIsGranuleAligned(
        rd,
    ) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD && AddrIsRttLevelAligned(
        ipa,
        realm.rtt_level_start,
    ) && !(UInt(ipa) >= (2 ^ realm.ipa_width) || AddrIsProtected(ipa, realm)) && !(
    realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes)) ==> (result == RMI_OK && RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx as int,
    ).state == UNASSIGNED_NS))
}