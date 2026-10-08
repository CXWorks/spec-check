pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok())
  && (PaIsDelegable(old_s, rd) ==> result.is_Ok())
  && (GranuleAt(old_s, rd).state != RD ==> result.is_Ok())
  && ((top) <= (base) ==> result.is_Ok())
  && (!AddrIsProtected(old_s,  ToAddress((top) - 4096), RealmAt(old_s, rd)) ==> result.is_Ok())
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(RealmAt(old_s, rd).state as int)))
  && (!AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (!AddrIsGranuleAligned(old_s, top) ==> result.is_Ok())
  && ((base) == (RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED)) ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int)))
  && (result.is_Ok() ==> RttEntriesInRangeRipas(new_s, RttAt(new_s, RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED), RAM))
  && (result.is_Ok() ==> RealmAt(new_s, rd).measurements[0] == RimExtendRipas(new_s, RealmAt(new_s, rd), base, RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED),RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level))
  && (result.is_Ok() ==> out_top == RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED))
  && ((!(AddrIsGranuleAligned(old_s, rd)) &&
       !(PaIsDelegable(old_s, rd)) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !((top) <= (base)) &&
       AddrIsProtected(old_s,  ToAddress((top) - 4096), RealmAt(old_s, rd)) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsRttLevelAligned(old_s, base, RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level as int) &&
       !(RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtte.state != UNASSIGNED) &&
       AddrIsGranuleAligned(old_s, top) &&
       !((base) == (RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(old_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).measurements[0] == RealmAt(old_s, rd).measurements[0])
  && (result.is_Err()
    ==> RttEntriesInRangeRipas(new_s, RttAt(new_s, RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).rtt_addr),RttWalk_(new_s, rd, base,RMM_RTT_PAGE_LEVEL as int,RMM_RTT_TREE_PRIMARY as int).level,base, top, UNASSIGNED), RAM))
}