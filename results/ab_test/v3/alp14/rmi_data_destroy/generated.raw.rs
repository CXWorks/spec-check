pub open spec fn rmi_data_destroy_spec(result: RmiCommandReturnCode, rd: Address, ipa: Address, data: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT) && top == walk_top))
    && (walk.rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT) && top == walk_top))
    && (AddrIsAuxLive(ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
         && AddrIsGranuleAligned(ipa) && AddrIsProtected(ipa, realm)
         && walk.level == RMM_RTT_PAGE_LEVEL && walk.rtte.state == ASSIGNED
         && !AddrIsAuxLive(ipa, realm))
        ==> (result.is_Ok()
             && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
             && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED
             && (walk.rtte.ripas == RAM ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED)
             && (walk.rtte.ripas != RAM ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == walk.rtte.ripas)
             && data == walk.rtte.addr
             && top == walk_top))
}