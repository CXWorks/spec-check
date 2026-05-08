pub open spec fn RMI_RTT_AUX_DESTROY_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_state
    (GranuleAt(old_s, rd).state != GranuleState::RD ==> ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_INPUT,
    )) &&
    // Failure condition: level_bound
    ((!RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level))
        ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
        &&
    // Failure condition: ipa_align
    (!AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
        &&
    // Failure condition: ipa_bound
    (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
        &&
    // Failure condition: index_bound
    ((realm.rtt_tree_per_plane == FeatureBit::FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY
        || index > realm.num_aux_planes as u64) ==> ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_INPUT,
    )) &&
    // Failure condition: rtt_walk
    (walk.level < level - 1 ==> (ResultEqual(result, RmiStatusCode::RMI_ERROR_RTT_AUX) && top
        == walk_top)) &&
    // Failure condition: rtte_state
    (walk.rtte.state != RmmRttEntryState::TABLE ==> (ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_RTT_AUX,
    ) && top == walk_top)) &&
    // Failure condition: rtt_live
    (RttIsLive(old_s, RttAt(old_s, walk.rtte.addr)) ==> (ResultEqual(
        result,
        RmiStatusCode::RMI_ERROR_RTT_AUX,
    ) && top == ipa)) &&
    // Success conditions
    ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state
        == GranuleState::RD && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(
        old_s,
        realm,
        level,
    ) && AddrIsRttLevelAligned(ipa, level - 1) && AddrIsProtected(ipa, realm) && !(
    realm.rtt_tree_per_plane == FeatureBit::FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes as u64) && walk.level >= level - 1 && walk.rtte.state
        == RmmRttEntryState::TABLE && !RttIsLive(old_s, RttAt(old_s, walk.rtte.addr))) ==> (
    result.is_Ok() && walk.rtte.state == RmmRttEntryState::AUX_DESTROYED && walk.rtte.ripas
        == RmmRipas::DESTROYED && GranuleAt(new_s, walk.rtte.addr).state == GranuleState::DELEGATED
        && rtt == walk.rtte.addr && top == walk_top))
}