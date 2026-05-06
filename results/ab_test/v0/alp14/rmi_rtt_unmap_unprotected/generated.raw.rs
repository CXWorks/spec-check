```verus
pub open spec fn RMI_RTT_UNMAP_UNPROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    top: Address,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY());
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions (in order)
    let rd_align_fail = !AddrIsGranuleAligned(s, rd);
    let rd_bound_fail = !PaIsDelegable(s, rd);
    let rd_state_fail = GranuleAt(s, rd).state != RD();
    let level_bound_fail = !RttLevelIsValid(s, realm, level) || level < 1;
    let ipa_align_fail = !AddrIsRttLevelAligned(s, ipa, level);
    let ipa_bound_fail = UInt(ipa) >= (1 << realm.ipa_width) || AddrIsProtected(s, ipa, realm);
    let rtt_walk_fail = walk.level < level;
    let rtte_state_fail = walk.rtte.state != ASSIGNED_NS();
    
    // Check all conditions in order
    (rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
    (!rd_align_fail ==> (
        (rd_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        (!rd_bound_fail ==> (
            (rd_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
            (!rd_state_fail ==> (
                (level_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
                (!level_bound_fail ==> (
                    (ipa_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
                    (!ipa_align_fail ==> (
                        (ipa_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
                        (!ipa_bound_fail ==> (
                            (rtt_walk_fail ==> (
                                ResultEqual(result, RMI_ERROR_RTT(walk.level)) &&
                                top == walk_top
                            )) &&
                            (!rtt_walk_fail ==> (
                                (rtte_state_fail ==> (
                                    ResultEqual(result, RMI_ERROR_RTT(walk.level)) &&
                                    top == walk_top
                                )) &&
                                (!rtte_state_fail ==> (
                                    result == RMI_SUCCESS() &&
                                    walk.rtte.state == UNASSIGNED_NS() &&
                                    top == walk_top
                                ))
                            ))
                        ))
                    ))
                ))
            ))
        ))
    ))
}
```