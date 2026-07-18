pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<RmiCommandReturnCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa, RealmAt(new_s, rd).rtt_level_start,RMM_RTT_TREE_PRIMARY).level as int)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, realm.rtt_level_start, index).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa, realm.rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte.state)
  && (result.is_Ok() ==> (RttMemAttrEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa, realm.rtt_level_start, index).rtte, RttWalk(old_s, RealmAt(old_s, rd), ipa, realm.rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte,RTT_UNPROTECTED) && RttS2APEqual(RttWalk(new_s, RealmAt(new_s, rd), ipa, realm.rtt_level_start, index).rtte, RttWalk(old_s, RealmAt(old_s, rd), ipa, realm.rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte,RealmAt(old_s, rd).rtt_s2ap_encoding)))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, realm.rtt_level_start, index).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa, realm.rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte.addr)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start) &&
       !(((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) || AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)))) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa, RealmAt(old_s, rd).rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte.state == UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, realm.rtt_level_start, index).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa, realm.rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa, realm.rtt_level_start, index).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa, realm.rtt_level_start,RMM_RTT_TREE_PRIMARY).rtte.addr)
}