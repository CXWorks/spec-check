pub open spec fn rmi_data_destroy_spec(result: RmiCommandReturnCode, data: Address, top: Address, rd: Address, ipa: Address, old_s: S, new_s: S) -> bool {
  let realm = RealmAt(old_s, rd);
  let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
  let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
  let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
  
  (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (walk.level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT) && top == walk_top))
  && (walk.rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT) && top == walk_top))
  && (AddrIsAuxLive(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
  && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && AddrIsGranuleAligned(old_s, ipa) && AddrIsProtected(old_s, ipa, realm) && walk.level == RMM_RTT_PAGE_LEVEL && walk.rtte.state == ASSIGNED && !AddrIsAuxLive(old_s, ipa, realm)) ==> (result == RMI_SUCCESS && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED && walk.rtte.state == UNASSIGNED && (walk.rtte.ripas == RAM ==> walk.rtte.ripas == DESTROYED) && data == walk.rtte.addr && top == walk_top))
}