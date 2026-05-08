```verus
pub open spec fn rmi_data_destroy_spec(
    result: RmiCommandReturnCode,
    data: Address,
    top: Address,
    old_s: S,
    new_s: S
) -> bool {
    let rd = old_s.cmd_input_x1;
    let ipa = old_s.cmd_input_x2;
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions (ordered by precedence)
    let rd_align_fail = !AddrIsGranuleAligned(old_s, rd);
    let rd_bound_fail = !PaIsDelegable(old_s, rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let ipa_align_fail = !AddrIsGranuleAligned(old_s, ipa);
    let ipa_bound_fail = !AddrIsProtected(old_s, ipa, realm);
    let rtt_walk_fail = walk.level < RMM_RTT_PAGE_LEVEL;
    let rtte_state_fail = walk.rtte.state != ASSIGNED;
    let aux_live_fail = AddrIsAuxLive(old_s, ipa, realm);
    
    // Input validation failures (checked first)
    (rd_align_fail ==> result == RMI_ERROR_INPUT) &&
    (rd_bound_fail ==> result == RMI_ERROR_INPUT) &&
    (rd_state_fail ==> result == RMI_ERROR_INPUT) &&
    (ipa_align_fail ==> result == RMI_ERROR_INPUT) &&
    (ipa_bound_fail ==> result == RMI_ERROR_INPUT) &&
    
    // RTT and entry state failures
    ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !ipa_align_fail && !ipa_bound_fail && rtt_walk_fail)
        ==> (result.is_err() && result.get_err() == RMI_ERROR_RTT && top == walk_top)) &&
    
    ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !ipa_align_fail && !ipa_bound_fail && !rtt_walk_fail && rtte_state_fail)
        ==> (result.is_err() && result.get_err() == RMI_ERROR_RTT && top == walk_top)) &&
    
    // Auxiliary live failure
    ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !ipa_align_fail && !ipa_bound_fail && !rtt_walk_fail && !rtte_state_fail && aux_live_fail)
        ==> result == RMI_ERROR_RTT_AUX) &&
    
    // Success conditions
    ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !ipa_align_fail && !ipa_bound_fail && !rtt_walk_fail && !rtte_state_fail && !aux_live_fail)
        ==> (
            result == RMI_SUCCESS &&
            GranuleAt(new_s, walk.rtte.addr).state == DELEGATED &&
            RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED &&
            (walk.rtte.ripas == RAM ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED) &&
            (walk.rtte.ripas != RAM ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == walk.rtte.ripas) &&
            data == walk.rtte.addr &&
            top == walk_top
        ))
}
```