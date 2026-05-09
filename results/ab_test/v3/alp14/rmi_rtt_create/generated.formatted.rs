pub open spec fn rmi_rtt_create_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: int,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let walk_pre = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let rtte_pre = walk_pre.rtte;
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!RttLevelIsValid(old_s, realm, level)
        || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsRttLevelAligned(old_s, ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    UInt64(ipa) >= (pow(2, realm.ipa_width)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rtt,
    ).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((realm.feat_lpa2
        == FEATURE_FALSE && UInt64(rtt) >= pow(2, 48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (walk.level < level - 1 ==> result.is_Err() && result.get_Err_0().code == RMI_ERROR_RTT
        && result.get_Err_0().index == walk.level) && (walk.rtte.state == TABLE ==> result.is_Err()
        && result.get_Err_0().code == RMI_ERROR_RTT && result.get_Err_0().index == walk.level) && ((
    AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level) && !RttLevelIsStarting(old_s, realm, level)
        && AddrIsRttLevelAligned(old_s, ipa, level - 1) && UInt64(ipa) < pow(2, realm.ipa_width)
        && AddrIsGranuleAligned(old_s, rtt) && PaIsDelegableDram(old_s, rtt) && GranuleAt(
        old_s,
        rtt,
    ).state == DELEGATED && (realm.feat_lpa2 != FEATURE_FALSE || UInt64(rtt) < pow(2, 48))
        && walk.level >= level - 1 && walk.rtte.state != TABLE) ==> (result.is_Ok() && GranuleAt(
        new_s,
        rtt,
    ).state == RTT && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == TABLE
        && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == rtt && (
    AddrIsProtected(old_s, ipa, realm) ==> RttAllEntriesRipas(
        new_s,
        RttAt(new_s, rtt),
        rtte_pre.ripas,
    )) && RttAllEntriesState(new_s, RttAt(new_s, rtt), rtte_pre.state) && ((rtte_pre.state
        != UNASSIGNED && rtte_pre.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(
        new_s,
        RttAt(new_s, rtt),
        rtte_pre.addr,
        level,
    ))))
}