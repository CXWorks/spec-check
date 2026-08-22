pub open spec fn rmi_data_destroy_spec(result: Result<(), RmiStatusCode>, data: Address, top: Address, rd: Address, ipa: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level < RMM_RTT_PAGE_LEVEL as int
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int))
            && top == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int, ipa)))
    && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED
        ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int))
            && top == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int, ipa)))
    && (AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0)))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level >= RMM_RTT_PAGE_LEVEL as int
        && RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED
        && !AddrIsAuxLive(old_s, ipa, RealmAt(old_s, rd)))
        ==> (result.is_Ok()
            && GranuleAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr).state == DELEGATED
            && RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.state == UNASSIGNED
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RAM
                ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == DESTROYED)
            && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RAM
                ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
            && data == RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.addr
            && top == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttWalk(old_s, RealmAt(old_s, rd), ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int, ipa)))
}