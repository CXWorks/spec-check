pub open spec fn RMI_RTT_INIT_RIPAS_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    base: Address,
    top: Address,
    result: RmiCommandReturnCode,
    out_top: Address,
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_top = RttSkipEntriesIfNotState(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        walk.level,
        base,
        top,
        UNASSIGNED,
    );
    let realm = RealmAt(new_s, rd);

    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (UInt(top) <= UInt(base) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (!AddrIsProtected(old_s, ToAddress(UInt(top) - RMM_GRANULE_SIZE), realm_pre)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (realm_pre.state != REALM_NEW ==> ResultEqual(
        result,
        RMI_ERROR_REALM,
    )) && (!AddrIsRttLevelAligned(base, walk.level) ==> ResultEqual(
        result,
        RMI_ERROR_RTT(walk.level as int),
    )) && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
        && (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (UInt(base)
        == UInt(walk_top) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))) || (result
        == RMI_SUCCESS && RttEntriesInRangeRipas(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        walk.level,
        base,
        walk_top,
        RAM,
    ) && realm.measurements[0] == RimExtendRipas(old_s, realm_pre, base, walk_top, walk.level)
        && out_top == walk_top)
}