pub open spec fn RMI_DATA_CREATE_UNKNOWN_spec(
    s: S,
    rd: Address,
    data: Address,
    ipa: Address,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);

    // Failure conditions (pre-conditions that lead to errors)
    let data_align_fail = !AddrIsGranuleAligned(s, data);
    let data_bound_fail = !PaIsDelegableDram(s, data);
    let data_state_fail = GranuleAt(s, data).state != RmmGranuleState::DELEGATED;
    let data_bound2_fail = (realm.feat_lpa2 == RmmFeature::FEATURE_FALSE) && (data as int
        >= 281474976710656);
    let rd_align_fail = !AddrIsGranuleAligned(s, rd);
    let rd_bound_fail = !PaIsDelegable(s, rd);
    let rd_state_fail = GranuleAt(s, rd).state != RmmGranuleState::RD;
    let ipa_align_fail = !AddrIsGranuleAligned(s, ipa);
    let ipa_bound_fail = !AddrIsProtected(s, ipa, realm);
    let rtt_walk_fail = walk.level < RMM_RTT_PAGE_LEVEL;
    let rtte_state_fail = walk.rtte.state != RmmRttEntryState::UNASSIGNED;

    // Success conditions
    let data_state_ok = GranuleAt(s, data).state == RmmGranuleState::DATA;
    let rtte_state_ok = walk.rtte.state == RmmRttEntryState::ASSIGNED;
    let rtte_addr_ok = walk.rtte.addr == data;
    let rtte_memattr_ok = walk.rtte.attr_prot == RmmRttMemAttr::MEMATTR_CACHEABLE;
    let rtte_sh_ok = walk.rtte.sh == RmmRttShareability::SHAREABILITY_INNER;

    // Check failure conditions with proper ordering
    if data_align_fail || data_bound_fail || data_state_fail || data_bound2_fail || rd_align_fail
        || rd_bound_fail || ipa_align_fail || ipa_bound_fail {
        false
    } else if rd_state_fail || rtt_walk_fail || rtte_state_fail {
        false
    } else {
        // Success case: all conditions must hold
        data_state_ok && rtte_state_ok && rtte_addr_ok && rtte_memattr_ok && rtte_sh_ok
    }
}