pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, rd, ipa).level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa).level) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk(new_s, rd, ipa).rtt_addr),RttWalk(new_s, rd, ipa).level,ipa))))
  && (RttWalk(old_s, rd, ipa).rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa).level)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk(new_s, rd, ipa).rtt_addr),RttWalk(new_s, rd, ipa).level,ipa))))
  && (result.is_Ok() ==> Granule(new_s, RttWalk(new_s, rd, ipa).rtte.addr).state == DELEGATED)
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa).rtte.state == UNASSIGNED)
  && (result.is_Ok() && RttWalk(old_s, rd, ipa).rtte.ripas == RAM ==> RttWalk(new_s, rd, ipa).rtte.ripas == DESTROYED)
  && (result.is_Ok() ==> data == RttWalk(new_s, rd, ipa).rtte.addr)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk(new_s, rd, ipa).rtt_addr),RttWalk(new_s, rd, ipa).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, Realm(old_s, rd)) &&
       !(RttWalk(old_s, rd, ipa).level < RMM_RTT_PAGE_LEVEL) &&
       !(RttWalk(old_s, rd, ipa).rtte.state != ASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, RttWalk(new_s, rd, ipa).rtte.addr).state == Granule(old_s, RttWalk(old_s, rd, ipa).rtte.addr).state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa).rtte.state == RttWalk(old_s, rd, ipa).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa).rtte.ripas == RttWalk(old_s, rd, ipa).rtte.ripas)
  && (!(result.is_Ok() && (RttWalk(old_s, rd, ipa).rtte.ripas == RAM)) ==> RttWalk(new_s, rd, ipa).rtte.ripas == RttWalk(old_s, rd, ipa).rtte.ripas)
}