```verus
pub open spec fn RMI_DATA_DESTROY_spec(s: S, rd: Address, ipa: Address) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions (ordered)
    // rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(/*result*/ Err(RMI_ERROR_INPUT), RMI_ERROR_INPUT)) &&
    // rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(/*result*/ Err(RMI_ERROR_INPUT), RMI_ERROR_INPUT)) &&
    // rd_state
    (GranuleAt(s, rd).state != RD ==> ResultEqual(/*result*/ Err(RMI_ERROR_INPUT), RMI_ERROR_INPUT)) &&
    // ipa_align
    (!AddrIsGranuleAligned(ipa) ==> ResultEqual(/*result*/ Err(RMI_ERROR_INPUT), RMI_ERROR_INPUT)) &&
    // ipa_bound
    (!AddrIsProtected(s, ipa, realm) ==> ResultEqual(/*result*/ Err(RMI_ERROR_INPUT), RMI_ERROR_INPUT)) &&
    // rtt_walk
    (walk.level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(/*result*/ Err(RMI_ERROR_RTT), RMI_ERROR_RTT) && walk_top >= walk.rtte.addr)) &&
    // rtte_state
    (walk.rtte.state != ASSIGNED ==> (ResultEqual(/*result*/ Err(RMI_ERROR_RTT), RMI_ERROR_RTT) && walk_top >= walk.rtte.addr)) &&
    // aux_live
    (AddrIsAuxLive(s, ipa, realm) ==> ResultEqual(/*result*/ Err(RMI_ERROR_RTT_AUX), RMI_ERROR_RTT_AUX)) &&
    
    // Success conditions
    (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(s, rd).state == RD &&
     AddrIsGranuleAligned(ipa) && AddrIsProtected(s, ipa, realm) &&
     walk.level == RMM_RTT_PAGE_LEVEL && walk.rtte.state == ASSIGNED &&
     !AddrIsAuxLive(s, ipa, realm) ==>
     (GranuleAt(s, walk.rtte.addr).state == DELEGATED &&
      walk.rtte.state == UNASSIGNED &&
      (walk.rtte.ripas == RAM ==> walk.rtte.ripas == DESTROYED) &&
      walk_top > ipa))
}
```