pub open spec fn rmi_rtt_aux_create_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: int,
    index: u64,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let unfold = RttWalk(old_s, realm, ipa, level - 1, index).rtte;

    // Failure conditions
    ((!AddrIsGranuleAligned(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !PaIsDelegable(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(
        old_s,
        rd,
    ).state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((!RttLevelIsValid(
        old_s,
        realm,
        level,
    )) || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsRttLevelAligned(old_s, ipa, level - 1)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsProtected(old_s, ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((
    realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsGranuleAligned(old_s, rtt)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !PaIsDelegableDram(old_s, rtt)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(
        old_s,
        rtt,
    ).state != DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((realm.feat_lpa2
        == FEATURE_FALSE) && (UInt(rtt) >= 281474976710656i64)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((walk.level < level - 1) ==> ResultEqual(result, RMI_ERROR_RTT_AUX)) && ((walk.rtte.state
        == TABLE) ==> ResultEqual(
        result,
        RMI_ERROR_RTT_AUX,
    ))
    // Success conditions
     && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state
        == RD && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level)
        && AddrIsRttLevelAligned(old_s, ipa, level - 1) && AddrIsProtected(old_s, ipa, realm) && !(
    realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index
        > realm.num_aux_planes) && AddrIsGranuleAligned(old_s, rtt) && PaIsDelegableDram(old_s, rtt)
        && GranuleAt(old_s, rtt).state == DELEGATED && !((realm.feat_lpa2 == FEATURE_FALSE) && (
    UInt(rtt) >= 281474976710656i64)) && walk.level >= level - 1 && walk.rtte.state != TABLE) ==> (
    GranuleAt(new_s, rtt).state == RTT && walk.rtte.state == TABLE && walk.rtte.addr == rtt && (
    AddrIsProtected(old_s, ipa, realm) ==> RttAllEntriesRipas(
        old_s,
        RttAt(old_s, rtt),
        unfold.ripas,
    )) && RttAllEntriesState(old_s, RttAt(old_s, rtt), unfold.state) && (unfold.state != UNASSIGNED
        ==> RttAllEntriesContiguous(old_s, RttAt(old_s, rtt), unfold.addr, level)) && result
        == RMI_SUCCESS))
}