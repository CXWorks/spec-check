pub open spec fn rmi_data_create_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level as int);
    let new_rtte = RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx as int);
    (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((realm_pre.feat_lpa2 == FEATURE_FALSE && (data as int) >= 0x1_0000_0000_0000) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && ((walk.level as int) < (RMM_RTT_PAGE_LEVEL as int) ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, src)
        && GranuleAccessPermitted(old_s, src, PAS_NS)
        && AddrIsGranuleAligned(old_s, data)
        && PaIsDelegableDram(old_s, data)
        && GranuleAt(old_s, data).state == DELEGATED
        && !(realm_pre.feat_lpa2 == FEATURE_FALSE && (data as int) >= 0x1_0000_0000_0000)
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, realm_pre)
        && realm_pre.state == REALM_NEW
        && (walk.level as int) == (RMM_RTT_PAGE_LEVEL as int)
        && walk.rtte.state == UNASSIGNED)
        ==> (result.is_Ok()
            && GranuleAt(new_s, data).state == DATA
            && new_rtte.state == ASSIGNED
            && new_rtte.ripas == RAM
            && new_rtte.addr == data
            && new_rtte.attr_prot == MEMATTR_CACHEABLE
            && new_rtte.sh == SHAREABILITY_INNER
            && RealmAt(new_s, rd).measurements[0] == RimExtendData(old_s, realm_pre, ipa, data, flags)))
}