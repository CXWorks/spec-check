pub open spec fn rmi_rtt_destroy_spec(rd: Address, ipa: Address, level: Int64, result: Result<(), RmiStatusCode>, rtt: Address, top: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, (level as int) - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let input_ok = AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level as int)
        && !RttLevelIsStarting(old_s, realm, level as int)
        && AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1)
        && (ipa as int) < (pow2(realm.ipa_width as nat) as int);
    let rtt_live = RttIsLive(old_s, RttAt(old_s, walk.rtte.addr));
    let aux_ref = AddrIsAuxRef(old_s, ipa, realm);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level as int) || RttLevelIsStarting(old_s, realm, level as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, (level as int) - 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ipa as int) >= (pow2(realm.ipa_width as nat) as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((input_ok && walk.level < (level as int) - 1) ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level)) && top == walk_top))
    && ((input_ok && walk.rtte.state != TABLE) ==> (ResultEqual(result, RMI_ERROR_RTT(walk.level)) && top == walk_top))
    && ((input_ok && walk.level == (level as int) - 1 && walk.rtte.state == TABLE && rtt_live) ==> (ResultEqual(result, RMI_ERROR_RTT(level as int)) && top == ipa))
    && ((input_ok && walk.level == (level as int) - 1 && walk.rtte.state == TABLE && !rtt_live && aux_ref) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((input_ok && walk.level == (level as int) - 1 && walk.rtte.state == TABLE && !rtt_live && !aux_ref) ==> (
        result.is_Ok()
        && (AddrIsProtected(old_s, ipa, realm) ==> (
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED
            && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED))
        && (!AddrIsProtected(old_s, ipa, realm) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED_NS)
        && GranuleAt(new_s, walk.rtte.addr).state == DELEGATED
        && rtt == walk.rtte.addr
        && top == walk_top))
}
