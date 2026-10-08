pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok())
  && (AddrIsAligned(old_s, rd, RMM_GRANULE_SIZE_ORDER as int) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && (AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int) ==> result.is_Ok())
  && (!((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) ==> result.is_Ok())
  && (AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> result.is_Ok())
  && (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE ==> result.is_Ok())
  && (index == RMM_RTT_TREE_PRIMARY ==> result.is_Ok())
  && (index > RealmAt(old_s, rd).num_aux_planes ==> result.is_Ok())
  && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int)).rtt_addr, RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).level as int)).state == UNASSIGNED_NS)
  && (result.is_Ok() ==> top == RttSkipNonLiveEntries(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).rtt_addr),RttWalk_(new_s, rd, ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).level,ipa))
  && ((!(AddrIsGranuleAligned(old_s, rd)) ||
       !(AddrIsAligned(old_s, rd, RMM_GRANULE_SIZE_ORDER as int)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int)) ||
       ((ipa) < pow2(RealmAt(old_s, rd).ipa_width as nat)) ||
       !(AddrIsProtected(old_s, ipa, RealmAt(old_s, rd))) ||
       !(RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE) ||
       !(index == RMM_RTT_TREE_PRIMARY) ||
       !(index > RealmAt(old_s, rd).num_aux_planes))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttEntryAt(new_s, RttAt(new_s, RttWalk_(new_s, rd, ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int)).rtt_addr, RttEntryIndex(new_s, ipa, RttWalk_(new_s, rd, ipa,RealmAt(new_s, rd).rtt_level_start as int,index as int).level as int)).state == RttWalk_(old_s, rd, ipa,RealmAt(old_s, rd).rtt_level_start as int,index as int).rtte.state)
}