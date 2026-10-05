pub open spec fn rmi_rtt_aux_map_protected_spec(rd: PhysicalAddress, ipa: Ipa, index: UInt64, result: Result<(), RmiStatusCode>, state: UInt8, ripas: UInt8, old_s: S, new_s: S) -> bool {
  ((rd) % GRANULE_SIZE == 0 ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (is_delegable_address(old_s, rd) ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && ((ipa) % GRANULE_SIZE == 0 ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (is_protected_ipa(old_s, ipa) ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (index == RMM_RTT_TREE_PRIMARY ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (index > RealmAt(old_s, rd).rtt_tree_per_plane ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (result == RMI_ERROR_RTT(RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).level as int) ==> RTTEntryAt(old_s, RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).rtt, RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).index as int).state == state)
  && (result == RMI_ERROR_RTT(RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).level as int) ==> RTTEntryAt(old_s, RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).rtt, RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).index as int).ripas == ripas)
  && (result == RMI_ERROR_RTT(RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).level as int) ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, RMM_RTT_TREE_PRIMARY as int).rtt, RttWalk(new_s, ipa, RMM_RTT_TREE_PRIMARY as int).index as int).state == RTTEntryAt(old_s, RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).rtt, RttWalk(old_s, ipa, RMM_RTT_TREE_PRIMARY as int).index as int).state)
  && (result == RMI_ERROR_RTT_AUX(RttWalk(old_s, ipa, index as int).level as int) ==> RTTEntryAt(old_s, RttWalk(old_s, ipa, index as int).rtt, RttWalk(old_s, ipa, index as int).index as int).state == state)
  && (result == RMI_ERROR_RTT_AUX(RttWalk(old_s, ipa, index as int).level as int) ==> RTTEntryAt(old_s, RttWalk(old_s, ipa, index as int).rtt, RttWalk(old_s, ipa, index as int).index as int).ripas == ripas)
  && (result == RMI_ERROR_RTT_AUX(RttWalk(old_s, ipa, index as int).level as int) ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, index as int).rtt, RttWalk(new_s, ipa, index as int).index as int).state == RTTEntryAt(old_s, RttWalk(old_s, ipa, index as int).rtt, RttWalk(old_s, ipa, index as int).index as int).state)
  && (result.is_Ok() ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, index as int).rtt, RttWalk(new_s, ipa, index as int).index as int).state == ASSIGNED)
  && (result.is_Ok() ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, index as int).rtt, RttWalk(new_s, ipa, index as int).index as int).protection_attributes == RTTEntryAt(new_s, RttWalk(new_s, ipa, RMM_RTT_TREE_PRIMARY as int).rtt, RttWalk(new_s, ipa, RMM_RTT_TREE_PRIMARY as int).index as int).protection_attributes)
  && (result.is_Ok() ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, index as int).rtt, RttWalk(new_s, ipa, index as int).index as int).shareability_attribute == RTTEntryAt(new_s, RttWalk(new_s, ipa, RMM_RTT_TREE_PRIMARY as int).rtt, RttWalk(new_s, ipa, RMM_RTT_TREE_PRIMARY as int).index as int).shareability_attribute)
  && ((!( (rd) % GRANULE_SIZE == 0) &&
       !(is_delegable_address(old_s, rd)) &&
       !(GranuleAt(old_s, rd).state == RD) &&
       !((ipa) % GRANULE_SIZE == 0) &&
       !(is_protected_ipa(old_s, ipa)) &&
       !(RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE) &&
       !(index == RMM_RTT_TREE_PRIMARY) &&
       !(index > RealmAt(old_s, rd).rtt_tree_per_plane))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, index as int).rtt, RttWalk(new_s, ipa, index as int).index as int).state == RTTEntryAt(old_s, RttWalk(old_s, ipa, index as int).rtt, RttWalk(old_s, ipa, index as int).index as int).state)
  && (result.is_Err()
    ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, index as int).rtt, RttWalk(new_s, ipa, index as int).index as int).protection_attributes == RTTEntryAt(old_s, RttWalk(old_s, ipa, index as int).rtt, RttWalk(old_s, ipa, index as int).index as int).protection_attributes)
  && (result.is_Err()
    ==> RTTEntryAt(new_s, RttWalk(new_s, ipa, index as int).rtt, RttWalk(new_s, ipa, index as int).index as int).shareability_attribute == RTTEntryAt(old_s, RttWalk(old_s, ipa, index as int).rtt, RttWalk(old_s, ipa, index as int).index as int).shareability_attribute)
}