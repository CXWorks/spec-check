```verus
pub open spec fn rmi_rtt_aux_fold_spec(
    result: Result<(), RmiStatusCode>,
    rtt: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let fold_pre = RttFold(RttAt(walk.rtte.addr));
    
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && 
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && 
    // Failure condition: rd_state
    (GranuleAt(rd).state != GranuleState::RD ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && 
    // Failure condition: level_bound
    ((!RttLevelIsValid(realm, level) || RttLevelIsStarting(realm, level)) 
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && 
    // Failure condition: ipa_align
    (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && 
    // Failure condition: ipa_bound
    (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && 
    // Failure condition: index_bound
    ((realm.rtt_tree_per_plane == FeatureFalse::FEATURE_FALSE || index == RmmRttTreeIndex::RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes) 
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && 
    // Failure condition: rtt_walk
    (walk.level < level - 1 ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX))
    && 
    // Failure condition: rtte_state
    (walk.rtte.state != RmmRttEntryState::TABLE ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX))
    && 
    // Failure condition: rtt_homo
    (!RttIsHomogeneous(RttAt(walk.rtte.addr)) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX))
    && 
    // Success condition: all preconditions met implies success
    ((AddrIsGranuleAligned(rd)
        && PaIsDelegable(rd)
        && GranuleAt(rd).state == GranuleState::RD
        && RttLevelIsValid(realm, level)
        && !RttLevelIsStarting(realm, level)
        && AddrIsRttLevelAligned(ipa, level - 1)
        && AddrIsProtected(ipa, realm)
        && !(realm.rtt_tree_per_plane == FeatureFalse::FEATURE_FALSE || index == RmmRttTreeIndex::RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes)
        && walk.level >= level - 1
        && walk.rtte.state == RmmRttEntryState::TABLE
        && RttIsHomogeneous(RttAt(walk.rtte.addr)))
        ==> (result.is_Ok()
            && rtt == walk.rtte.addr
            && GranuleAt(walk.rtte.addr).state == GranuleState::DELEGATED
            && walk.rtte.state == fold_pre.state
            && (fold_pre.state != RmmRttEntryState::UNASSIGNED && fold_pre.state != RmmRttEntryState::UNASSIGNED_NS
                ==> walk.rtte.addr == fold_pre.addr)
            && (fold_pre.state == RmmRttEntryState::ASSIGNED
                ==> (RttMemAttrEqual(walk.rtte, fold_pre, RmmRttProtected::RTT_PROTECTED)
                    && RttS2APEqual(walk.rtte, fold_pre, RmmRttS2APEncoding::S2AP_INDIRECT)))
            && (fold_pre.state == RmmRttEntryState::ASSIGNED_NS
                ==> (RttMemAttrEqual(walk.rtte, fold_pre, RmmRttProtected::RTT_UNPROTECTED)
                    && RttS2APEqual(walk.rtte, fold_pre, realm.rtt_s2ap_encoding)))
            && (AddrIsProtected(ipa, realm) ==> walk.rtte.ripas == fold_pre.ripas)))
}
```