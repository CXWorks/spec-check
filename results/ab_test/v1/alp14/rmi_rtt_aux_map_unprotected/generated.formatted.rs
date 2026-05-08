pub open spec fn RMI_RTT_AUX_MAP_UNPROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    index: u64,
) -> (result: Result<(), RmiStatusCode>) {
    let realm = RealmAt(s, rd);
    let walk_pri = RttWalk(s, realm, ipa, realm.rtt_level_start, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(s, ipa, walk_aux.level);

    // Failure condition: rd_align
    if !AddrIsGranuleAligned(rd) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    }
    // Failure condition: rd_bound
     else if !PaIsDelegable(rd) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    }
    // Failure condition: rd_state
     else if GranuleAt(s, rd).state != RmmGranuleState::RD {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    }
    // Failure condition: ipa_align
     else if !AddrIsRttLevelAligned(ipa, realm.rtt_level_start) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    }
    // Failure condition: ipa_bound
     else if UInt(ipa) >= (1u64 << realm.ipa_width) || AddrIsProtected(s, ipa, realm) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    }
    // Failure condition: index_bound
     else if realm.rtt_tree_per_plane == RMM_FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    }
    // Failure condition: pri_state
     else if walk_pri.rtte.state == RmmRttEntryState::UNASSIGNED_NS {
        Err(RmiStatusCode::RMI_ERROR_RTT(walk_pri.level))
    }
    // Success conditions
     else if walk_aux.rtte.state == walk_pri.rtte.state && RttMemAttrEqual(
        walk_aux.rtte,
        walk_pri.rtte,
        RmmRttProtected::RTT_UNPROTECTED,
    ) && RttS2APEqual(walk_aux.rtte, walk_pri.rtte, realm.rtt_s2ap_encoding) && walk_aux.rtte.addr
        == walk_pri.rtte.addr {
        Ok(())
    } else {
        Err(RmiStatusCode::RMI_ERROR_INVALID)
    }
}