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
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((top) < (RttUpperBound(old_s, base, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int, RealmAt(old_s, rd).ipa_width))) && RttEntryHasRipas(old_s, RttEntry(RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr,RttEntryIndex(top, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int))) && !AddrIsRttLevelAligned(old_s, top, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s, Rtt(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr), RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level, base, out_top, Rec(new_s, rec).ripas_value))
  && (result.is_Ok() ==> Rec(new_s, rec).ripas_addr == out_top)
  && (result.is_Ok() ==> out_top == out_top)
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
       AddrIsRttLevelAligned(old_s, base, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(((top) < (RttUpperBound(old_s, base, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int, RealmAt(old_s, rd).ipa_width))) && RttEntryHasRipas(old_s, RttEntry(RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr,RttEntryIndex(top, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int))) && !AddrIsRttLevelAligned(old_s, top, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rec(new_s, rec).ripas_addr == Rec(old_s, rec).ripas_addr)
  && (result.is_Err()
    ==> Rec(new_s, rec).ripas_addr == Rec(old_s, rec).ripas_addr)
}