pub open spec fn rmi_rtt_read_entry_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, walk_level: UInt64, state: RmiRttEntryState, desc: Bits64, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok())
  && (PaIsDelegable(old_s, rd) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && (RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) ==> result.is_Ok())
  && (AddrIsRttLevelAligned(old_s, ipa, level as int) ==> result.is_Ok())
  && (ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> result.is_Ok())
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, level as int,RMM_RTT_TREE_PRIMARY as int).level == walk_level)
  && (result.is_Ok() ==> state == RttEntryStateToRmi(new_s, RttEntryAt(new_s, RttAt(new_s, RealmAt(new_s, rd).rtt_base[0]), 0), RttEntryState(RttEntryAt(new_s, RttAt(new_s, RealmAt(new_s, rd).rtt_base[0]), 0).state)))
  && (result.is_Ok() ==> desc == RttEntryAt(new_s, RttAt(new_s, RealmAt(new_s, rd).rtt_base[0]), 0).addr)
  && (result.is_Ok() ==> ripas == RipasToRmi(new_s, RttEntryAt(new_s, RttAt(new_s, RealmAt(new_s, rd).rtt_base[0]), 0).ripas))
  && ((!(AddrIsGranuleAligned(old_s, rd)) ||
       !PaIsDelegable(old_s, rd) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) ||
       !AddrIsRttLevelAligned(old_s, ipa, level as int) ||
       !(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)))
    ==> result.is_Err())
}