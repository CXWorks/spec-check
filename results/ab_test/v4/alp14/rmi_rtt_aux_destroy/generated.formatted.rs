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
    let walk = RttWalk(old_s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!RttLevelIsValid(old_s, realm, level)
        || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    realm.rtt_tree_per_plane == FEATURE_FALSE || index as int == RMM_RTT_TREE_PRIMARY
        || index as int > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    walk.level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && top == walk_top)) && (
    walk.rtte.state != TABLE ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && top == walk_top)) && (
    RttIsLive(old_s, RttAt(old_s, walk.rtte.addr)) ==> (ResultEqual(result, RMI_ERROR_RTT_AUX)
        && top == ipa)) && (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(
        old_s,
        rd,
    ).state == RD && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(
        old_s,
        realm,
        level,
    ) && AddrIsRttLevelAligned(ipa, level - 1) && AddrIsProtected(old_s, ipa, realm) && (
    realm.rtt_tree_per_plane != FEATURE_FALSE && index as int != RMM_RTT_TREE_PRIMARY
        && index as int <= realm.num_aux_planes) && walk.level == level - 1 && walk.rtte.state
        == TABLE && !RttIsLive(old_s, RttAt(old_s, walk.rtte.addr)) ==> (walk.rtte.state
        == AUX_DESTROYED && walk.rtte.ripas == DESTROYED && GranuleAt(new_s, walk.rtte.addr).state
        == DELEGATED && rtt == walk.rtte.addr && top == walk_top)))
}