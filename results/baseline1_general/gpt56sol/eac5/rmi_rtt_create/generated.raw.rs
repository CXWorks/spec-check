pub open spec fn rmi_rtt_create_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: Int64,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, rd, level as int)
        || RttLevelIsStarting(old_s, rd, level as int)) ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1) ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa >= (1u64 << Realm(old_s, rd).ipa_width) ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rtt) ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rtt) ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rtt).state != DELEGATED ==>
        ResultEqual(result, RMI_ERROR_INPUT))
    && (
        AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsValid(old_s, rd, level as int)
        && !RttLevelIsStarting(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1)
        && ipa < (1u64 << Realm(old_s, rd).ipa_width)
        && AddrIsGranuleAligned(old_s, rtt)
        && PaIsDelegable(old_s, rtt)
        && Granule(old_s, rtt).state == DELEGATED
        && RttWalk(old_s, rd, ipa).level < (level as int) - 1
        ==> ResultEqual(
            result,
            RMI_ERROR_RTT_AUX(RttWalk(old_s, rd, ipa).level as int),
        )
    )
    && (
        AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsValid(old_s, rd, level as int)
        && !RttLevelIsStarting(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1)
        && ipa < (1u64 << Realm(old_s, rd).ipa_width)
        && AddrIsGranuleAligned(old_s, rtt)
        && PaIsDelegable(old_s, rtt)
        && Granule(old_s, rtt).state == DELEGATED
        && RttWalk(old_s, rd, ipa).rtte.state == TABLE
        ==> ResultEqual(
            result,
            RMI_ERROR_RTT_AUX(RttWalk(old_s, rd, ipa).level as int),
        )
    )
    && (
        AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsValid(old_s, rd, level as int)
        && !RttLevelIsStarting(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1)
        && ipa < (1u64 << Realm(old_s, rd).ipa_width)
        && AddrIsGranuleAligned(old_s, rtt)
        && PaIsDelegable(old_s, rtt)
        && Granule(old_s, rtt).state == DELEGATED
        && RttWalk(old_s, rd, ipa).level >= (level as int) - 1
        && RttWalk(old_s, rd, ipa).rtte.state != TABLE
        ==> (
            result.is_Ok()
            && Granule(new_s, rtt).state == RTT
            && RttEntry(
                new_s,
                RttWalk(old_s, rd, ipa).rtt_addr,
                RttEntryIndex(
                    old_s,
                    ipa,
                    RttWalk(old_s, rd, ipa).level as int,
                ),
            ).state == TABLE
            && RttEntry(
                new_s,
                RttWalk(old_s, rd, ipa).rtt_addr,
                RttEntryIndex(
                    old_s,
                    ipa,
                    RttWalk(old_s, rd, ipa).level as int,
                ),
            ).addr == rtt
            && (
                AddrIsProtected(old_s, ipa, Realm(old_s, rd))
                ==> RttAllEntriesRipas(
                    new_s,
                    Rtt(new_s, rtt),
                    RttWalk(old_s, rd, ipa).rtte.ripas,
                )
            )
            && RttAllEntriesState(
                new_s,
                Rtt(new_s, rtt),
                RttWalk(old_s, rd, ipa).rtte.state,
            )
            && (
                RttWalk(old_s, rd, ipa).rtte.state != UNASSIGNED
                && RttWalk(old_s, rd, ipa).rtte.state != UNASSIGNED_NS
                ==> RttAllEntriesContiguous(
                    new_s,
                    Rtt(new_s, rtt),
                    RttWalk(old_s, rd, ipa).rtte.addr,
                    level as int,
                )
            )
        )
    )
}