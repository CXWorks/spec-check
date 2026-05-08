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

    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (
    !PaIsDelegable(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rd,
    ).state != GranuleState::RD ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (
    !AddrIsRttLevelAligned(ipa, realm.rtt_level_start) ==> ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_INPUT,
    )) && ((ipa as int >= (1 << realm.ipa_width) || AddrIsProtected(old_s, ipa, realm))
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && ((realm.rtt_tree_per_plane
        == FeatureBool::FEATURE_FALSE || index == 0 || index as int > realm.num_aux_planes)
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && ((walk_pri.rtte.state
        == RmmRttEntryState::UNASSIGNED_NS && (!AddrIsGranuleAligned(rd) || !PaIsDelegable(rd)
        || GranuleAt(old_s, rd).state != GranuleState::RD || ipa as int >= (1 << realm.ipa_width)
        || AddrIsProtected(old_s, ipa, realm) || (realm.rtt_tree_per_plane
        == FeatureBool::FEATURE_FALSE || index == 0 || index as int > realm.num_aux_planes)))
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT)) && (result.is_Ok() ==> (
    walk_aux.rtte.state == walk_pri.rtte.state && RttMemAttrEqual(
        walk_aux.rtte,
        walk_pri.rtte,
        RmmRttProtected::RTT_UNPROTECTED,
    ) && RttS2APEqual(walk_aux.rtte, walk_pri.rtte, realm.rtt_s2ap_encoding) && walk_aux.rtte.addr
        == walk_pri.rtte.addr)))
}