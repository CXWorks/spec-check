```verus
pub open spec fn RMI_RTT_CREATE_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    rtt: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_pre = RttWalk(old_s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
    let rtte_pre = walk_pre.rtte;
    
    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let level_bound_fail = !RttLevelIsValid(old_s, realm, level) || RttLevelIsStarting(old_s, realm, level);
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level - 1);
    let ipa_bound_fail = UInt(ipa) >= (1 << realm.ipa_width);
    let rtt_align_fail = !AddrIsGranuleAligned(rtt);
    let rtt_bound_fail = !PaIsDelegableDram(rtt);
    let rtt_state_fail = GranuleAt(old_s, rtt).state != DELEGATED;
    let rtt_bound2_fail = (realm.feat_lpa2 == FEATURE_FALSE) && (UInt(rtt) >= (1 << 48));
    let rtt_walk_fail = walk.level < level - 1;
    let rtte_state_fail = walk.rtte.state == TABLE;
    
    // Success condition: no failures
    let success = !rd_align_fail && !rd_bound_fail && !rd_state_fail && !level_bound_fail 
        && !ipa_align_fail && !ipa_bound_fail && !rtt_align_fail && !rtt_bound_fail 
        && !rtt_state_fail && !rtt_bound2_fail && !rtt_walk_fail && !rtte_state_fail;
    
    (rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (rd_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (rd_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (level_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (ipa_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (ipa_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (rtt_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (rtt_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (rtt_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (rtt_bound2_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (rtt_walk_fail ==> ResultEqual(result, RMI_ERROR_RTT))
        && (rtte_state_fail ==> ResultEqual(result, RMI_ERROR_RTT))
        && (success ==> (
            result.is_Ok()
            && GranuleAt(new_s, rtt).state == RTT
            && RttAllEntriesState(old_s, RttAt(new_s, rtt), rtte_pre.state)
            && (AddrIsProtected(ipa, realm) ==> RttAllEntriesRipas(old_s, RttAt(new_s, rtt), rtte_pre.ripas))
            && ((rtte_pre.state != UNASSIGNED && rtte_pre.state != UNASSIGNED_NS) 
                ==> RttAllEntriesContiguous(old_s, RttAt(new_s, rtt), rtte_pre.addr, level))
        ))
}
```