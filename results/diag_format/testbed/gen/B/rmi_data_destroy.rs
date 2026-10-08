pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let entry_idx = RttEntryIndex(ipa, walk.level as int);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level as int, ipa);

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level as int < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level as int)) && top == walk_top))
    && (walk.rtte.state != ASSIGNED ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level as int)) && top == walk_top))
    && (AddrIsAuxLive(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT_AUX(0)))
    && (ResultEqual(result, RMI_SUCCESS) ==> (data == Old(walk.rtte).addr && top == walk_top && GranuleAt(Old(walk.rtte).addr).state == DELEGATED && RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx).state == UNASSIGNED && (Old(walk.rtte).ripas == RAM ==> RttEntryAt(RttAt(old_s, walk.rtt_addr), entry_idx).ripas == DESTROYED)))
}