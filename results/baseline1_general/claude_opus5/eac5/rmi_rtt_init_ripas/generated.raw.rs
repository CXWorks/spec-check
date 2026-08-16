pub open spec fn rmi_rtt_init_ripas_spec(result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S, rd: Address, base: Address, top: Address) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((top as int) <= (base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ToAddress((top as int) - RMM_GRANULE_SIZE), Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, rd, base).level as int) ==> ResultEqual(result, RMI_ERROR_RTT))
    && (RttWalk(old_s, rd, base).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((((top as int) < (RttUpperBound(old_s, base, RttWalk(old_s, rd, base).level as int, Realm(old_s, rd).ipa_width as int) as int))
            && RttEntryHasRipas(old_s, RttEntry(old_s, RttWalk(old_s, rd, base).rtt_addr, RttEntryIndex(old_s, top, RttWalk(old_s, rd, base).level as int)))
            && !AddrIsRttLevelAligned(old_s, top, RttWalk(old_s, rd, base).level as int)) ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && Granule(old_s, rd).state == RD
            && (top as int) > (base as int)
            && AddrIsGranuleAligned(old_s, top)
            && AddrIsProtected(old_s, ToAddress((top as int) - RMM_GRANULE_SIZE), Realm(old_s, rd))
            && Realm(old_s, rd).state == REALM_NEW
            && AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, rd, base).level as int)
            && RttWalk(old_s, rd, base).rtte.state == UNASSIGNED
            && !(((top as int) < (RttUpperBound(old_s, base, RttWalk(old_s, rd, base).level as int, Realm(old_s, rd).ipa_width as int) as int))
                && RttEntryHasRipas(old_s, RttEntry(old_s, RttWalk(old_s, rd, base).rtt_addr, RttEntryIndex(old_s, top, RttWalk(old_s, rd, base).level as int)))
                && !AddrIsRttLevelAligned(old_s, top, RttWalk(old_s, rd, base).level as int)))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(new_s, Rtt(new_s, RttWalk(old_s, rd, base).rtt_addr), RttWalk(old_s, rd, base).level as int, base,
                    RttSkipEntriesWithRipas(old_s, Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr), RttWalk(old_s, rd, base).level as int, base, top, false), RAM)
            && Realm(new_s, rd).measurements[0] == RimExtendRipas(old_s, Realm(old_s, rd), base,
                    RttSkipEntriesWithRipas(old_s, Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr), RttWalk(old_s, rd, base).level as int, base, top, false),
                    RttWalk(old_s, rd, base).level as int)
            && out_top == RttSkipEntriesWithRipas(old_s, Rtt(old_s, RttWalk(old_s, rd, base).rtt_addr), RttWalk(old_s, rd, base).level as int, base, top, false)))
}