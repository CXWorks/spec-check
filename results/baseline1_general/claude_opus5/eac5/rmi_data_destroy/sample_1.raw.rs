pub open spec fn rmi_data_destroy_spec(result: Result<(), RmiStatusCode>, data: Address, top: Address, rd: Address, ipa: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RttWalk(old_s, rd, ipa).level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT)
            && top == RttSkipNonLiveEntries(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr), RttWalk(old_s, rd, ipa).level as int, ipa)))
    && (RttWalk(old_s, rd, ipa).rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT)
            && top == RttSkipNonLiveEntries(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr), RttWalk(old_s, rd, ipa).level as int, ipa)))
    && ((AddrIsGranuleAligned(old_s, rd)
         && PaIsDelegable(old_s, rd)
         && Granule(old_s, rd).state == RD
         && AddrIsGranuleAligned(old_s, ipa)
         && AddrIsProtected(old_s, ipa, Realm(old_s, rd))
         && RttWalk(old_s, rd, ipa).level >= RMM_RTT_PAGE_LEVEL
         && RttWalk(old_s, rd, ipa).rtte.state == ASSIGNED)
        ==> (result.is_Ok()
             && Granule(new_s, RttWalk(old_s, rd, ipa).rtte.addr).state == DELEGATED
             && RttWalk(new_s, rd, ipa).rtte.state == UNASSIGNED
             && (RttWalk(old_s, rd, ipa).rtte.ripas == RAM ==> RttWalk(new_s, rd, ipa).rtte.ripas == DESTROYED)
             && data == RttWalk(old_s, rd, ipa).rtte.addr
             && top == RttSkipNonLiveEntries(old_s, Rtt(old_s, RttWalk(old_s, rd, ipa).rtt_addr), RttWalk(old_s, rd, ipa).level as int, ipa)))
}