pub open spec fn rmi_rtt_init_ripas_spec(result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S, rd: Address, base: Address, top: Address) -> bool {
  let realm_pre = RealmAt(old_s, rd);
  let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
  let walk_top = RttSkipEntriesIfNotState(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, top, UNASSIGNED);
  let realm = RealmAt(new_s, rd);
  
  // Failure conditions
  (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((top as int) <= (base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ToAddress((top as int) - RMM_GRANULE_SIZE), realm_pre) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (realm_pre.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
  && (!AddrIsRttLevelAligned(base, walk.level) ==> ResultEqual(result, RMI_ERROR_RTT))
  && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
  && (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((base as int) == (walk_top as int) ==> ResultEqual(result, RMI_ERROR_RTT))
  
  // Success condition
  && (AddrIsGranuleAligned(rd)
      && PaIsDelegable(rd)
      && GranuleAt(old_s, rd).state == RD
      && (top as int) > (base as int)
      && AddrIsProtected(old_s, ToAddress((top as int) - RMM_GRANULE_SIZE), realm_pre)
      && realm_pre.state == REALM_NEW
      && AddrIsRttLevelAligned(base, walk.level)
      && walk.rtte.state == UNASSIGNED
      && AddrIsGranuleAligned(top)
      && (base as int) != (walk_top as int)
      ==> (result.is_Ok()
           && RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level, base, walk_top, RAM)
           && realm.measurements[0] == RimExtendRipas(old_s, realm_pre, base, walk_top, walk.level)
           && out_top == walk_top))
}