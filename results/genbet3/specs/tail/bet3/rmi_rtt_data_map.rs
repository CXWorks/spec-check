pub open spec fn rmi_rtt_data_map_spec(rd: Address, base: Address, top: Address, flags: RmiRttProtMapFlags, oaddr: RmiAddrSetDesc, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.oaddr_type == RMI_ADDR_TYPE_SINGLE && !AddrIsRmiGranuleAligned(old_s, oaddr.data.single.addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.oaddr_type != RMI_ADDR_TYPE_SINGLE && flags.oaddr_type != RMI_ADDR_TYPE_LIST) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr != oaddr_first) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level) > (top - base)) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT)))
  && ((!TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, oaddr_first)) || TrackingRegionGranularity(old_s, TrackingRegionAt(old_s, oaddr_first)) > (top - base)) ==> (ResultEqual(result, RMI_ERROR_TRACKING) && ResultEqual(result, RMI_ERROR_TRACKING)))
  && (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && GranuleAt(old_s, oaddr_first).state != GRAN_DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RealmAt(old_s, rd).feat_ats == FEATURE_TRUE && !DptEntryCanDescribe(old_s, oaddr_first, (top - base))) ==> (ResultEqual(result, RMI_ERROR_DPT) && ResultEqual(result, RMI_ERROR_DPT)))
  && (result.is_Ok() ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, RTTE_DATA))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE ==> RttTreeRangeAllOaddrContig(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base, (out_top - base)))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_LIST ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, oaddr.data.list_addr.addr, (out_top - base)))
  && (result.is_Ok() ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, MEMATTR_CACHEABLE))
  && (result.is_Ok() ==> RttTreeRangeAllShareability(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, SHAREABILITY_INNER))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((result.is_Ok()) ==> RttTreeRangeAllOaddrContig(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base, (out_top - base)))
  && ((result.is_Ok()) ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, oaddr.data.list_addr.addr, (out_top - base)))
  && ((result.is_Ok()) ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, MEMATTR_CACHEABLE))
  && ((result.is_Ok()) ==> RttTreeRangeAllShareability(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, SHAREABILITY_INNER))
  && (((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_SINGLE && !AddrIsRmiGranuleAligned(old_s, oaddr.data.single.addr))) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8))) &&
       !((flags.oaddr_type != RMI_ADDR_TYPE_SINGLE && flags.oaddr_type != RMI_ADDR_TYPE_LIST)) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr))) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr != oaddr_first)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level) > (top - base))) &&
       ((TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, oaddr_first)) && !(TrackingRegionGranularity(old_s, TrackingRegionAt(old_s, oaddr_first)) > (top - base)))) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && GranuleAt(old_s, oaddr_first).state != GRAN_DELEGATED)) &&
       !((RealmAt(old_s, rd).feat_ats == FEATURE_TRUE && !DptEntryCanDescribe(old_s, oaddr_first, (top - base))))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, RTTE_DATA))
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE
    ==> RttTreeRangeAllOaddrContig(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base, (out_top - base)))
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_LIST
    ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, oaddr.data.list_addr.addr, (out_top - base)))
  && (result.is_Err()
    ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, MEMATTR_CACHEABLE))
  && (result.is_Err()
    ==> RttTreeRangeAllShareability(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, SHAREABILITY_INNER))
}