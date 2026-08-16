pub open spec fn rmi_rtt_unmap_unprotected_spec(
    rd: Address,
    ipa: Address,
    level: Int64,
    result: Result<(), RmiStatusCode>,
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
    )) && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(
        old_s,
        ipa,
        Realm(old_s, rd),
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (RttWalk_(old_s, rd, ipa, level as int).level
        < level ==> (ResultEqual(
        result,
        RMI_ERROR_RTT(RttWalk_(new_s, rd, ipa, level as int).level as int),
    ) && (top == RttSkipNonLiveEntries(
        new_s,
        Rtt(new_s, RttWalk_(new_s, rd, ipa, level as int).rtt_addr),
        RttWalk_(new_s, rd, ipa, level as int).level,
        ipa,
    )))) && (RttWalk_(old_s, rd, ipa, level as int).rtte.state != ASSIGNED_NS ==> (ResultEqual(
        result,
        RMI_ERROR_RTT(RttWalk_(new_s, rd, ipa, level as int).level as int),
    ) && (top == RttSkipNonLiveEntries(
        new_s,
        Rtt(new_s, RttWalk_(new_s, rd, ipa, level as int).rtt_addr),
        RttWalk_(new_s, rd, ipa, level as int).level,
        ipa,
    )))) && (result.is_Ok() ==> RttWalk_(new_s, rd, ipa, level as int).rtte.state == UNASSIGNED_NS)
        && (result.is_Ok() ==> top == RttSkipNonLiveEntries(
        new_s,
        Rtt(new_s, RttWalk_(new_s, rd, ipa, level as int).rtt_addr),
        RttWalk_(new_s, rd, ipa, level as int).level,
        ipa,
    )) && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && !(Granule(
        old_s,
        rd,
    ).state != RD) && RttLevelIsBlockOrPage(old_s, rd, level as int) && AddrIsRttLevelAligned(
        old_s,
        ipa,
        level as int,
    ) && !(((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) || AddrIsProtected(
        old_s,
        ipa,
        Realm(old_s, rd),
    ))) && !(RttWalk_(old_s, rd, ipa, level as int).level < level) && !(RttWalk_(
        old_s,
        rd,
        ipa,
        level as int,
    ).rtte.state != ASSIGNED_NS)) ==> result.is_Ok()) && (result.is_Err() ==> RttWalk_(
        new_s,
        rd,
        ipa,
        level as int,
    ).rtte.state == RttWalk_(old_s, rd, ipa, level as int).rtte.state) && (RttWalk_(
        new_s,
        rd,
        ipa,
        level as int,
    ).rtte.ripas == RttWalk_(old_s, rd, ipa, level as int).rtte.ripas)
}