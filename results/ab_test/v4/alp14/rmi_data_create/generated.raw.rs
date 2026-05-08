pub open spec fn RMI_DATA_CREATE_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    data: Address,
    ipa: Address,
    src: Address,
    flags: RmiDataFlags,
    result: RmiCommandReturnCode
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let realm = RealmAt(new_s, rd);
    
    (
        (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        ((realm_pre.feat_lpa2 == FEATURE_FALSE && (data as int) >= (1 << 48)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!AddrIsProtected(old_s, ipa, realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM)) &&
        (walk.level < RMM_RTT_PAGE_LEVEL as int ==> ResultEqual(result, RMI_ERROR_RTT)) &&
        (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT)) &&
        (
            result.is_Ok() ==>
            GranuleAt(new_s, data).state == DATA &&
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED &&
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == RAM &&
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == data &&
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_prot == MEMATTR_CACHEABLE &&
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).sh == SHAREABILITY_INNER &&
            realm.measurements[0] == RimExtendData(old_s, realm_pre, ipa, data, flags)
        )
    )
}