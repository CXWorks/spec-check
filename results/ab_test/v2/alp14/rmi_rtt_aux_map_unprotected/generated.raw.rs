```verus
pub open spec fn rmi_rtt_aux_map_unprotected_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk_pri = RttWalk(old_s, realm, ipa, realm.rtt_level_start, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(old_s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk_aux.level);

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, realm.rtt_level_start);
    let ipa_bound_fail = (UInt(ipa) >= (Int::pow(2, realm.ipa_width as int)) 
                          || AddrIsProtected(ipa, realm));
    let index_bound_fail = (realm.rtt_tree_per_plane == FEATURE_FALSE
                            || index == RMM_RTT_TREE_PRIMARY
                            || index > realm.num_aux_planes);
    let pri_state_fail = walk_pri.rtte.state == UNASSIGNED_NS;

    // Ordering constraints
    let rd_checks_first = rd_align_fail || rd_bound_fail;
    let ipa_index_checks_first = ipa_align_fail || ipa_bound_fail || index_bound_fail;

    // Failure path
    (rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_bound_fail && !rd_checks_first ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_state_fail && !rd_checks_first ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_bound_fail && !ipa_index_checks_first ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (index_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pri_state_fail && !rd_checks_first && !ipa_index_checks_first && !rd_state_fail 
        ==> ResultEqual(result, RMI_ERROR_RTT))

    // Success conditions
    && (!rd_align_fail && !rd_bound_fail && !rd_state_fail 
        && !ipa_align_fail && !ipa_bound_fail && !index_bound_fail && !pri_state_fail
        ==> (result.is_Ok()
             && walk_aux.rtte.state == walk_pri.rtte.state
             && RttMemAttrEqual(walk_aux.rtte, walk_pri.rtte, RTT_UNPROTECTED)
             && RttS2APEqual(walk_aux.rtte, walk_pri.rtte, realm.rtt_s2ap_encoding)
             && walk_aux.rtte.addr == walk_pri.rtte.addr))
}
```