```verus
pub open spec fn RMI_RTT_CREATE_spec(s: S, rd: Address, rtt: Address, ipa: Address, level: int) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY());
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_pre = RttWalk(s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY());
    let rtte_pre = walk_pre.rtte;
    
    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(s, rd);
    let rd_bound_fail = !PaIsDelegable(s, rd);
    let rd_state_fail = GranuleAt(s, rd).state != RD();
    let level_bound_fail = !RttLevelIsValid(s, realm, level) || RttLevelIsStarting(s, realm, level);
    let ipa_align_fail = !AddrIsRttLevelAligned(s, ipa, level - 1);
    let ipa_bound_fail = (ipa as int) >= (1 << realm.ipa_width);
    let rtt_align_fail = !AddrIsGranuleAligned(s, rtt);
    let rtt_bound_fail = !PaIsDelegableDram(s, rtt);
    let rtt_state_fail = GranuleAt(s, rtt).state != DELEGATED();
    let rtt_bound2_fail = (realm.feat_lpa2 == FEATURE_FALSE()) && ((rtt as int) >= (1 << 48));
    let rtt_walk_fail = walk.level < level - 1;
    let rtte_state_fail = walk.rtte.state == TABLE();
    
    // Success conditions
    let rtt_state_succ = GranuleAt(s, rtt).state == RTT();
    let rtte_state_succ = walk.rtte.state == TABLE();
    let rtte_addr_succ = walk.rtte.addr == rtt;
    let rtte_c_ripas_succ = !AddrIsProtected(s, ipa, realm) || RttAllEntriesRipas(s, RttAt(s, rtt), rtte_pre.ripas);
    let rtte_c_state_succ = RttAllEntriesState(s, RttAt(s, rtt), rtte_pre.state);
    let rtte_c_addr_succ = (rtte_pre.state == UNASSIGNED() || rtte_pre.state == UNASSIGNED_NS()) || RttAllEntriesContiguous(s, RttAt(s, rtt), rtte_pre.addr, level);
    
    // If any failure condition is true, the command fails
    if rd_align_fail || rd_bound_fail || rd_state_fail || level_bound_fail || ipa_align_fail || ipa_bound_fail || rtt_align_fail || rtt_bound_fail || rtt_state_fail || rtt_bound2_fail || rtt_walk_fail || rtte_state_fail {
        false
    } else {
        // All success conditions must hold
        rtt_state_succ && rtte_state_succ && rtte_addr_succ && rtte_c_ripas_succ && rtte_c_state_succ && rtte_c_addr_succ
    }
}
```