pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk_(old_s, rd, base, RMM_RTT_PAGE_LEVEL as int);
    let walk_top = RttSkipEntriesIfNotState(RttAt(walk.rtt_addr), walk.level, base, top, UNASSIGNED);
    let realm = realm_pre;
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(ToAddress(top - RMM_GRANULE_SIZE), realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (!AddrIsRttLevelAligned(base, walk.level) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base == walk_top ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (ResultEqual(result, RMI_SUCCESS) ==> (out_top == walk_top))
    && (ResultEqual(result, RMI_SUCCESS) ==> RttEntriesInRangeRipas(RttAt(walk.rtt_addr), walk.level, base, walk_top, RAM))
    && (ResultEqual(result, RMI_SUCCESS) ==> new_s.CurrentRealm().measurements[0] == RimExtendRipas(realm_pre, base, walk_top, walk.level))
}