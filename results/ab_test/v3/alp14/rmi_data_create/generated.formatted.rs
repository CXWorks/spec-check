pub open spec fn rmi_data_create_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    data: Address,
    ipa: Address,
    src: Address,
    flags: RmiDataFlags,
    old_s: S,
    new_s: S,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);

    (!AddrIsGranuleAligned(src) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(data) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegableDram(
        old_s,
        data,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, data).state != DELEGATED
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((realm_pre.feat_lpa2 == FEATURE_FALSE) && (
    UInt(data) >= (1 << 48))) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(old_s, rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsProtected(
        old_s,
        ipa,
        realm_pre,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (realm_pre.state != REALM_NEW ==> ResultEqual(
        result,
        RMI_ERROR_REALM,
    )) && (walk.level < RMM_RTT_PAGE_LEVEL ==> result.is_Err() && result.get_Err_0()
        == RMI_ERROR_RTT) && (walk.rtte.state != UNASSIGNED ==> result.is_Err()
        && result.get_Err_0() == RMI_ERROR_RTT) && ((AddrIsGranuleAligned(src)
        && GranuleAccessPermitted(old_s, src, PAS_NS) && AddrIsGranuleAligned(data)
        && PaIsDelegableDram(old_s, data) && GranuleAt(old_s, data).state == DELEGATED && !((
    realm_pre.feat_lpa2 == FEATURE_FALSE) && (UInt(data) >= (1 << 48))) && AddrIsGranuleAligned(rd)
        && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && AddrIsGranuleAligned(ipa)
        && AddrIsProtected(old_s, ipa, realm_pre) && realm_pre.state == REALM_NEW && walk.level
        == RMM_RTT_PAGE_LEVEL && walk.rtte.state == UNASSIGNED) ==> (result.is_Ok() && GranuleAt(
        new_s,
        data,
    ).state == DATA && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED
        && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == RAM && RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx,
    ).addr == data && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_prot
        == MEMATTR_CACHEABLE && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).sh
        == SHAREABILITY_INNER && RealmAt(new_s, rd).measurements[0] == RimExtendData(
        old_s,
        realm_pre,
        ipa,
        data,
        flags,
    )))
}