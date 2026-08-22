pub open spec fn rmi_rtt_init_ripas_spec(result: Result<(), RmiStatusCode>, out_top: Address, rd: Address, base: Address, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((top as int) <= (base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ToAddress((top as int) - RMM_GRANULE_SIZE), RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int)
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state != UNASSIGNED
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((base as int) == (RttSkipEntriesIfNotState(old_s,
            RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr),
            RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int,
            base, top, UNASSIGNED) as int)
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && (top as int) > (base as int)
        && AddrIsProtected(old_s, ToAddress((top as int) - RMM_GRANULE_SIZE), RealmAt(old_s, rd))
        && AddrIsGranuleAligned(old_s, top)
        && RealmAt(old_s, rd).state == REALM_NEW
        && AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int)
        && RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED
        && (base as int) != (RttSkipEntriesIfNotState(old_s,
                RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr),
                RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int,
                base, top, UNASSIGNED) as int))
        ==> result.is_Ok()
            && RttEntriesInRangeRipas(new_s,
                RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr),
                RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int,
                base,
                RttSkipEntriesIfNotState(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int,
                    base, top, UNASSIGNED),
                RAM)
            && RealmAt(new_s, rd).measurements[0] == RimExtendRipas(old_s, RealmAt(old_s, rd), base,
                RttSkipEntriesIfNotState(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int,
                    base, top, UNASSIGNED),
                RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int)
            && out_top == RttSkipEntriesIfNotState(old_s,
                RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).rtt_addr),
                RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY).level as int,
                base, top, UNASSIGNED))
}