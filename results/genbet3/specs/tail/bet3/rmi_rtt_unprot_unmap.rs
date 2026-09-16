pub open spec fn rmi_rtt_unprot_unmap_spec(rd: Address, base: Address, top: Address, flags: RmiRttUnmapFlags, oaddr: RmiAddrSetDescT, result: Result<(), RmiStatusCode>, out_top: Address, out_range: RmiAddrRangeDesc, out_count: UInt64, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (AddrIsProtected(old_s, base, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_MAPPED_NS && !AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT).then(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_MAPPED_NS && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) > (top) - (base)) ==> (ResultEqual(result, RMI_ERROR_RTT) && ResultEqual(result, RMI_ERROR_RTT).then(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, RTTE_UNMAPPED_NS))
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE ==> out_range.data.addr == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Ok() && flags.oaddr_type == RMI_ADDR_TYPE_LIST ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, oaddr.data.list_addr.addr, (out_top) - (base)))
  && (result.is_Ok() ==> result.status == RMI_SUCCESS)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, base) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((top) <= (base)) &&
       !(AddrIsProtected(old_s, base, RealmAt(old_s, rd))) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !AddrIsAligned(old_s, oaddr.data.list_addr.addr, 8))) &&
       !((flags.oaddr_type == RMI_ADDR_TYPE_LIST && !NonSecureAccessPermitted(old_s, oaddr.data.list_addr.addr))) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_MAPPED_NS && !AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state == RTTE_MAPPED_NS && RttLevelSize(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) > (top) - (base))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RttTreeRangeAllState(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, out_top, RTTE_UNMAPPED_NS))
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_SINGLE
    ==> out_range.data.addr == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && (result.is_Err() && flags.oaddr_type == RMI_ADDR_TYPE_LIST
    ==> RttTreeRangeAllOaddrList(new_s, RealmAt(new_s, rd), RMM_RTT_TREE_PRIMARY as int, base, oaddr.data.list_addr.addr, (out_top) - (base)))
  && (result.is_Err()
    ==> result.status == RMI_SUCCESS)
}