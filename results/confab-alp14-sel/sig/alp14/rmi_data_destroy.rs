pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level < RMM_RTT_PAGE_LEVEL
        ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int))
            && top == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr), RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int, ipa))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != ASSIGNED
        ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int))
            && top == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr), RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int, ipa))
    && (AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0)))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level == RMM_RTT_PAGE_LEVEL
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED
        && !AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)))
        ==> result.is_Ok()
            && GranuleAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr).state == DELEGATED
            && RttEntryAt(new_s, RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int)).state == UNASSIGNED
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas == RAM
                ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int)).ripas == DESTROYED)
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas != RAM
                ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int)).ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.ripas)
            && data == RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.addr
            && top == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr), RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int, ipa))
}