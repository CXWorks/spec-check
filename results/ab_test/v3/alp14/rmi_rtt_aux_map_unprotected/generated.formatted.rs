pub open spec fn RMI_RTT_AUX_MAP_UNPROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, realm.rtt_level_start, 0);
    let walk_aux = RttWalk(old_s, realm, ipa, realm.rtt_level_start, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level);

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
    ((UInt(ipa) >= pow(2, realm.ipa_width) || AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) &&
    // index_bound failure
    ((realm.rtt_tree_per_plane == FEATURE_FALSE || index == 0 || index
        > realm.num_aux_planes as u64) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // pri_state failure
    (walk_pri.rtte.state == UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
    // Success conditions (when no failure conditions are triggered)
    ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && AddrIsRttLevelAligned(ipa, realm.rtt_level_start) && !(UInt(ipa) >= pow(
        2,
        realm.ipa_width,
    ) || AddrIsProtected(old_s, ipa, realm)) && !(realm.rtt_tree_per_plane == FEATURE_FALSE || index
        == 0 || index > realm.num_aux_planes as u64) && walk_pri.rtte.state != UNASSIGNED_NS) ==> (
    result.is_Ok() && RttMemAttrEqual(walk_aux.rtte, walk_pri.rtte, RTT_UNPROTECTED)
        && RttS2APEqual(walk_aux.rtte, walk_pri.rtte, realm.rtt_s2ap_encoding) && walk_aux.rtte.addr
        == walk_pri.rtte.addr)))
}