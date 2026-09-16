pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top) <= (base) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected( ToAddress((top) - RmmGlobal(new_s).dynamic.rmi_granule_size as int), RealmAt(new_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) ==> (ResultEqual(result, RMI_ERROR_RTT(0)) && (result.get_Err_0().level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && ((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA) ==> (ResultEqual(result, RMI_ERROR_RTT(0)) && (result.get_Err_0().level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!AddrIsRmiGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base) == (RttSkipEntriesUnlessVoidData(old_s,  RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top)). ==> (ResultEqual(result, RMI_ERROR_RTT(0)) && (result.get_Err_0().level == RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Err() && result.get_Err_0() != RMI_ERROR_RTT(0) && result.get_Err_0() != RMI_ERROR_RTT_AUX(0) ==> RttEntriesInRangeRipas(new_s, RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr), RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level, base, RttSkipEntriesUnlessVoidData(new_s,  RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top),RIPAS_RAM))
  && (result.is_Ok() ==> RealmAt(new_s, rd).rim == RimExtendRipas(new_s, RealmAt(new_s, rd), base, RttSkipEntriesUnlessVoidData(new_s,  RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top)))
  && (result.is_Ok() ==> out_top == RttSkipEntriesUnlessVoidData(new_s,  RttAt(new_s, RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(new_s, RealmAt(new_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top))
  && ((AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !((top) <= (base)) &&
       AddrIsProtected(old_s,  ToAddress((top) - RmmGlobal(old_s).dynamic.rmi_granule_size as int), RealmAt(old_s, rd)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) &&
       !((RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_VOID && RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != RTTE_DATA)) &&
       AddrIsRmiGranuleAligned(old_s, top) &&
       !((base) == (RttSkipEntriesUnlessVoidData(old_s,  RttAt(old_s, RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk(old_s, RealmAt(old_s, rd), base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rim == RealmAt(old_s, rd).rim)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).rim == RealmAt(old_s, rd).rim)
}