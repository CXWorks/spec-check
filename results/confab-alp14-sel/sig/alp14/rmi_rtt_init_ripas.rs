pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top = RttSkipEntriesIfNotState(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, base, top, UNASSIGNED);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((top as int) <= (base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ToAddress((top as int) - (RMM_GRANULE_SIZE as int)), realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
    && (!AddrIsRttLevelAligned(old_s, base, walk.level as int) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((base as int) == (walk_top as int) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((AddrIsGranuleAligned(old_s, rd)
         && PaIsDelegable(old_s, rd)
         && GranuleAt(old_s, rd).state == RD
         && (top as int) > (base as int)
         && AddrIsProtected(old_s, ToAddress((top as int) - (RMM_GRANULE_SIZE as int)), realm_pre)
         && AddrIsGranuleAligned(old_s, top)
         && realm_pre.state == REALM_NEW
         && AddrIsRttLevelAligned(old_s, base, walk.level as int)
         && walk.rtte.state == UNASSIGNED
         && (base as int) != (walk_top as int))
        ==> (result.is_Ok()
             && RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level as int, base, walk_top, RAM)
             && RealmAt(new_s, rd).measurements[0] == RimExtendRipas(old_s, realm_pre, base, walk_top, walk.level as int)
             && out_top == walk_top))
}