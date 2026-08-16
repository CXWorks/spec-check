pub open spec fn rmi_data_destroy_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    ipa: Address,
    data: Address,
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
    ) && top == 0 && new_s == old_s)) && (!AddrIsGranuleAligned(old_s, ipa) ==> (ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ) && top == 0 && new_s == old_s)) && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> (
    ResultEqual(result, RMI_ERROR_INPUT) && top == 0 && new_s == old_s)) && ((AddrIsGranuleAligned(
        old_s,
        rd,
    ) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state == RD && AddrIsGranuleAligned(
        old_s,
        ipa,
    ) && AddrIsProtected(old_s, ipa, Realm(old_s, rd)) && walk.level < RMM_RTT_PAGE_LEVEL) ==> (
    ResultEqual(result, RMI_ERROR_RTT) && top == walk_top && new_s == old_s)) && ((
    AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa) && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && walk.level == RMM_RTT_PAGE_LEVEL && walk.rtte.state != ASSIGNED) ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && top == walk_top && new_s == old_s)) && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(
        old_s,
        rd,
    ) && Granule(old_s, rd).state == RD && AddrIsGranuleAligned(old_s, ipa) && AddrIsProtected(
        old_s,
        ipa,
        Realm(old_s, rd),
    ) && walk.level == RMM_RTT_PAGE_LEVEL && walk.rtte.state == ASSIGNED && walk.rtte.ripas == RAM)
        ==> (result.is_Ok() && Granule(new_s, walk.rtte.addr).state == DELEGATED && RttEntry(
        new_s,
        walk.rtt_addr,
        entry_idx,
    ).state == UNASSIGNED && RttEntry(new_s, walk.rtt_addr, entry_idx).ripas == DESTROYED && data
        == walk.rtte.addr && top == walk_top))
}