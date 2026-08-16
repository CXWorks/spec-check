pub open spec fn rmi_data_destroy_spec(result: Result<(), RmiStatusCode>, rd: Address, ipa: Address, data: Address, top: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0))
    && (!PaIsDelegable(old_s, rd) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0))
    && (Granule(old_s, rd).state != RD ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0))
    && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> (ResultEqual(result, RMI_ERROR_INPUT) && top == 0))
    && (
        AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && RttWalk(old_s, rd, ipa).level < RMM_RTT_PAGE_LEVEL
        ==> (
            ResultEqual(result, RMI_ERROR_RTT)
            && top == RttSkipNonLiveEntries(
                old_s,
                Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
                RttWalk(old_s, rd, ipa).level,
                ipa,
            )
        )
    )
    && (
        AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && RttWalk(old_s, rd, ipa).level == RMM_RTT_PAGE_LEVEL
        && RttWalk(old_s, rd, ipa).rtte.state != ASSIGNED
        ==> (
            ResultEqual(result, RMI_ERROR_RTT)
            && top == RttSkipNonLiveEntries(
                old_s,
                Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
                RttWalk(old_s, rd, ipa).level,
                ipa,
            )
        )
    )
    && (
        AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
        && RttWalk(old_s, rd, ipa).level == RMM_RTT_PAGE_LEVEL
        && RttWalk(old_s, rd, ipa).rtte.state == ASSIGNED
        && RttWalk(old_s, rd, ipa).rtte.ripas == RAM
        ==> (
            result.is_Ok()
            && data == RttWalk(old_s, rd, ipa).rtte.addr
            && top == RttSkipNonLiveEntries(
                old_s,
                Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr),
                RttWalk(old_s, rd, ipa).level,
                ipa,
            )
            && Granule(new_s, RttWalk(old_s, rd, ipa).rtte.addr).state == DELEGATED
            && RttEntry(
                new_s,
                RttWalk(old_s, rd, ipa).rtt_addr,
                RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level),
            ).state == UNASSIGNED
            && RttEntry(
                new_s,
                RttWalk(old_s, rd, ipa).rtt_addr,
                RttEntryIndex(old_s, ipa, RttWalk(old_s, rd, ipa).level),
            ).ripas == DESTROYED
        )
    )
}