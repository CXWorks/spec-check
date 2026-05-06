pub open spec fn RMI_RTT_UNMAP_UNPROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let rtte = RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx);

    // Failure: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure: rd_state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure: level_bound
    ((!RttLevelIsValid(old_s, realm, level) || level < 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure: ipa_align
    (!AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure: ipa_bound
    ((UInt(ipa) >= (1 << realm.ipa_width) || AddrIsProtected(ipa, realm)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) &&
    // Failure: rtt_walk
    (walk.level < level ==> (ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1
        == walk.level && top == walk_top))
        &&
    // Failure: rtte_state (when walk succeeded but entry not ASSIGNED_NS)
    ((walk.level >= level && rtte.state != ASSIGNED_NS) ==> (ResultEqual(result, RMI_ERROR_RTT)
        && result.get_Err_0().1 == walk.level && top == walk_top))
        &&
    // Success: when all preconditions pass and entry is ASSIGNED_NS
    ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && RttLevelIsValid(old_s, realm, level) && level >= 1 && AddrIsRttLevelAligned(ipa, level)
        && UInt(ipa) < (1 << realm.ipa_width) && !AddrIsProtected(ipa, realm) && walk.level >= level
        && rtte.state == ASSIGNED_NS) ==> (result.is_Ok() && top == walk_top && RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx,
    ).state == UNASSIGNED_NS))
}