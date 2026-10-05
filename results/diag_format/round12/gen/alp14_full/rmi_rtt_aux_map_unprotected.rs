pub open spec fn rmi_rtt_aux_map_unprotected_spec(rd: Rd, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  ((!(RealmAt(old_s, rd).rd as Address) % GRANULE_SIZE == 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_delegatable_physical_address(old_s, RealmAt(old_s, rd).rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd, (RealmAt(old_s, rd).rd as Address) % GRANULE_SIZE).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!((ipa) % (pow2(RealmAt(old_s, rd).rtt_level_start as nat))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (is_protected_address(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(RealmAt(old_s, rd).per_plane_rtt_tree) || (index) == RMM_RTT_TREE_PRIMARY || (index) > RealmAt(old_s, rd).num_auxiliary_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int).state == UNASSIGNED_NS ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int)))
  && (result.is_Ok() ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).state == RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).state)
  && (result.is_Ok() ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).mem_attr == RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).mem_attr)
  && (result.is_Ok() ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).stage2_access_perms == RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).stage2_access_perms)
  && (result.is_Ok() ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).output_addr == RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).output_addr)
  && ((!(RealmAt(old_s, rd).rd as Address) % GRANULE_SIZE == 0 &&
       is_delegatable_physical_address(old_s, RealmAt(old_s, rd).rd) &&
       !(GranuleAt(old_s, rd, (RealmAt(old_s, rd).rd as Address) % GRANULE_SIZE).state != RD) &&
       !((ipa) % (pow2(RealmAt(old_s, rd).rtt_level_start as nat))) &&
       !((ipa) >= pow2(RealmAt(old_s, rd).ipa_width as nat)) &&
       !(is_protected_address(old_s, ipa, RealmAt(old_s, rd))) &&
       !((!(RealmAt(old_s, rd).per_plane_rtt_tree) || (index) == RMM_RTT_TREE_PRIMARY || (index) > RealmAt(old_s, rd).num_auxiliary_planes)) &&
       !(RttWalk(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int).state == UNASSIGNED_NS))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).state == RttAt(old_s, ipa, RttWalk(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int).level as int).state)
  && (result.is_Err()
    ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).mem_attr == RttAt(old_s, ipa, RttWalk(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int).level as int).mem_attr)
  && (result.is_Err()
    ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).stage2_access_perms == RttAt(old_s, ipa, RttWalk(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int).level as int).stage2_access_perms)
  && (result.is_Err()
    ==> RttAt(new_s, ipa, RttWalk(new_s, ipa, RealmAt(new_s, rd).rtt_level_start as int).level as int).output_addr == RttAt(old_s, ipa, RttWalk(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int).level as int).output_addr)
}