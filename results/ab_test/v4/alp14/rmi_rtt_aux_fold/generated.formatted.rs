pub open spec fn RMI_RTT_AUX_FOLD_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
    result: Result<(Address), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));

    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!RttLevelIsValid(old_s, realm, level)
        || RttLevelIsStarting(old_s, realm, level)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsRttLevelAligned(ipa, level - 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY as u64 || (
    index as int) > realm.num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (walk.level
        < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT_AUX)) && (walk.rtte.state != TABLE
        ==> ResultEqual(result, RMI_ERROR_RTT_AUX)) && (!RttIsHomogeneous(
        old_s,
        RttAt(old_s, walk.rtte.addr),
    ) ==> ResultEqual(result, RMI_ERROR_RTT_AUX)) && (result.is_Ok() ==> (walk.rtte.state
        == fold_pre.state && ((fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS)
        ==> walk.rtte.addr == fold_pre.addr) && ((fold_pre.state == ASSIGNED) ==> (RttMemAttrEqual(
        walk.rtte,
        fold_pre,
        RTT_PROTECTED,
    ) && RttS2APEqual(walk.rtte, fold_pre, S2AP_INDIRECT))) && ((fold_pre.state == ASSIGNED_NS)
        ==> (RttMemAttrEqual(walk.rtte, fold_pre, RTT_UNPROTECTED) && RttS2APEqual(
        walk.rtte,
        fold_pre,
        realm.rtt_s2ap_encoding,
    ))) && (AddrIsProtected(old_s, ipa, realm) ==> walk.rtte.ripas == fold_pre.ripas) && GranuleAt(
        new_s,
        walk.rtte.addr,
    ).state == DELEGATED && result.get_Ok_0() == walk.rtte.addr)))
}