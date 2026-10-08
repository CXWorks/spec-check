pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((!(AddrIsGranuleAligned(old_s, rd)) ||
       !(PaIsDelegable(old_s, rd)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)) ||
       !(RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)) ||
       !(AddrIsRttLevelAligned(old_s, ipa, level - 1 as int)) ||
       !(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) ||
       !(AddrIsGranuleAligned(old_s, rtt)) ||
       !(PaIsDelegable(old_s, rtt)) ||
       !(GranuleAt(old_s, rtt).state == DELEGATED) ||
       !(!(ImplFeatures(old_s).feat_lpa2 == FEATURE_TRUE) && rtt >= (1 << 48)))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (result.is_Ok()
      ==> GranuleAt(new_s, rtt).state == RTT)
    && (result.is_Ok()
      ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == TABLE)
    && (result.is_Ok()
      ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == rtt)
    && ((!(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int)) &&
         !(RttLevelIsStarting(old_s, RealmAt(old_s, rd), level as int)))
      ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
    && (result.is_Ok()
      ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
    && (result.is_Err()
      ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
    && (result.is_Err()
      ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.state)
    && (result.is_Err()
      ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
}