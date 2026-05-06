```verus
pub open spec fn rmi_rtt_aux_fold_spec(
    result: RmiCommandReturnCode,
    rtt: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: i64,
    index: u64
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level - 1, index as int);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let fold_pre = RttFold(old_s, RttAt(old_s, walk.rtte.addr));
    
    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let level_bound_fail = !RttLevelIsValid(old_s, realm, level as int) || 
                          RttLevelIsStarting(old_s, realm, level as int);
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level as int - 1);
    let ipa_bound_fail = !AddrIsProtected(ipa, realm);
    let index_bound_fail = realm.rtt_tree_per_plane == FEATURE_FALSE || 
                          index == RMM_RTT_TREE_PRIMARY || 
                          index > realm.num_aux_planes;
    let rtt_walk_fail = walk.level < level as int - 1;
    let rtte_state_fail = walk.rtte.state != TABLE;
    let rtt_homo_fail = !RttIsHomogeneous(old_s, RttAt(old_s, walk.rtte.addr));
    
    // Success conditions
    let rtte_state_ok = walk.rtte.state == fold_pre.state;
    let rtte_addr_ok = (fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS) ==>
                       walk.rtte.addr == fold_pre.addr;
    let rtte_attr_prot_ok = (fold_pre.state == ASSIGNED) ==>
                            (RttMemAttrEqual(old_s, walk.rtte, fold_pre, RTT_PROTECTED) &&
                             RttS2APEqual(old_s, walk.rtte, fold_pre, S2AP_INDIRECT));
    let rtte_attr_unprot_ok = (fold_pre.state == ASSIGNED_NS) ==>
                              (RttMemAttrEqual(old_s, walk.rtte, fold_pre, RTT_UNPROTECTED) &&
                               RttS2APEqual(old_s, walk.rtte, fold_pre, realm.rtt_s2ap_encoding));
    let rtte_ripas_ok = AddrIsProtected(ipa, realm) ==> 
                       walk.rtte.ripas == fold_pre.ripas;
    let rtt_state_ok = GranuleAt(new_s, walk.rtte.addr).state == DELEGATED;
    let rtt_ok = rtt == walk.rtte.addr;
    
    // Ordered failure conditions (check in priority order)
    (rd_align_fail || rd_bound_fail || rd_state_fail || level_bound_fail || 
     ipa_align_fail || ipa_bound_fail || index_bound_fail) ==>
        ResultEqual(result, RMI_ERROR_INPUT)
    
    && (rtt_walk_fail ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (rtte_state_fail && !rtt_walk_fail ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && (rtt_homo_fail && !rtt_walk_fail && !rtte_state_fail ==> 
        ResultEqual(result, RMI_ERROR_RTT_AUX))
    
    // Success path
    && ((!rd_align_fail && !rd_bound_fail && !rd_state_fail && !level_bound_fail && 
         !ipa_align_fail && !ipa_bound_fail && !index_bound_fail && !rtt_walk_fail && 
         !rtte_state_fail && !rtt_homo_fail) ==>
        (ResultEqual(result, RMI_SUCCESS) &&
         rtte_state_ok && rtte_addr_ok && rtte_attr_prot_ok && rtte_attr_unprot_ok &&
         rtte_ripas_ok && rtt_state_ok && rtt_ok))
}
```