pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((!(AddrIsAligned(old_s, rd, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(PaIsDelegable(old_s, rd)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int)) ||
       !(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) ||
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ||
       (ImplFeatures(old_s).rtt_tree_per_plane == FEATURE_FALSE) ||
       (index == RMM_RTT_TREE_PRIMARY) ||
       (index > RealmAt(old_s, rd).num_aux_planes))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Err()
    ==> GranuleAt(new_s, rd).state == GranuleAt(old_s, rd).state)
  && (result.is_Ok()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.ripas)
  && (result.is_Ok()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot == RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.attr_unprot)
  && (result.is_Ok()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.overlay_index == RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.s2ap_indirect.overlay_index)
  && ((!(AddrIsAligned(old_s, rd, RMM_GRANULE_SIZE_ORDER as int)) &&
       PaIsDelegable(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) &&
       !(ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) &&
       !(ImplFeatures(old_s).rtt_tree_per_plane == FEATURE_FALSE) &&
       !(index == RMM_RTT_TREE_PRIMARY) &&
       !(index > RealmAt(old_s, rd).num_aux_planes))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_unprot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.overlay_index == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,RMM_RTT_TREE_PRIMARY as int).rtte.s2ap_indirect.overlay_index)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtte.ripas)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.attr_unprot == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtte.attr_unprot)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.s2ap_indirect.overlay_index == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtte.s2ap_indirect.overlay_index)
}