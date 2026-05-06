pub open spec fn rmi_rtt_unmap_unprotected_spec(
    result: RmiCommandReturnCode,
    top: Address,
    old_s: S,
    new_s: S,
) -> bool {
    let rd = old_s.get_input_x1();
    let ipa = old_s.get_input_x2();
    let level = old_s.get_input_x3() as int;

    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let level_bound_fail = !RttLevelIsValid(old_s, realm, level) || level < 1;
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level);
    let ipa_bound_fail = UInt(ipa) >= (pow(2, realm.ipa_width)) || AddrIsProtected(ipa, realm);
    let rtt_walk_fail = walk.level < level;
    let rtte_state_fail = walk.rtte.state != ASSIGNED_NS;

    // Input validation errors
    (rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (rd_bound_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (rd_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (level_bound_fail
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (ipa_align_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (ipa_bound_fail ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // RTT walk and entry state errors
     && ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !level_bound_fail && !ipa_align_fail
        && !ipa_bound_fail && rtt_walk_fail) ==> (ResultEqual(result, RMI_ERROR_RTT) && top
        == walk_top)) && ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !level_bound_fail
        && !ipa_align_fail && !ipa_bound_fail && !rtt_walk_fail && rtte_state_fail) ==> (
    ResultEqual(result, RMI_ERROR_RTT) && top
        == walk_top))
    // Success condition
     && ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !level_bound_fail && !ipa_align_fail
        && !ipa_bound_fail && !rtt_walk_fail && !rtte_state_fail) ==> (result == RMI_SUCCESS && top
        == walk_top && RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).state
        == ASSIGNED_NS && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state
        == UNASSIGNED_NS))
}