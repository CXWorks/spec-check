pub open spec fn rmi_rtt_destroy_spec(
    result: RmiCommandReturnCode,
    rtt: Address,
    top: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    // Failure conditions
    ((!AddrIsGranuleAligned(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !PaIsDelegable(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(
        old_s,
        rd,
    ).state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((!RttLevelIsValid(
        old_s,
        realm,
        level,
    ) || RttLevelIsStarting(old_s, realm, level))) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsRttLevelAligned(old_s, ipa, level - 1)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    UInt(ipa) >= (1 << realm.ipa_width)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((walk.level
        < level - 1) ==> (ResultEqual(result, RMI_ERROR_RTT, walk.level) && (top == walk_top))) && (
    (walk.rtte.state != TABLE) ==> (ResultEqual(result, RMI_ERROR_RTT, walk.level) && (top
        == walk_top))) && ((RttIsLive(old_s, RttAt(old_s, walk.rtte.addr))) ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
        level,
    ) && (top == ipa))) && ((AddrIsAuxRef(old_s, ipa, realm)) ==> ResultEqual(
        result,
        RMI_ERROR_RTT,
        walk.level,
    ))
    // Success conditions
     && ((walk.level == level - 1 && walk.rtte.state == TABLE && !RttIsLive(
        old_s,
        RttAt(old_s, walk.rtte.addr),
    ) && !AddrIsAuxRef(old_s, ipa, realm)) ==> (result == RMI_SUCCESS && rtt == walk.rtte.addr
        && top == walk_top && (AddrIsProtected(old_s, ipa, realm) ==> (walk.rtte.state == UNASSIGNED
        && walk.rtte.ripas == DESTROYED)) && (!AddrIsProtected(old_s, ipa, realm)
        ==> walk.rtte.state == UNASSIGNED_NS) && GranuleAt(new_s, walk.rtte.addr).state
        == DELEGATED))
}