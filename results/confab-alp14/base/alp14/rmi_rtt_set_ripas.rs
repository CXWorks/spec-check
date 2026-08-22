pub open spec fn rmi_rtt_set_ripas_spec(result: Result<(), RmiStatusCode>, out_top: Address, rd: Address, rec_ptr: Address, base: Address, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RecAt(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (RecAt(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && ((top as int) <= (base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != RecAt(old_s, rec_ptr).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((top as int) > (RecAt(old_s, rec_ptr).ripas_top as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)
            && RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value)
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((base as int) == (RttSkipEntriesWithRipas(old_s,
                RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int,
                base, top,
                RecAt(old_s, rec_ptr).ripas_value == RAM && RecAt(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED) as int)
            && RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RecAt(old_s, rec_ptr).ripas_value
        ==> ResultEqual(result, RMI_ERROR_RTT))
    && (AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, rec_ptr)
        && PaIsDelegable(old_s, rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == REC
        && RecAt(old_s, rec_ptr).state != REC_RUNNING
        && RecAt(old_s, rec_ptr).owner == rd
        && (top as int) > (base as int)
        && base == RecAt(old_s, rec_ptr).ripas_addr
        && (top as int) <= (RecAt(old_s, rec_ptr).ripas_top as int)
        && AddrIsGranuleAligned(old_s, top)
        && (AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int)
            || RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RecAt(old_s, rec_ptr).ripas_value)
        && ((base as int) != (RttSkipEntriesWithRipas(old_s,
                RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int,
                base, top,
                RecAt(old_s, rec_ptr).ripas_value == RAM && RecAt(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED) as int)
            || RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RecAt(old_s, rec_ptr).ripas_value)
        && !AddrRangeIsAuxLive(old_s, base, top, RealmAt(old_s, rd)))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(new_s,
                RttAt(new_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int,
                base,
                RttSkipEntriesWithRipas(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int,
                    base, top,
                    RecAt(old_s, rec_ptr).ripas_value == RAM && RecAt(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED),
                RecAt(old_s, rec_ptr).ripas_value)
            && RecAt(new_s, rec_ptr).ripas_addr == MinAddress(top,
                RttSkipEntriesWithRipas(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int,
                    base, top,
                    RecAt(old_s, rec_ptr).ripas_value == RAM && RecAt(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED))
            && out_top == MinAddress(top,
                RttSkipEntriesWithRipas(old_s,
                    RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).rtt_addr),
                    RttWalk(old_s, RealmAt(old_s, rd), base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int).level as int,
                    base, top,
                    RecAt(old_s, rec_ptr).ripas_value == RAM && RecAt(old_s, rec_ptr).ripas_destroyed != CHANGE_DESTROYED))))
}