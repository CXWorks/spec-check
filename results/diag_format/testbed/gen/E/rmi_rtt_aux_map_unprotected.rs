pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(ipa < pow2(RealmAt(old_s, rd).ipa_width as nat)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (index == RMM_RTT_TREE_PRIMARY ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (index > RealmAt(old_s, rd).num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int).rtte.state == UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RMM_RTT_TREE_PRIMARY as int)))
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.state == RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.state)
  && (result.is_Ok() ==> RttEntryStateToRmi(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.attr_unprot) == RttEntryStateToRmi(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.attr_unprot))
  && (result.is_Ok() ==> RttS2APEqual(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.s2ap_indirect.s2ap_indirect.encoding) == RttS2APEqual(new_s, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte, RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.s2ap_indirect.s2ap_indirect.encoding))
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.s2ap_indirect.s2ap_indirect.base_index == RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.s2ap_indirect.s2ap_indirect.base_index)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) &&
       (ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) &&
       !(RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE) &&
       !(index == RMM_RTT_TREE_PRIMARY) &&
       !(index > RealmAt(old_s, rd).num_aux_planes) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int).rtte.state == UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int).rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int).rtte.state)
}