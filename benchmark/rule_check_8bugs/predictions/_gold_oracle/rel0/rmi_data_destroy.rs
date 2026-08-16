pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level,ipa))))
  && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level,ipa))))
  && (result.is_Ok() ==> Granule(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == UNASSIGNED)
  && (result.is_Ok() && RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RAM ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> data == RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtt_addr),RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, Realm(old_s, rd)) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state != ASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr).state == Granule(old_s, RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.addr).state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas)
  && (!(result.is_Ok() && (RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RAM)) ==> RttWalk_(new_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas == RttWalk_(old_s,rd, ipa,RMM_RTT_PAGE_LEVEL as int).rtte.ripas)
}
