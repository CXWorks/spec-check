pub open spec fn rmi_data_create_spec(rd: Address, data: Address, ipa: Address, src: Address, flags: RmiDataFlags, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, src) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, src, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE && (data as int) >= 0x1_0000_0000_0000) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level < RMM_RTT_PAGE_LEVEL
        ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level)))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != UNASSIGNED
        ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level)))
    && ((AddrIsGranuleAligned(old_s, src)
        && GranuleAccessPermitted(old_s, src, PAS_NS)
        && AddrIsGranuleAligned(old_s, data)
        && PaIsDelegableDram(old_s, data)
        && GranuleAt(old_s, data).state == DELEGATED
        && !(RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE && (data as int) >= 0x1_0000_0000_0000)
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
        && RealmAt(old_s, rd).state == REALM_NEW
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level >= RMM_RTT_PAGE_LEVEL
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED)
        ==> (result.is_Ok()
            && GranuleAt(new_s, data).state == DATA
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas == RAM
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr == data
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.attr_prot == MEMATTR_CACHEABLE
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.sh == SHAREABILITY_INNER
            && RealmAt(new_s, rd).measurements[0] == RimExtendData(old_s, RealmAt(old_s, rd), ipa, data, flags)))
}