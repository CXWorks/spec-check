pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<RmiCommandReturnCode, RmiStatusCode>, new_s: S) -> bool {
  (!AddrIsGranuleAligned(rd, new_s) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(rd, new_s) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(rd, new_s).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(ipa, RealmAt(rd, new_s).rtt_level_start as int, new_s) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa) >= pow2(RealmAt(rd, new_s).ipa_width as nat) || AddrIsProtected(ipa, RealmAt(rd, new_s), new_s)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(rd, new_s).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(rd, new_s).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr,RttEntryIndex(ipa, RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int), new_s).state == UNASSIGNED_NS)
  && (result.is_Ok() ==> RttSkipNonLiveEntries(RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr),RttLevel(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int),ipa,new_s) == RttSkipNonLiveEntries(RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr),RttLevel(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int),ipa,new_s))
  && ((AddrIsGranuleAligned(rd, new_s) &&
       PaIsDelegable(rd, new_s) &&
       !(GranuleAt(rd, new_s).state != RD) &&
       AddrIsRttLevelAligned(ipa, RealmAt(rd, new_s).rtt_level_start as int, new_s) &&
       !(((ipa) >= pow2(RealmAt(rd, new_s).ipa_width as nat) || AddrIsProtected(ipa, RealmAt(rd, new_s), new_s))) &&
       !((RealmAt(rd, new_s).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(rd, new_s).num_aux_planes)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr,RttEntryIndex(ipa, RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int), new_s).state == RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr,RttEntryIndex(ipa, RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int), new_s).state)
    && RttSkipNonLiveEntries(RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr),RttLevel(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int),ipa,new_s) == RttSkipNonLiveEntries(RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr),RttLevel(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int),ipa,new_s))
    && RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr,RttEntryIndex(ipa, RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int), new_s).state == RttAt(RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).rtt_addr,RttEntryIndex(ipa, RttWalk(RealmAt(rd, new_s), ipa, RealmAt(rd, new_s).rtt_level_start,RttTree(index as int), new_s).level as int), new_s).state))
}