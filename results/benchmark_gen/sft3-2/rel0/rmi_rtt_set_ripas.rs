pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Rec(old_s, rec_ptr).state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
  && (Rec(old_s, rec_ptr).owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base != Rec(old_s, rec_ptr).ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) > (Rec(old_s, rec_ptr).ripas_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) && ripas != Rec(old_s, rec_ptr).ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) == (RttSkipEntriesWithRipas(old_s, Rtt(RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(old_s, rec_ptr).ripas_destroyed !=) && ripas != Rec(old_s, rec_ptr).ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s, Rtt(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, RttSkipEntriesWithRipas(new_s, Rtt(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec_ptr).ripas_destroyed !=), Rec(new_s, rec_ptr).ripas_value))
  && (result.is_Ok() ==> Rec(new_s, rec_ptr).ripas_addr == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, Rtt(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec_ptr).ripas_destroyed !=)))
  && (result.is_Ok() ==> out_top == MinAddress(new_s, top, RttSkipEntriesWithRipas(new_s, Rtt(RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(new_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(new_s, rec_ptr).ripas_destroyed !=)))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, rec_ptr) &&
       PaIsDelegable(old_s, rec_ptr) &&
       !(Granule(old_s, rec_ptr).state != REC) &&
       !(Rec(old_s, rec_ptr).state == REC_RUNNING) &&
       !(Rec(old_s, rec_ptr).owner != rd) &&
       !((top) <= (base)) &&
       !(base != Rec(old_s, rec_ptr).ripas_addr) &&
       !((top) > (Rec(old_s, rec_ptr).ripas_top)) &&
       !((!AddrIsRttLevelAligned(old_s, base, RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level as int) && ripas != Rec(old_s, rec_ptr).ripas_value)) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((!((RttSkipEntriesWithRipas(old_s, Rtt(RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk(old_s,rd, base,RMM_RTT_PAGE_LEVEL as int).level,base, top,Rec(old_s, rec_ptr).ripas_destroyed !=)) && ripas != Rec(old_s, rec_ptr).ripas_value)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Rec(new_s, rec_ptr).ripas_addr == Rec(old_s, rec_ptr).ripas_addr)
  && (result.is_Err()
    ==> Rec(new_s, rec_ptr).ripas_addr == Rec(old_s, rec_ptr).ripas_addr)
}