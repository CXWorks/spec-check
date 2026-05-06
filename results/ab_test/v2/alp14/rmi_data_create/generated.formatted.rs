pub open spec fn rmi_data_create_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    data: Address,
    ipa: Address,
    src: Address,
    flags: RmiDataFlags,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);

    // Failure conditions (in priority order)
    (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        data,
    ).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((realm_pre.feat_lpa2
        == FEATURE_FALSE && UInt(data) >= 0x1_0000_0000_0000) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rd,
    ).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(old_s, ipa)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsProtected(old_s, ipa, realm_pre)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (realm_pre.state != REALM_NEW ==> ResultEqual(
        result,
        RMI_ERROR_REALM,
    )) && (walk.level < RMM_RTT_PAGE_LEVEL ==> ResultEqual(result, RMI_ERROR_RTT)) && (RttEntryAt(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        entry_idx,
    ).state != UNASSIGNED ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    ))
    // Success conditions
     && (AddrIsGranuleAligned(old_s, src) && GranuleAccessPermitted(old_s, src, PAS_NS)
        && AddrIsGranuleAligned(old_s, data) && PaIsDelegableDram(old_s, data) && GranuleAt(
        old_s,
        data,
    ).state == DELEGATED && (realm_pre.feat_lpa2 == FEATURE_TRUE || UInt(data) < 0x1_0000_0000_0000)
        && AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state
        == RD && AddrIsGranuleAligned(old_s, ipa) && AddrIsProtected(old_s, ipa, realm_pre)
        && realm_pre.state == REALM_NEW && walk.level == RMM_RTT_PAGE_LEVEL && RttEntryAt(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        entry_idx,
    ).state == UNASSIGNED ==> (result == RMI_SUCCESS && GranuleAt(new_s, data).state == DATA
        && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED
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