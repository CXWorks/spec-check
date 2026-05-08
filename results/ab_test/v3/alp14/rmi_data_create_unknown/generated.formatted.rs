pub open spec fn RMI_DATA_CREATE_UNKNOWN_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    data: Address,
    ipa: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let old_rtte = RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx);
    let new_rtte = RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx);

    // Failure conditions
    (!AddrIsGranuleAligned(data) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegableDram(
        data,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, data).state != DELEGATED
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((realm.feat_lpa2 == FEATURE_FALSE && UInt(
        data,
    ) >= 281474976710656) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsProtected(
        ipa,
        realm,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (walk.level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    )) && (old_rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
        &&
    // Success conditions
    ((AddrIsGranuleAligned(data) && PaIsDelegableDram(data) && GranuleAt(old_s, data).state
        == DELEGATED && (realm.feat_lpa2 != FEATURE_FALSE || UInt(data) < 281474976710656)
        && AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(ipa) && AddrIsProtected(ipa, realm) && walk.level
        >= RMM_RTT_PAGE_LEVEL && old_rtte.state == UNASSIGNED) ==> (result.is_Ok() && GranuleAt(
        new_s,
        data,
    ).state == DATA && new_rtte.state == ASSIGNED && new_rtte.addr == data && new_rtte.attr_prot
        == MEMATTR_CACHEABLE && new_rtte.sh == SHAREABILITY_INNER))
}