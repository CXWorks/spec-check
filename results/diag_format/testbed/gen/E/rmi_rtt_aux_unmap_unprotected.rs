pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, ipa, pow2(RealmAt(old_s, rd).ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(ipa < pow2(RealmAt(old_s, rd).ipa_width as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (index == RMM_RTT_TREE_PRIMARY ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (index > RealmAt(old_s, rd).num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RttEntry(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtt_addr, RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).level as int)).state == UNASSIGNED_NS)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtt_addr), RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).level, ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsAligned(old_s, ipa, pow2(RealmAt(old_s, rd).ipa_width as int)) &&
       (ipa < pow2(RealmAt(old_s, rd).ipa_width as int)) &&
       !(AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) &&
       !(RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE) &&
       !(index == RMM_RTT_TREE_PRIMARY) &&
       !(index > RealmAt(old_s, rd).num_aux_planes))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttEntry(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtt_addr, RttEntryIndex(new_s, ipa, RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).level as int)).state == RttEntry(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtt_addr, RttEntryIndex(old_s, ipa, RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).level as int)).state)
  && (result.is_Err()
    ==> top == RttSkipNonLiveEntries(old_s, RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtt_addr), RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).level, ipa))
}