pub open spec fn rmi_rtt_unmap_unprotected_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    ipa: Address,
    level: Int64,
    top: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let walk = RttWalk(old_s, rd, ipa);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level as int);
    let walk_top = RttSkipNonLiveEntries(old_s, Rtt(old_s, walk.rtt_addr), walk.level as int, ipa);
    (!AddrIsGranuleAligned(old_s, rd) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0 && new_s
        == old_s)) && (!PaIsDelegable(old_s, rd) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top
        == 0 && new_s == old_s)) && (Granule(old_s, rd).state != RD ==> (ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ) && top == 0 && new_s == old_s)) && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> (
    ResultEqual(result, RMI_ERROR_INPUT) && top == 0 && new_s == old_s)) && (!AddrIsRttLevelAligned(
        old_s,
        ipa,
        level as int,
    ) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0 && new_s == old_s)) && (((ipa as int)
        >= (1 << (Realm(old_s, rd).ipa_width as int)) || AddrIsProtected(
        old_s,
        ipa,
        Realm(old_s, rd),
    )) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0 && new_s == old_s)) && (walk.level
        < level as int ==> (ResultEqual(result, RMI_ERROR_RTT_AUX(walk.level as int)) && top
        == walk_top && new_s == old_s)) && (walk.rtte.state != ASSIGNED_NS ==> (ResultEqual(
        result,
        RMI_ERROR_RTT_AUX(walk.level as int),
    ) && top == walk_top && new_s == old_s)) && (AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(
        old_s,
        rd,
    ) && Granule(old_s, rd).state == RD && RttLevelIsBlockOrPage(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, level as int) && (ipa as int) < (1 << (Realm(
        old_s,
        rd,
    ).ipa_width as int)) && !AddrIsProtected(old_s, ipa, Realm(old_s, rd)) && walk.level
        == level as int && walk.rtte.state == ASSIGNED_NS ==> (result.is_Ok() && top == walk_top
        && RttEntry(new_s, walk.rtt_addr, entry_idx as int).state == UNASSIGNED_NS))
}