pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, rec).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (Rec(old_s, rec).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != Rec(old_s, rec).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (Rec(old_s, rec).ripas_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((((top) < (RttUpperBound(old_s, base, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level, Realm(old_s, rd).ipa_width as int))) && RttEntryHasRipas(old_s, RttEntry(old_s, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr,RttEntryIndex(old_s, top, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level))) && !AddrIsRttLevelAligned(old_s, top, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s,  Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec).ripas_destroyed!=CHANGE_DESTROYED), Rec(new_s, rec).ripas_value))
  && (result.is_Ok() ==> Rec(new_s, rec).ripas_addr == RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec).ripas_destroyed!=CHANGE_DESTROYED))
  && (result.is_Ok() ==> out_top == RttSkipEntriesWithRipas(new_s, Rtt(new_s, RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec).ripas_destroyed!=CHANGE_DESTROYED))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec) &&
       PaIsDelegable(old_s, rec) &&
       !(Granule(old_s, rec).state != REC) &&
       !(Rec(old_s, rec).state == REC_RUNNING) &&
       !(Rec(old_s, rec).owner != rd) &&
       !((top) <= (base)) &&
       !(base != Rec(old_s, rec).ripas_addr) &&
       !((top) > (Rec(old_s, rec).ripas_top)) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((((top) < (RttUpperBound(old_s, base, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level, Realm(old_s, rd).ipa_width as int))) && RttEntryHasRipas(old_s, RttEntry(old_s, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr,RttEntryIndex(old_s, top, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level))) && !AddrIsRttLevelAligned(old_s, top, RttWalk_(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rec(new_s, rec).ripas_addr == Rec(old_s, rec).ripas_addr)
}
