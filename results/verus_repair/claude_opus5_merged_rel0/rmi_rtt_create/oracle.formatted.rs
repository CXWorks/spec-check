pub open spec fn rmi_rtt_create_spec(
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: Int64,
    result: Result<(), RmiStatusCode>,
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
        level - 1 as int,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(old_s, rtt)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(old_s, rtt) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (Granule(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((
    Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2 ^ 48)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (RttWalk_(old_s, rd, ipa, level - 1 as int).level < level - 1 ==> ResultEqual(
        result,
        RMI_ERROR_RTT(RttWalk_(new_s, rd, ipa, level - 1 as int).level as int),
    )) && (RttWalk_(old_s, rd, ipa, level - 1 as int).rtte.state == TABLE ==> ResultEqual(
        result,
        RMI_ERROR_RTT(RttWalk_(new_s, rd, ipa, level - 1 as int).level as int),
    )) && (result.is_Ok() ==> Granule(new_s, rtt).state == RTT) && (result.is_Ok() ==> RttWalk_(
        new_s,
        rd,
        ipa,
        level - 1 as int,
    ).rtte.state == TABLE) && (result.is_Ok() ==> RttWalk_(
        new_s,
        rd,
        ipa,
        level - 1 as int,
    ).rtte.addr == rtt) && (result.is_Ok() && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        ==> RttAllEntriesRipas(
        new_s,
        Rtt(new_s, rtt),
        RttWalk_(new_s, rd, ipa, level - 1 as int).rtte.ripas,
    )) && (result.is_Ok() ==> RttAllEntriesState(
        new_s,
        Rtt(new_s, rtt),
        RttWalk_(new_s, rd, ipa, level - 1 as int).rtte.state,
    )) && (result.is_Ok() && (RttWalk_(old_s, rd, ipa, level - 1 as int).rtte.state != UNASSIGNED
        && RttWalk_(old_s, rd, ipa, level - 1 as int).rtte.state != UNASSIGNED_NS)
        ==> RttAllEntriesContiguous(
        new_s,
        Rtt(new_s, rtt),
        RttWalk_(new_s, rd, ipa, level - 1 as int).rtte.addr,
        level as int,
    )) && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && !(Granule(
        old_s,
        rd,
    ).state != RD) && !((!RttLevelIsValid(old_s, rd, level as int) || RttLevelIsStarting(
        old_s,
        rd,
        level as int,
    ))) && AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) && !((ipa) >= pow2(
        Realm(old_s, rd).ipa_width as nat,
    )) && AddrIsGranuleAligned(old_s, rtt) && PaIsDelegable(old_s, rtt) && !(Granule(
        old_s,
        rtt,
    ).state != DELEGATED) && !(((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2 ^ 48)))
        && !(RttWalk_(old_s, rd, ipa, level - 1 as int).level < level - 1) && !(RttWalk_(
        old_s,
        rd,
        ipa,
        level - 1 as int,
    ).rtte.state == TABLE)) ==> result.is_Ok()) && (result.is_Err() ==> Granule(new_s, rtt).state
        == Granule(old_s, rtt).state) && (result.is_Err() ==> RttWalk_(
        new_s,
        rd,
        ipa,
        level - 1 as int,
    ).rtte.state == RttWalk_(old_s, rd, ipa, level - 1 as int).rtte.state) && (result.is_Err()
        ==> RttWalk_(new_s, rd, ipa, level - 1 as int).rtte.addr == RttWalk_(
        old_s,
        rd,
        ipa,
        level - 1 as int,
    ).rtte.addr) && (RttWalk_(new_s, rd, ipa, level - 1 as int).rtte.ripas == RttWalk_(
        old_s,
        rd,
        ipa,
        level - 1 as int,
    ).rtte.ripas)
}