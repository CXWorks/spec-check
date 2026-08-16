pub open spec fn rmi_rtt_create_spec(
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: UInt64,
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
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((ipa as int) >= ((1u64 << (Realm(
        old_s,
        rd,
    ).ipa_width as u64)) as int) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rtt,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rtt).state != DELEGATED
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE
        && (rtt as int) >= 0x1_0000_0000_0000) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    RttWalk(old_s, rd, ipa).level < level as int - 1 ==> ResultEqual(
        result,
        RMI_ERROR_RTT(level as int),
    )) && (RttWalk(old_s, rd, ipa).rtte.state == TABLE ==> ResultEqual(
        result,
        RMI_ERROR_RTT(level as int),
    )) && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state
        == RD && RttLevelIsValid(old_s, rd, level as int) && !RttLevelIsStarting(
        old_s,
        rd,
        level as int,
    ) && AddrIsRttLevelAligned(old_s, ipa, level as int - 1) && (ipa as int) < ((1u64 << (Realm(
        old_s,
        rd,
    ).ipa_width as u64)) as int) && AddrIsGranuleAligned(old_s, rtt) && PaIsDelegable(old_s, rtt)
        && Granule(old_s, rtt).state == DELEGATED && !(Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE
        && (rtt as int) >= 0x1_0000_0000_0000) && RttWalk(old_s, rd, ipa).level >= level as int - 1
        && RttWalk(old_s, rd, ipa).rtte.state != TABLE) ==> (result.is_Ok() && Granule(
        new_s,
        rtt,
    ).state == RTT && RttWalk(new_s, rd, ipa).rtte.state == TABLE && RttWalk(
        new_s,
        rd,
        ipa,
    ).rtte.addr == rtt && (AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> RttAllEntriesRipas(
        new_s,
        Rtt(new_s, rtt),
        RttWalk(old_s, rd, ipa).rtte.ripas,
    )) && RttAllEntriesState(new_s, Rtt(new_s, rtt), RttWalk(old_s, rd, ipa).rtte.state) && ((
    RttWalk(old_s, rd, ipa).rtte.state != UNASSIGNED && RttWalk(old_s, rd, ipa).rtte.state
        != UNASSIGNED_NS) ==> RttAllEntriesContiguous(
        new_s,
        Rtt(new_s, rtt),
        RttWalk(old_s, rd, ipa).rtte.addr,
        level as int,
    ))))
}