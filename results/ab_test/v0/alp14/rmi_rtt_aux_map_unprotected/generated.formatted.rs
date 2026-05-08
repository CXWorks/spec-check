pub open spec fn RMI_RTT_AUX_MAP_UNPROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk_pri = RttWalk(s, realm, ipa, realm.rtt_level_start, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(s, ipa, walk_aux.level);

    let rd_align_fail = !AddrIsGranuleAligned(s, rd);
    let rd_bound_fail = !PaIsDelegable(s, rd);
    let rd_state_fail = GranuleAt(s, rd).state != RmmGranuleState::RD;
    let ipa_align_fail = !AddrIsRttLevelAligned(s, ipa, realm.rtt_level_start);
    let ipa_bound_fail = (UInt(ipa) >= (1 << realm.ipa_width)) || AddrIsProtected(s, ipa, realm);
    let index_bound_fail = (realm.rtt_tree_per_plane == RmmBool::False) || (index
        == RMM_RTT_TREE_PRIMARY) || (index > realm.num_aux_planes);
    let pri_state_fail = walk_pri.rtte.state == RmmRttEntryState::UNASSIGNED_NS;

    (rd_align_fail ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (rd_bound_fail
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (rd_state_fail ==> ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_INPUT,
    )) && (ipa_align_fail ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (
    ipa_bound_fail ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (index_bound_fail
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)) && (pri_state_fail ==> ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_RTT,
    )) && ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !ipa_align_fail && !ipa_bound_fail
        && !index_bound_fail && !pri_state_fail) ==> (result.is_Ok() && walk_aux.rtte.state
        == walk_pri.rtte.state && RttMemAttrEqual(
        walk_aux.rtte,
        walk_pri.rtte,
        RmmRttProtected::UNPROTECTED,
    ) && RttS2APEqual(walk_aux.rtte, walk_pri.rtte, realm.rtt_s2ap_encoding) && walk_aux.rtte.addr
        == walk_pri.rtte.addr))
}