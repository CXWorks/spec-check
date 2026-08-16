pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ToAddress((top) - RMM_GRANULE_SIZE),Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Realm(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((((top) < (RttUpperBound(old_s, base, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level, Realm(old_s, rd).ipa_width as int))) && RttEntryHasRipas(old_s, RttEntry(old_s, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr,RttEntryIndex(old_s, top, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level))) && !AddrIsRttLevelAligned(old_s, top, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s,  Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false), RAM))
  && (result.is_Ok() ==> Realm(new_s, rd).measurements[0] == RimExtendRipas(new_s, Realm(new_s, rd), base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false), RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level))
  && (result.is_Ok() ==> out_top == RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((top) <= (base)) &&
       AddrIsProtected(old_s, ToAddress((top) - RMM_GRANULE_SIZE),Realm(old_s, rd)) &&
       !(Realm(old_s, rd).state != REALM_NEW) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int) &&
       !(RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((((top) < (RttUpperBound(old_s, base, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level, Realm(old_s, rd).ipa_width as int))) && RttEntryHasRipas(old_s, RttEntry(old_s, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).rtt_addr,RttEntryIndex(old_s, top, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level))) && !AddrIsRttLevelAligned(old_s, top, RttWalk_(old_s,rd,base,RMM_RTT_PAGE_LEVEL as int).level as int))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Realm(new_s, rd).measurements[0] == Realm(old_s, rd).measurements[0])
}