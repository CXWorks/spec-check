pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top = RttSkipEntriesIfNotState(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, top, UNASSIGNED);
    let rd_ok = AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD;
    let top_aligned = AddrIsGranuleAligned(old_s, top);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && !AddrIsProtected(old_s, ToAddress((top as int) - (RMM_GRANULE_SIZE as int)), realm_pre)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && realm_pre.state != REALM_NEW) ==> ResultEqual(result, RMI_ERROR_REALM(0)))
    && ((rd_ok && !AddrIsRttLevelAligned(old_s, base, walk.level as int)) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((rd_ok && walk.rtte.state != UNASSIGNED) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (!top_aligned ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((rd_ok && top_aligned && walk_top == base) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (result.is_Err() ==> new_s == old_s)
    && ((rd_ok
        && top > base
        && AddrIsProtected(old_s, ToAddress((top as int) - (RMM_GRANULE_SIZE as int)), realm_pre)
        && realm_pre.state == REALM_NEW
        && AddrIsRttLevelAligned(old_s, base, walk.level as int)
        && walk.rtte.state == UNASSIGNED
        && top_aligned
        && walk_top != base)
        ==> (result.is_Ok()
            && out_top == walk_top
            && RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level as int, base, walk_top, RAM)
            && RealmAt(new_s, rd).measurements[0] == RimExtendRipas(old_s, realm_pre, base, walk_top, walk.level as int)))
}
