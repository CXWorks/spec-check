pub open spec fn rmi_rtt_create_spec(rd: Rd, rtt: Rtt, ipa: Address, level: int, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((!(AddrIsDelegable(old_s, rd)) ||
    !(GranuleAt(old_s, rd).state == RD) ||
    !(level < RttLevelMax(old_s, rd) || !(RttLevelStart(old_s, rd) == level)) ||
    !(AddrIsRttLevelAligned(old_s, ipa, RttLevelStart(old_s, rd) as int)) ||
    !(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) ||
    !(AddrIsDelegable(old_s, rtt)) ||
    !(GranuleAt(old_s, rtt).state == DELEGATED) ||
    !(ImplFeatures(old_s).feat_lpa2 == FEATURE_FALSE && rtt >= 2^48))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result == RMI_ERROR_RTT(0)
    ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).level as int)))
  && (result == RMI_ERROR_RTT(0)
    ==> RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).state == TABLE)
  && (result.is_Ok()
    ==> GranuleAt(new_s, rtt).state == RTT)
  && (result.is_Ok()
    ==> RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).state == TABLE)
  && (result.is_Ok()
    ==> RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).rtt == rtt)
  && ((!(AddrIsDelegable(old_s, rd)) &&
       !(GranuleAt(old_s, rd).state == RD) &&
       !(level < RttLevelMax(old_s, rd) || !(RttLevelStart(old_s, rd) == level)) &&
       !(AddrIsRttLevelAligned(old_s, ipa, RttLevelStart(old_s, rd) as int)) &&
       !(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(AddrIsDelegable(old_s, rtt)) &&
       !(GranuleAt(old_s, rtt).state == DELEGATED) &&
       !(ImplFeatures(old_s).feat_lpa2 == FEATURE_FALSE && rtt >= 2^48))
     ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
  && (result.is_Err()
    ==> RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).state == RttEntry(old_s, RttWalk(old_s, ipa, RttLevelStart(old_s, rd) as int).index).state)
  && (result.is_Err()
    ==> RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).rtt == RttEntry(old_s, RttWalk(old_s, ipa, RttLevelStart(old_s, rd) as int).index).rtt)
  && (RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).state == UNASSIGNED ||
       RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).state == UNASSIGNED_NS
    ==> result.is_Err())
  && (!(result.is_Ok() && (RttEntry(old_s, RttWalk(old_s, ipa, RttLevelStart(old_s, rd) as int).index).state != UNASSIGNED && RttEntry(old_s, RttWalk(old_s, ipa, RttLevelStart(old_s, rd) as int).index).state != UNASSIGNED_NS)) ==> result.is_Err())
  && (result.is_Ok()
    ==> RttEntry(new_s, RttWalk(new_s, ipa, RttLevelStart(new_s, rd) as int).index).state == TABLE)
}