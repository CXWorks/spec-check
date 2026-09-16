pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && ((top) <= (base) ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsProtected(old_s,  ToAddress((top) - RmmGlobal(new_s).dynamic.rmi_granule_size),RealmAt(new_s, rd)) ==> result.status == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result.status == RMI_ERROR_REALM(0))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) ==> (result.status == RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA) ==> (result.status == RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> result.status == RMI_ERROR_INPUT)
  && ((base) == (RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr) ==> (result.status == RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result != RMI_SUCCESS ==> result.status == RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int))
  && (result == RMI_SUCCESS ==> RttEntriesInRangeRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, out_top,RIPAS_RAM))
  && (result == RMI_SUCCESS ==> RealmAt(new_s, rd).rim == RimExtendRipas(new_s, RealmAt(new_s, rd), base, out_top))
  && (result == RMI_SUCCESS ==> out_top == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !((top) <= (base)) &&
       AddrIsProtected(old_s,  ToAddress((top) - RmmGlobal(new_s).dynamic.rmi_granule_size),RealmAt(new_s, rd)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((base) == (RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.addr)))
    ==> result == RMI_SUCCESS)
  && (result == RMI_ERROR_INPUT
    ==> RealmAt(new_s, rd).rim == RealmAt(old_s, rd).rim)
  && (result == RMI_ERROR_INPUT
    ==> RealmAt(new_s, rd).rim == RealmAt(old_s, rd).rim)
}