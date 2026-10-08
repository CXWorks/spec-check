pub open spec fn rmi_rtt_create_spec(rd: Address, rtt: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !(level == RealmAt(old_s, rd).rtt_level_start)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(ipa < pow2(RealmAt(old_s, rd).ipa_width as nat)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDram(old_s, rtt) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rtt).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE && rtt >= pow2(48) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1 ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).state == TABLE ==> ResultEqual(result, RMI_ERROR_RTT(0)))
  && (result.is_Ok() ==> GranuleAt(new_s, rtt).state == RTT)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).state == TABLE)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).rtt == rtt)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas)
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).state)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) && !(level == RealmAt(old_s, rd).rtt_level_start))) &&
       AddrIsRttLevelAligned(old_s, ipa, level - 1 as int) &&
       !((ipa < pow2(RealmAt(old_s, rd).ipa_width as nat))) &&
       AddrIsGranuleAligned(old_s, rtt) &&
       PaIsDelegableDram(old_s, rtt) &&
       !(GranuleAt(old_s, rtt).state != DELEGATED) &&
       !((RealmAt(old_s, rd).feat_lpa2 == FEATURE_FALSE && rtt >= pow2(48))) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level < level - 1) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).state == TABLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, rtt).state == GranuleAt(old_s, rtt).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).state == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).state)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).rtt == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).rtt)
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas == RttEntryAt(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,level - 1 as int,RMM_RTT_TREE_PRIMARY as int).level)).ripas)
}