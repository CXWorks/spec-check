pub open spec fn rmi_data_destroy_spec(rd: Address, ipa: Address, result: Result<(), RmiStatusCode>, data: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let inputs_ok = AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, ipa)
        && AddrIsProtected(old_s, ipa, realm);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!inputs_ok ==> top == 0)
    && ((inputs_ok && walk.level < RMM_RTT_PAGE_LEVEL) ==>
        (ResultEqual(result, RMI_ERROR_RTT(walk.level)) && top == walk_top))
    && ((inputs_ok && walk.rtte.state != ASSIGNED) ==>
        (ResultEqual(result, RMI_ERROR_RTT(walk.level)) && top == walk_top))
    && ((inputs_ok
        && walk.level == RMM_RTT_PAGE_LEVEL
        && walk.rtte.state == ASSIGNED
        && AddrIsAuxLive(old_s, ipa, realm)) ==>
        ResultEqual(result, RMI_ERROR_RTT_AUX(0)))
    && (result.is_Err() ==> new_s == old_s)
    && ((inputs_ok
        && walk.level == RMM_RTT_PAGE_LEVEL
        && walk.rtte.state == ASSIGNED
        && !AddrIsAuxLive(old_s, ipa, realm)) ==>
        (result.is_Ok()
        && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
        && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED
        && (walk.rtte.ripas == RAM ==>
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED)
        && (walk.rtte.ripas != RAM ==>
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == walk.rtte.ripas)
        && data == walk.rtte.addr
        && top == walk_top))
}
