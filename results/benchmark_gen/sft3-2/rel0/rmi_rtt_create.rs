pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, rd, level) || RttLevelIsStarting(old_s, rd, level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (Granule(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, rd, ipa,level - 1).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa,level - 1).level as int)))
  && (RttWalk(old_s, rd, ipa,level - 1).rtte.state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, rd, ipa,level - 1).level as int)))
  && (result.is_Ok() ==> Granule(new_s, rtt).state == RTT)
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa,level - 1).rtte.state == TABLE)
  && (result.is_Ok() ==> RttWalk(new_s, rd, ipa,level - 1).rtte.addr == rtt)
  && (result.is_Ok() && AddrIsProtected(old_s, ipa, Realm(old_s, rd)) ==> RttAllEntriesRipas(new_s, Rtt(new_s, rtt), RttWalk(new_s, rd, ipa,level - 1).rtte.ripas))
  && (result.is_Ok() ==> RttAllEntriesState(new_s, Rtt(new_s, rtt), RttWalk(new_s, rd, ipa,level - 1).rtte.state))
  && (result.is_Ok() && (RttWalk(old_s, rd, ipa,level - 1).rtte.state != UNASSIGNED && RttWalk(old_s, rd, ipa,level - 1).rtte.state != UNASSIGNED_NS) ==> RttAllEntriesContiguous(new_s, Rtt(new_s, rtt), RttWalk(new_s, rd, ipa,level - 1).rtte.addr, level as int))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(Granule(old_s, rd).state != RD) &&
       !((!RttLevelIsValid(old_s, rd, level) || RttLevelIsStarting(old_s, rd, level))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa) >= pow2(Realm(old_s, rd).ipa_width as nat)) &&
       AddrIsGranuleAligned(old_s, rtt) &&
       PaIsDelegable(old_s, rtt) &&
       !(Granule(old_s, rtt).state != DELEGATED) &&
       !(((Realm(old_s, rd).feat_lpa2 == FEATURE_FALSE) && ((rtt) >= 2^48))) &&
       !(RttWalk(old_s, rd, ipa,level - 1).level < level - 1) &&
       !(RttWalk(old_s, rd, ipa,level - 1).rtte.state == TABLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> Granule(new_s, rtt).state == Granule(old_s, rtt).state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa,level - 1).rtte.state == RttWalk(old_s, rd, ipa,level - 1).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa,level - 1).rtte.addr == RttWalk(old_s, rd, ipa,level - 1).rtte.addr)
  && (result.is_Err()
    ==> RttWalk(new_s, rd, ipa,level - 1).rtte.state == RttWalk(old_s, rd, ipa,level - 1).rtte.state)
  && (RttWalk(new_s, rd, ipa,level - 1).rtte.ripas == RttWalk(old_s, rd, ipa,level - 1).rtte.ripas)
}