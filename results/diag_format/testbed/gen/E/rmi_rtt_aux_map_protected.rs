pub open spec fn rmi_rtt_aux_map_protected_spec(rd: Address, ipa: Address, index: UInt64, result: Result<(), RmiStatusCode>, state: RmiRttEntryState, ripas: RmiRipas, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU ==> ResultEqual(result, RMI_ERROR_RTT(RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RAM ==> ResultEqual(result, RMI_ERROR_RTT(RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != DEV ==> ResultEqual(result, RMI_ERROR_RTT(RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == AUX_DESTROYED ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int))))
  && (RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level < RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level ==> ResultEqual(result, RMI_ERROR_RTT_AUX(RttEntryIndex(old_s, ipa, RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level as int))))
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == ASSIGNED)
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.attr_prot == RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.attr_prot)
  && (result.is_Ok() ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.sh == RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.sh)
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, ipa) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > RealmAt(old_s, rd).num_aux_planes) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_DEV && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != ASSIGNED_VSMMU) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RAM) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == ASSIGNED_DEV && RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != DEV) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == AUX_DESTROYED) &&
       !(RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).level < RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.state)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.attr_prot == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.attr_prot)
  && (result.is_Err()
    ==> RttWalk_(new_s, RealmAt(new_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.sh == RttWalk_(old_s, RealmAt(old_s, rd), ipa,RMM_RTT_PAGE_LEVEL as int,index as int).rtte.sh)
}