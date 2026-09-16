pub open spec fn rmi_rtt_dev_map_spec(rd: Address, vdev_ptr: Address, base: Address, top: Address, flags: RmiRttProtMapFlags, oaddr: RmiAddrSetDesc, result: Result<(Address,), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, base) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && ((top) <= (base) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) ==> result.status == RMI_ERROR_INPUT)
  && ((flags.oaddr_type == RMI_ADDR_TYPE_SINGLE && !AddrIsRmiGranuleAligned(old_s, oaddr.data.single.addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8)) ==> result.status == RMI_ERROR_INPUT)
  && ((flags.oaddr_type != RMI_ADDR_TYPE_SINGLE && flags.oaddr_type != RMI_ADDR_TYPE_LIST) ==> result.status == RMI_ERROR_INPUT)
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_NARCH_DEV && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr != oaddr.data.single.addr) ==> (result.status == RMI_ERROR_RTT))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_NARCH_DEV && result.data.0 == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level) ==> (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_NARCH_DEV))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level) > (top - base)) ==> (result.status == RMI_ERROR_RTT && result.data.0 == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_RAM ==> (result.status == RMI_ERROR_RTT && result.data.0 == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && ((!TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, oaddr.data.single.addr)) || TrackingRegionGranularity(old_s, TrackingRegionAt(old_s, oaddr.data.single.addr)) > (top - base)) ==> (result.status == RMI_ERROR_TRACKING && result.data.0 == TrackingToRmiResult(old_s, oaddr.data.single.addr).level && result.data.0 == AddrShiftRmiGranule(old_s, oaddr.data.single.addr)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && GranuleAt(old_s, oaddr.data.single.addr).state != GRAN_DELEGATED) ==> result.status == RMI_ERROR_INPUT)
  && ((RealmAt(old_s, rd).feat_ats == FEATURE_TRUE && !DptEntryCanDescribe(old_s, oaddr.data.single.addr, (top - base))) ==> (result.status == RMI_ERROR_DPT && result.data.0 == DptLevel(old_s, oaddr.data.single.addr) && result.data.0 == AddrShiftRmiGranule(old_s, oaddr.data.single.addr)))
  && ((flags.oaddr_type == RMI_ADDR_TYPE_SINGLE && !VdevAddrSingleInRange(old_s, oaddr.data.single,(top - base),VdevAt(old_s, vdev_ptr))) ==> result.status == RMI_ERROR_INPUT)
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !VdevAddrListInRange(old_s, oaddr,(top - base),VdevAt(old_s, vdev_ptr))) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, RTTE_NARCH_DEV))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE ==> RttTreeRangeAllOaddrContig(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base,(top - base)))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_LIST ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, oaddr.data.list_addr.addr,(top - base)))
  && (result.is_Ok() ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, MEMATTR_NON_CACHEABLE))
  && (result.is_Ok() && (AddrSetAllDelegableCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base)) && IsMemattrPassthroughSafe(new_s)) ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, MEMATTR_PASSTHROUGH))
  && (result.is_Ok() && (AddrSetAllDelegableCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base)) && !IsMemattrPassthroughSafe(new_s)) ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, MEMATTR_CACHEABLE))
  && (result.is_Ok() && AddrSetAllDelegableNonCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base)) ==> RttTreeRangeAllShareability(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, SHAREABILITY_OUTER))
  && (result.is_Ok() && AddrSetAllDelegableCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base)) ==> RttTreeRangeAllShareability(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, SHAREABILITY_INNER))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE ==> GranulesAllState(new_s, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base,(top - base), GRAN_DEV))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_LIST ==> GranulesAllStateList(new_s, oaddr.data.list_addr.addr,(top - base), GRAN_DEV))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsTracked(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_SINGLE && !AddrIsRmiGranuleAligned(old_s, oaddr.data.single.addr))) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8))) &&
       !((flags.oaddr_type != RMI_ADDR_TYPE_SINGLE && flags.oaddr_type != RMI_ADDR_TYPE_LIST)) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr))) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_NARCH_DEV && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr != oaddr.data.single.addr)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_NARCH_DEV && result.data.0 == RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level)) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level) > (top - base))) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.ripas == RIPAS_RAM) &&
       !((!TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, oaddr.data.single.addr)) || TrackingRegionGranularity(old_s, TrackingRegionAt(old_s, oaddr.data.single.addr)) > (top - base))) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_VOID && GranuleAt(old_s, oaddr.data.single.addr).state != GRAN_DELEGATED)) &&
       !((RealmAt(old_s, rd).feat_ats == FEATURE_TRUE && !DptEntryCanDescribe(old_s, oaddr.data.single.addr, (top - base)))) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_SINGLE && !VdevAddrSingleInRange(old_s, oaddr.data.single,(top - base),VdevAt(old_s, vdev_ptr)))) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !VdevAddrListInRange(old_s, oaddr,(top - base),VdevAt(old_s, vdev_ptr)))))
    ==> result.status == RMI_SUCCESS)
  && (result.is_Err()
    ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, RTTE_NARCH_DEV))
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE
    ==> RttTreeRangeAllOaddrContig(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base,(top - base)))
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_LIST
    ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, oaddr.data.list_addr.addr,(top - base)))
  && (result.is_Err()
    ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, MEMATTR_NON_CACHEABLE))
  && (result.is_Err() && (AddrSetAllDelegableCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base)) && IsMemattrPassthroughSafe(new_s))
    ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, MEMATTR_PASSTHROUGH))
  && (result.is_Err() && (AddrSetAllDelegableCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base)) && !IsMemattrPassthroughSafe(new_s))
    ==> RttTreeRangeAllMemAttr(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, MEMATTR_CACHEABLE))
  && (result.is_Err() && AddrSetAllDelegableNonCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base))
    ==> RttTreeRangeAllShareability(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, SHAREABILITY_OUTER))
  && (result.is_Err() && AddrSetAllDelegableCohDevMem(new_s, oaddr, flags.oaddr_type, (top - base))
    ==> RttTreeRangeAllShareability(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, SHAREABILITY_INNER))
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE
    ==> GranulesAllState(new_s, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base,(top - base), GRAN_DEV))
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_LIST
    ==> GranulesAllStateList(new_s, oaddr.data.list_addr.addr,(top - base), GRAN_DEV))
}