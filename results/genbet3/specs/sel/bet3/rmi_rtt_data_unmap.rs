pub open spec fn rmi_rtt_data_unmap_spec(rd: Address, base: Address, top: Address, flags: RmiRttUnmapFlags, oaddr: RmiAddrSetDesc, result: Result<RmiCommandReturnCode, RmiStatusCode>, out_top: Address, out_range: RmiAddrRangeDesc, out_count: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, base) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && ((top) <= (base) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) ==> result.status == RMI_ERROR_INPUT)
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8)) ==> result.status == RMI_ERROR_INPUT)
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_DATA && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) > (top - base)) ==> (result.status == RMI_ERROR_RTT && result.data.level.level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && ((!TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, oaddr.data.list_addr.addr)) || TrackingRegionGranularity(old_s, TrackingRegionAt(old_s, oaddr.data.list_addr.addr)) > (top - base)) ==> (result.status == RMI_ERROR_TRACKING && result.data.level_addr.level == TrackingToRmiResult(new_s, oaddr.data.list_addr.addr) && result.data.level_addr.addr == AddrShiftRmiGranule(new_s, oaddr.data.list_addr.addr)))
  && ((RealmAt(old_s, rd).feat_ats == FEATURE_TRUE && !DptEntryCanDescribe(old_s, oaddr.data.list_addr.addr, (top - base))) ==> (result.status == RMI_ERROR_DPT && result.data.level_addr.level == DptLevel(old_s, oaddr.data.list_addr.addr) && result.data.level_addr.addr == AddrShiftRmiGranule(new_s, oaddr.data.list_addr.addr)))
  && (result.is_Ok() ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, out_top, RTTE_VOID))
  && (result.is_Ok() ==> RealmIpaRangeAllRipasIf(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd),base, out_top,RIPAS_RAM, RIPAS_DESTROYED))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE ==> out_range.data.addr == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_LIST ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base,oaddr.data.list_addr.addr,(top - base)))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE ==> GranulesAllState(new_s, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base,(top - base), GRAN_DELEGATED))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_LIST ==> GranulesAllStateList(new_s, oaddr.data.list_addr.addr,(top - base), GRAN_DELEGATED))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       AddrRangeIsProtected(old_s, base, top, RealmAt(old_s, rd)) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8))) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr))) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_DATA && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) > (top - base))) &&
       !((!TrackingRegionIsTracked(old_s, TrackingRegionAt(old_s, oaddr.data.list_addr.addr)) || TrackingRegionGranularity(old_s, TrackingRegionAt(old_s, oaddr.data.list_addr.addr)) > (top - base))) &&
       !((RealmAt(old_s, rd).feat_ats == FEATURE_TRUE && !DptEntryCanDescribe(old_s, oaddr.data.list_addr.addr, (top - base)))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base, top, RTTE_VOID))
  && (result.is_Err()
    ==> RealmIpaRangeAllRipasIf(new_s, RealmAt(new_s, rd),RealmAt(new_s, rd),base, top,RIPAS_RAM, RIPAS_DESTROYED))
  && (result.is_Err()
    ==> out_range.data.addr == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Err()
    ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY,base,oaddr.data.list_addr.addr,(top - base)))
  && (result.is_Err()
    ==> GranulesAllState(new_s, RmiAddrRangeDescDecode(new_s, oaddr.data.single).base,(top - base), GRAN_DELEGATED))
  && (result.is_Err()
    ==> GranulesAllStateList(new_s, oaddr.data.list_addr.addr,(top - base), GRAN_DELEGATED))
}