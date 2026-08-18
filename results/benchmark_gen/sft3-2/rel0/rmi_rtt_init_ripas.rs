pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ToAddress((top - RMM_GRANULE_SIZE) as int), RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (RttWalk(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) == (RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false), RAM))
  && (result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimExtendRipas(new_s, RealmAt(new_s, rd), base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false), RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level))
  && (result.is_Ok() ==> out_top == RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((top) <= (base)) &&
       AddrIsProtected(old_s, ToAddress((top - RMM_GRANULE_SIZE) as int), RealmAt(old_s, rd)) &&
       RealmAt(old_s, rd).state == REALM_NEW &&
       AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) &&
       !(RttWalk(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtte.state != UNASSIGNED) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((base) == (RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
  && (result.is_Err()
    ==> RttEntriesInRangeRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top, false), RAM))
}