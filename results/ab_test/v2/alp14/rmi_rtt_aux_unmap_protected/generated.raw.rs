```rust
pub open spec fn rmi_rtt_aux_unmap_protected_spec(
    result: RmiCommandReturnCode,
    top: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions (in order of precedence)
    let rd_align_fail = !AddrIsGranuleAligned(old_s, rd);
    let rd_bound_fail = !PaIsDelegable(old_s, rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let ipa_align_fail = !AddrIsGranuleAligned(old_s, ipa);
    let ipa_bound_fail = !AddrIsProtected(old_s, ipa, realm);
    let index_bound_fail = (realm.rtt_tree_per_plane == FEATURE_FALSE
                            || index == RMM_RTT_TREE_PRIMARY
                            || index > realm.num_aux_planes);
    let rtte_state_fail = walk.rtte.state != ASSIGNED;
    
    // Condition ordering: [rd_bound, rd_state] < [rtte_state]
    //                     [ipa_bound, index_bound] < [rtte_state]
    let early_fail = rd_align_fail || ipa_align_fail || (rd_bound_fail || rd_state_fail || ipa_bound_fail || index_bound_fail);
    
    (rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_bound_fail && !rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_state_fail && !rd_align_fail && !rd_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_align_fail && !rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_bound_fail && !early_fail && !rtte_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (index_bound_fail && !early_fail && !rtte_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rtte_state_fail && !early_fail ==> (ResultEqual(result, RMI_ERROR_RTT_AUX) && top == walk_top))
    
    // Success conditions
    && (!early_fail && !rtte_state_fail ==> (walk.rtte.state == ASSIGNED ==> (
        RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx as int).state == UNASSIGNED
        && top == walk_top
    )))
}
```