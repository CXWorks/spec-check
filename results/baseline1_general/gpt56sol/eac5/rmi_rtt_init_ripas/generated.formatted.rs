pub open spec fn rmi_rtt_init_ripas_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    base: Address,
    top: Address,
    out_top: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = Realm(old_s, rd);
    let walk = RttWalk(old_s, rd, base);
    let walk_top = RttSkipEntriesWithRipas(
        old_s,
        Rtt(old_s, walk.rtt_addr),
        walk.level,
        base,
        top,
        false,
    );
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsProtected(
        old_s,
        ToAddress((top as int) - RMM_GRANULE_SIZE),
        realm,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (realm.state != REALM_NEW ==> ResultEqual(
        result,
        RMI_ERROR_REALM,
    )) && (!AddrIsRttLevelAligned(old_s, base, walk.level) ==> ResultEqual(result, RMI_ERROR_RTT))
        && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT)) && (
    !AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((top
        < RttUpperBound(old_s, base, walk.level, realm.ipa_width as int) && RttEntryHasRipas(
        old_s,
        RttEntry(old_s, walk.rtt_addr, RttEntryIndex(old_s, top, walk.level)),
    ) && !AddrIsRttLevelAligned(old_s, top, walk.level)) ==> ResultEqual(result, RMI_ERROR_RTT))
        && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state
        == RD && top > base && AddrIsProtected(
        old_s,
        ToAddress((top as int) - RMM_GRANULE_SIZE),
        realm,
    ) && realm.state == REALM_NEW && AddrIsRttLevelAligned(old_s, base, walk.level)
        && walk.rtte.state == UNASSIGNED && AddrIsGranuleAligned(old_s, top) && !(top
        < RttUpperBound(old_s, base, walk.level, realm.ipa_width as int) && RttEntryHasRipas(
        old_s,
        RttEntry(old_s, walk.rtt_addr, RttEntryIndex(old_s, top, walk.level)),
    ) && !AddrIsRttLevelAligned(old_s, top, walk.level))) ==> (result.is_Ok()
        && RttEntriesInRangeRipas(new_s, Rtt(new_s, walk.rtt_addr), walk.level, base, walk_top, RAM)
        && Realm(new_s, rd).measurements[0] == RimExtendRipas(
        old_s,
        realm,
        base,
        walk_top,
        walk.level,
    ) && out_top == walk_top))
}