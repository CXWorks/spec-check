pub open spec fn rmi_rtt_aux_prot_map_spec(rd: Address, base: Address, top: Address, flags: RmiRttAuxMapFlags, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.block == RMI_RTT_AUX_BLOCK_NO_CREATE && !AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level))) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.invalid_pri == RMI_RTT_AUX_INVALID_PRI_STOP && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || flags.tree_index == RMM_RTT_TREE_PRIMARY || flags.tree_index > RealmAt(old_s, rd).num_aux_planes) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_NARCH_DEV && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_ARCH_DEV) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT).then(RttWalk(new_s, RealmAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RIPAS_RAM) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT).then(RttWalk(new_s, RealmAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_NARCH_DEV && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RIPAS_DEV) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT).then(RttWalk(new_s, RealmAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_AUX_DESTROYED ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && ResultEqual(result, RMI_ERROR_RTT_AUX).then(RttWalk(new_s, RealmAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level) > (top - base)) ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && ResultEqual(result, RMI_ERROR_RTT_AUX).then(RttWalk(new_s, RealmAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), flags.tree_index as int, base, out_top, RTTE_DATA))
  && (result.is_Ok() ==> RttTreeRangeAllOaddrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Ok() ==> RttTreeRangeAllMemAttrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Ok() ==> RttTreeRangeAllShareabilityEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)) &&
       !((flags.block == RMI_RTT_AUX_BLOCK_NO_CREATE && !AddrIsAligned(old_s, base, RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level))) ) &&
       !((flags.invalid_pri == RMI_RTT_AUX_INVALID_PRI_STOP && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) &&
       !((RealmAt(old_s, rd).rtt_tree_per_plane == FEATURE_FALSE || flags.tree_index == RMM_RTT_TREE_PRIMARY || flags.tree_index > RealmAt(old_s, rd).num_aux_planes)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_NARCH_DEV && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_ARCH_DEV)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RIPAS_RAM)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_NARCH_DEV && RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas != RIPAS_DEV)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_AUX_DESTROYED)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int,flags.tree_index as int).level) > (top - base)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), flags.tree_index as int, base, out_top, RTTE_DATA))
  && (result.is_Err()
    ==> RttTreeRangeAllOaddrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Err()
    ==> RttTreeRangeAllMemAttrEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Err()
    ==> RttTreeRangeAllShareabilityEqual(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY, flags.tree_index as int, base, out_top))
  && (result.is_Err()
    ==> result.status == RMI_SUCCESS)
}