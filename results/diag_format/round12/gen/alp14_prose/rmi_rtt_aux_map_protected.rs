pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  ((rd) % GRANULE_SIZE == 0 ==> result.is_Ok())
  && (is_delegable_physical_address(old_s, rd) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok())
  && ((ipa) % GRANULE_SIZE == 0 ==> result.is_Ok())
  && (is_protected_ipa(old_s, RealmAt(old_s, rd), ipa) ==> result.is_Ok())
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).rtt_aux_plane_count) ==> result.is_err())
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas != RAM ==> result.is_err())
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas != DEV ==> result.is_err())
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state == AUX_DESTROYED ==> result.is_err())
  && (result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level ==> result.is_err())
  && ((!( ((rd) % GRANULE_SIZE == 0) &&
        is_delegable_physical_address(old_s, rd) &&
        !(GranuleAt(old_s, rd).state == RD) &&
        !((ipa) % GRANULE_SIZE == 0) &&
        !is_protected_ipa(old_s, RealmAt(old_s, rd), ipa) &&
        !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).rtt_aux_plane_count)) &&
        !(result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas != RAM) &&
        !(result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == ASSIGNED && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas != DEV) &&
        !(result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state == AUX_DESTROYED) &&
        !(result.is_Ok() && RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level < RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).state)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).ripas)
  && (RttWalk(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state == RttWalk(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).state)
}