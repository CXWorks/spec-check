pub open spec fn rmi_rtt_init_ripas_spec(rd: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).rtt_addr,RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, top, UNASSIGNED), RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM(0)))
  && (!(RttWalk_(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int)).rtt_addr,RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, 0).level == RMM_RTT_PAGE_LEVEL) ==> ResultEqual(result, RMI_ERROR_RTT(RMM_RTT_PAGE_LEVEL)))
  && (RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int)).rtt_addr, 0).state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RMM_RTT_PAGE_LEVEL)))
  && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (base == RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int)).rtt_addr,RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, top, UNASSIGNED) ==> ResultEqual(result, RMI_ERROR_RTT(RMM_RTT_PAGE_LEVEL)))
  && (result.is_Ok() ==> RealmAt(new_s, rd).state == REALM_ACTIVE)
  && (result.is_Ok() ==> out_top == RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk_(new_s, RttAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).rtt_addr,RttWalk_(new_s, RttAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, top, UNASSIGNED))
  && ((AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       !(top <= base) &&
       AddrIsProtected(old_s, RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).rtt_addr,RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, top, UNASSIGNED), RealmAt(old_s, rd)) &&
       RealmAt(old_s, rd).state == REALM_NEW &&
       (RttWalk_(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int)).rtt_addr,RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, 0).level == RMM_RTT_PAGE_LEVEL) &&
       !(RttEntryAt(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int)).rtt_addr, 0).state != UNASSIGNED) &&
       AddrIsGranuleAligned(old_s, top) &&
       !(base == RttSkipEntriesIfNotState(old_s, RttAt(old_s, RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int)).rtt_addr,RttWalk_(old_s, RttAt(old_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, top, UNASSIGNED)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> RealmAt(new_s, rd).state == RealmAt(old_s, rd).state)
  && (result.is_Err()
    ==> out_top == RttSkipEntriesIfNotState(new_s, RttAt(new_s, RttWalk_(new_s, RttAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).rtt_addr,RttWalk_(new_s, RttAt(new_s, rd), RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int, 0 as int).level, base, top, UNASSIGNED))
}