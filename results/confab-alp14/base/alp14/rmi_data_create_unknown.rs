pub open spec fn rmi_data_create_unknown_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, rd: Address, data: Address, ipa: Address) -> bool {
    (!AddrIsGranuleAligned(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(old_s, data) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, data).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data as int) >= 0x1_0000_0000_0000)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL as int)
        ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)))
    && ((RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED)
        ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)))
    && ((AddrIsGranuleAligned(old_s, data)
        && PaIsDelegableDram(old_s, data)
        && GranuleAt(old_s, data).state == DELEGATED
        && !((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((data as int) >= 0x1_0000_0000_0000))
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level >= RMM_RTT_PAGE_LEVEL as int
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED)
        ==> (result.is_Ok()
            && GranuleAt(new_s, data).state == DATA
            && RttEntryAt(new_s,
                RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                RttEntryIndex(new_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)).state == ASSIGNED
            && RttEntryAt(new_s,
                RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                RttEntryIndex(new_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)).addr == data
            && RttEntryAt(new_s,
                RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                RttEntryIndex(new_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)).attr_prot == MEMATTR_CACHEABLE
            && RttEntryAt(new_s,
                RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                RttEntryIndex(new_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)).sh == SHAREABILITY_INNER))
}