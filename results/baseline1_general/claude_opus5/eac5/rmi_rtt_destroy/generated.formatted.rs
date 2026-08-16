pub open spec fn rmi_rtt_destroy_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    ipa: Address,
    level: u64,
    rtt: Address,
    top: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(
        old_s,
        rd,
        level as int,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(
        old_s,
        ipa,
        level as int - 1,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((ipa as int) >= (1u64 << Realm(
        old_s,
        rd,
    ).ipa_width) as int ==> ResultEqual(result, RMI_ERROR_INPUT)) && (RttWalk(old_s, rd, ipa).level
        < level as int - 1 ==> ResultEqual(result, RMI_ERROR_RTT) && top == RttSkipNonLiveEntries(
        old_s,
        Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
        RttWalk(old_s, rd, ipa).level as int,
        ipa,
    )) && (RttWalk(old_s, rd, ipa).rtte.state != TABLE ==> ResultEqual(result, RMI_ERROR_RTT) && top
        == RttSkipNonLiveEntries(
        old_s,
        Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
        RttWalk(old_s, rd, ipa).level as int,
        ipa,
    )) && (RttIsLive(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)) ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && top == ipa) && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(
        old_s,
        rd,
    ).state == RD && RttLevelIsValid(old_s, rd, level as int) && !RttLevelIsStarting(
        old_s,
        rd,
        level as int,
    ) && AddrIsRttLevelAligned(old_s, ipa, level as int - 1) && (ipa as int) < (1u64 << Realm(
        old_s,
        rd,
    ).ipa_width) as int && RttWalk(old_s, rd, ipa).level >= level as int - 1 && RttWalk(
        old_s,
        rd,
        ipa,
    ).rtte.state == TABLE && !RttIsLive(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtte.addr)))
        ==> result.is_Ok() && RttWalk(new_s, rd, ipa).rtte.state == UNASSIGNED && RttWalk(
        new_s,
        rd,
        ipa,
    ).rtte.ripas == DESTROYED && Granule(new_s, RttWalk(old_s, rd, ipa).rtte.addr).state
        == DELEGATED && rtt == RttWalk(old_s, rd, ipa).rtte.addr && top == RttSkipNonLiveEntries(
        old_s,
        Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
        RttWalk(old_s, rd, ipa).level as int,
        ipa,
    ))
}