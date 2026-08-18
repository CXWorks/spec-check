pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, rd, level) || RttLevelIsStarting(old_s, rd, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, rd, ipa,level - 1 as int).level < level - 1 ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa,level - 1 as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk(new_s, rd, ipa,level - 1 as int).rtt_addr),RttWalk(new_s, rd, ipa,level - 1 as int).level,ipa))))
  && (RttWalk(old_s, rd, ipa,level - 1 as int).rtte.state != TABLE ==> (ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa,level - 1 as int).level as int)) && (top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk(new_s, rd, ipa,level - 1 as int).rtt_addr),RttWalk(new_s, rd, ipa,level - 1 as int).level,ipa))))
  && (RttIsLive(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr)) ==> (ResultEqual(result, RMI_ERROR_RTT(level)) && (top == ipa)))
  && (result == RMI_SUCCESS ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.state == UNASSIGNED)
  && (result == RMI_SUCCESS ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.ripas == DESTROYED)
  && (result == RMI_SUCCESS ==> Granule(new_s, RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr).state == DELEGATED)
  && (result == RMI_SUCCESS ==> rtt == RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr)
  && (result == RMI_SUCCESS ==> top == RttSkipNonLiveEntries(new_s, Rtt(new_s, RttWalk(new_s, rd, ipa,level - 1 as int).rtt_addr),RttWalk(new_s, rd, ipa,level - 1 as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, rd, level) || RttLevelIsStarting(old_s, rd, level))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)) &&
       !(RttWalk(old_s, rd, ipa,level - 1 as int).level < level - 1) &&
       !(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.state != TABLE) &&
       !(RttIsLive(old_s, Rtt(RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr))))
    ==> result == RMI_SUCCESS)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.state == RttWalk(old_s, rd, ipa,level - 1 as int).rtte.state)
  && (result != RMI_SUCCESS
    ==> RttWalk(new_s, rd, ipa,level - 1 as int).rtte.ripas == RttWalk(old_s, rd, ipa,level - 1 as int).rtte.ripas)
  && (result != RMI_SUCCESS
    ==> Granule(new_s, RttWalk(new_s, rd, ipa,level - 1 as int).rtte.addr).state == Granule(old_s, RttWalk(old_s, rd, ipa,level - 1 as int).rtte.addr).state)
  && (RttWalk(new_s, rd, ipa,level - 1 as int).rtte.ripas == DESTROYED
    ==> result == RMI_SUCCESS)
}