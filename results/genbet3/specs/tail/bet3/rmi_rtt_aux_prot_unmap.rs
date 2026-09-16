pub open spec fn rmi_rtt_aux_prot_unmap_spec(rd: Address, base: Address, top: Address, flags: RmiRttAuxUnmapFlags, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || flags.tree_index == RMM_RTT_TREE_PRIMARY || flags.tree_index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).rtte.state != RTTE_DATA ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && ResultEqual(result, RMI_ERROR_RTT_AUX).then(|| result.get_Err_0().level.level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level))))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level as int) > (top - base)) ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && ResultEqual(result, RMI_ERROR_RTT_AUX).then(|| result.get_Err_0().level.level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level))))
  && (result.is_Ok() ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), flags.tree_index as int,base, out_top, RTTE_VOID))
  && (result.is_Ok() ==> RealmIpaRangeAllRipasIf(new_s, RealmAt(new_s, rd), RealmAt(new_s, rd),base, out_top,RIPAS_RAM, RIPAS_DESTROYED))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !(top <= base) &&
       AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || flags.tree_index == RMM_RTT_TREE_PRIMARY || flags.tree_index > RealmAt(old_s, rd).num_aux_planes)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).rtte.state != RTTE_DATA) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level as int) > (top - base))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), flags.tree_index as int,base, top, RTTE_VOID))
  && (result.is_Err()
    ==> RealmIpaRangeAllRipasIf(new_s, RealmAt(new_s, rd), RealmAt(new_s, rd),base, top,RIPAS_RAM, RIPAS_DESTROYED))
}