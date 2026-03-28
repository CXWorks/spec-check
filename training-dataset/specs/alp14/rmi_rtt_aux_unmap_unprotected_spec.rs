pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.state == UNASSIGNED_NS)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).level,ipa))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) &&
       !(((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtte.state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtte.ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtte.ripas)
}
