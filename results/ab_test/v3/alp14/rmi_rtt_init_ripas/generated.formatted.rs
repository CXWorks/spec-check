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
    let walk_rtt = RttAt(old_s, walk.rtt_addr);
    let walk_top = RttSkipEntriesIfNotState(old_s, walk_rtt, walk.level, base, top, UNASSIGNED);
    let realm = RealmAt(new_s, rd);

    // Failure conditions with ordering constraints
    (
    // rd_align: !AddrIsGranuleAligned(rd) ==> RMI_ERROR_INPUT
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // rd_bound: !PaIsDelegable(rd) ==> RMI_ERROR_INPUT
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // rd_state: GranuleAt(rd).state != RD ==> RMI_ERROR_INPUT
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // size_valid: UInt(top) <= UInt(base) ==> RMI_ERROR_INPUT
    (UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // top_bound: !AddrIsProtected(ToAddress(UInt(top) - RMM_GRANULE_SIZE), realm_pre) ==> RMI_ERROR_INPUT
    (!AddrIsProtected(old_s, ToAddress(UInt(top) - RMM_GRANULE_SIZE), realm_pre) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
        &&
    // top_gran_align: !AddrIsGranuleAligned(top) ==> RMI_ERROR_INPUT
    (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // realm_state: realm_pre.state != REALM_NEW ==> RMI_ERROR_REALM
    (realm_pre.state != REALM_NEW ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_REALM)
        &&
    // base_align: !AddrIsRttLevelAligned(base, walk.level) ==> RMI_ERROR_RTT
    (!AddrIsRttLevelAligned(old_s, base, walk.level) ==> result.is_Err() && result.get_Err_0()
        == RMI_ERROR_RTT)
        &&
    // rtte_state: walk.rtte.state != UNASSIGNED ==> RMI_ERROR_RTT
    (walk.rtte.state != UNASSIGNED ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT)
        &&
    // no_progress: UInt(base) == UInt(walk_top) ==> RMI_ERROR_RTT
    (UInt(base) == UInt(walk_top) ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT)
        &&
    // Ordering constraints: [rd_bound, rd_state] < [realm_state]
    ((!PaIsDelegable(rd) || GranuleAt(old_s, rd).state != RD) ==> realm_pre.state == REALM_NEW)
        &&
    // Ordering constraints: [rd_bound, rd_state] < [base_align, rtte_state]
    ((!PaIsDelegable(rd) || GranuleAt(old_s, rd).state != RD) ==> (AddrIsRttLevelAligned(
        old_s,
        base,
        walk.level,
    ) && walk.rtte.state == UNASSIGNED))
        &&
    // Ordering constraints: [rd_bound, rd_state] < [no_progress]
    ((!PaIsDelegable(rd) || GranuleAt(old_s, rd).state != RD) ==> UInt(base) != UInt(walk_top))
        &&
    // Ordering constraints: [top_gran_align] < [no_progress]
    (!AddrIsGranuleAligned(top) ==> UInt(base) != UInt(walk_top))
        &&
    // Success conditions
    (result.is_Ok() ==> (RttEntriesInRangeRipas(new_s, walk_rtt, walk.level, base, walk_top, RAM)
        && realm.measurements[[0]] == RimExtendRipas(old_s, realm_pre, base, walk_top, walk.level)
        && out_top == walk_top)))
}