pub open spec fn rmi_rtt_unmap_unprotected_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, level: Int64, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RttLevelIsBlockOrPage(old_s, rd, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((ipa as int) >= (1 << Realm(old_s, rd).ipa_width)
        || AddrIsProtected(old_s, ipa, Realm(old_s, rd)))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, rd, ipa).level < level as int
        ==> (ResultEqual(result, RMI_ERROR_RTT)
            && top == RttSkipNonLiveEntries(
                old_s,
                Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
                RttWalk(old_s, rd, ipa).level,
                ipa
            )))
    && (RttWalk(old_s, rd, ipa).rtte.state != ASSIGNED_NS
        ==> (ResultEqual(result, RMI_ERROR_RTT)
            && top == RttSkipNonLiveEntries(
                old_s,
                Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
                RttWalk(old_s, rd, ipa).level,
                ipa
            )))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && RttLevelIsBlockOrPage(old_s, rd, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, level as int)
        && (ipa as int) < (1 << Realm(old_s, rd).ipa_width)
        && !AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && RttWalk(old_s, rd, ipa).level >= level as int
        && RttWalk(old_s, rd, ipa).rtte.state == ASSIGNED_NS)
        ==> (result.is_Ok()
            && top == RttSkipNonLiveEntries(
                old_s,
                Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
                RttWalk(old_s, rd, ipa).level,
                ipa
            )
            && RttEntry(
                new_s,
                RttWalk(old_s, rd, ipa).rtt_addr,
                RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level)
            ).state == UNASSIGNED_NS))
}