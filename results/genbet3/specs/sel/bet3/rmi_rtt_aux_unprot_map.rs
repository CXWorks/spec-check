pub open spec fn rmi_rtt_aux_unprot_map_spec(rd: Address, base: Address, top: Address, flags: RmiRttAuxMapFlags, result: Result<(Address,), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && ((top) <= (base) ==> result.status == RMI_ERROR_INPUT)
  && (AddrIsProtected(old_s, base, RealmAt(old_s, rd)) ==> result.status == RMI_ERROR_INPUT)
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || flags.tree_index == RMM_RTT_TREE_PRIMARY || flags.tree_index > RealmAt(old_s, rd).num_aux_planes) ==> result.status == RMI_ERROR_INPUT)
  && (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_MAPPED_NS ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level as int) > (top - base)) ==> (result.status == RMI_ERROR_RTT_AUX && result.data.level.level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level))
  && (result.is_Ok() ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), flags.tree_index as int, base, out_top, RTTE_MAPPED_NS))
  && (result.is_Ok() ==> RttTreeRangeAllOaddrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Ok() ==> RttTreeRangeAllMemAttrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Ok() ==> RttTreeRangeAllShareabilityEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       !(AddrIsProtected(old_s, base, RealmAt(old_s, rd))) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || flags.tree_index == RMM_RTT_TREE_PRIMARY || flags.tree_index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_MAPPED_NS) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level as int) > (top - base))))
    ==> result.status == RMI_SUCCESS)
  && (result.is_Err()
    ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), flags.tree_index as int, base, out_top, RTTE_MAPPED_NS))
  && (result.is_Err()
    ==> RttTreeRangeAllOaddrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Err()
    ==> RttTreeRangeAllMemAttrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Err()
    ==> RttTreeRangeAllShareabilityEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
}