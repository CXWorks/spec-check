pub open spec fn rmi_rtt_destroy_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    ipa: Address,
    level: Int64,
    rtt: Address,
    top: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let walk = RttWalk(old_s, rd, ipa);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(
        old_s,
        Rtt(old_s, walk.rtt_addr),
        walk.level,
        ipa,
    );
    (!AddrIsGranuleAligned(old_s, rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, rd, level as int)
            || RttLevelIsStarting(old_s, rd, level as int))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (1int << (Realm(old_s, rd).ipa_width as int))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < (level - 1) as int
        ==> ResultEqual(result, RMI_ERROR_RTT) && top == walk_top)
    && (walk.rtte.state != TABLE
        ==> ResultEqual(result, RMI_ERROR_RTT) && top == walk_top)
    && (RttIsLive(old_s, Rtt(old_s, walk.rtte.addr))
        ==> ResultEqual(result, RMI_ERROR_RTT) && top == ipa)
    && (AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsValid(old_s, rd, level as int)
        && !RttLevelIsStarting(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, (level - 1) as int)
        && (ipa as int) < (1int << (Realm(old_s, rd).ipa_width as int))
        && walk.level == (level - 1) as int
        && walk.rtte.state == TABLE
        && !RttIsLive(old_s, Rtt(old_s, walk.rtte.addr))
        ==> result.is_Ok()
            && RttEntry(new_s, walk.rtt_addr, entry_idx).state == UNASSIGNED
            && RttEntry(new_s, walk.rtt_addr, entry_idx).ripas == DESTROYED
            && Granule(new_s, walk.rtte.addr).state == DELEGATED
            && rtt == walk.rtte.addr
            && top == walk_top)
}