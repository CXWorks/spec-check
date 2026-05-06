pub open spec fn RMI_RTT_AUX_MAP_PROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    index: u64,
    result: Result<(RmiRttEntryState, RmiRipas), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk_pri = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, index as int);
    let entry_idx = RttEntryIndex(s, ipa, walk_aux.level);

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(s, rd).state != RD;
    let ipa_align_fail = !AddrIsGranuleAligned(ipa);
    let ipa_bound_fail = !AddrIsProtected(s, ipa, realm);
    let index_bound_fail = realm.rtt_tree_per_plane == FEATURE_FALSE || index as int
        == RMM_RTT_TREE_PRIMARY || index as int > realm.num_aux_planes;
    let pri_unassigned_fail = walk_pri.rtte.state != ASSIGNED && walk_pri.rtte.state != ASSIGNED_DEV
        && walk_pri.rtte.state != ASSIGNED_VSMMU;
    let pri_ram_fail = walk_pri.rtte.state == ASSIGNED && walk_pri.rtte.ripas != RAM;
    let pri_dev_fail = walk_pri.rtte.state == ASSIGNED_DEV && walk_pri.rtte.ripas != DEV;
    let aux_destroyed_fail = walk_aux.rtte.state == AUX_DESTROYED;
    let level_fail = walk_aux.level < walk_pri.level;

    // Check failure conditions in order
    if rd_align_fail || rd_bound_fail || rd_state_fail {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT
    } else if ipa_align_fail || ipa_bound_fail || index_bound_fail {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT
    } else if pri_unassigned_fail {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT && result.get_Err_0().level
            == walk_pri.level
    } else if pri_ram_fail {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT && result.get_Err_0().level
            == walk_pri.level
    } else if pri_dev_fail {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT && result.get_Err_0().level
            == walk_pri.level
    } else if aux_destroyed_fail {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT_AUX && result.get_Err_0().level
            == walk_aux.level
    } else if level_fail {
        result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT_AUX && result.get_Err_0().level
            == walk_aux.level
    } else {
        // Success case
        result.is_Ok() && walk_aux.rtte.state == ASSIGNED && walk_aux.rtte.attr_prot
            == walk_pri.rtte.attr_prot && walk_aux.rtte.sh == walk_pri.rtte.sh && walk_aux.rtte.addr
            == walk_pri.rtte.addr + (entry_idx * RttLevelSize(s, walk_aux.level))
    }
}