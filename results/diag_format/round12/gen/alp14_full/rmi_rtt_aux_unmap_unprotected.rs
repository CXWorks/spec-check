pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(rd: Rd, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, top: Address, old_s: S, new_s: S) -> bool {
  ((!(AddrIsGranuleAligned(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(IsDelegatablePhysicalAddress(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(GranuleAt(old_s, rd).state == RD) ==> ResultEqual(result, RMI_ERROR_INPUT))
   ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(AddrIsRttLevelAligned(old_s, ipa, RealmAt(old_s, rd).rtt_level_start as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
   ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (((ipa >= pow2(RealmAt(old_s, rd).ipa_width as nat)) ||
       (IsProtectedIpa(old_s, rd, ipa))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
   ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE) &&
       !(index == RMM_RTT_TREE_PRIMARY) &&
       !(index > RealmAt(old_s, rd).num_aux_planes)
    ==> result.is_Ok())
   ==> result.is_Ok())
  && (result.is_Ok()
    ==> RttAt(new_s, rd, ipa, RealmAt(new_s, rd).rtt_level_start as int, index).state == UNASSIGNED_NS)
  && (result.is_Ok()
    ==> top == RttWalk(new_s, rd, ipa, RealmAt(new_s, rd).rtt_level_start as int, index).top)
  && ((result.is_Err()
       ==> RttAt(new_s, rd, ipa, RealmAt(new_s, rd).rtt_level_start as int, index).state == RttAt(old_s, rd, ipa, RealmAt(old_s, rd).rtt_level_start as int, index).state)
    ==> true)
}