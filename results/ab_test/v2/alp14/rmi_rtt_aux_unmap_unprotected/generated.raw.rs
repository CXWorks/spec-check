```verus
pub open spec fn rmi_rtt_aux_unmap_unprotected_spec(
    result: RmiCommandReturnCode,
    top: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    index: u64
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions
    let rd_align_fails = !AddrIsGranuleAligned(rd);
    let rd_bound_fails = !PaIsDelegable(rd);
    let rd_state_fails = GranuleAt(old_s, rd).state != RD;
    let ipa_align_fails = !AddrIsRttLevelAligned(ipa, realm.rtt_level_start);
    let ipa_bound_fails = UInt(ipa) >= (1u64 << realm.ipa_width) || AddrIsProtected(old_s, ipa, realm);
    let index_bound_fails = realm.rtt_tree_per_plane == FEATURE_FALSE 
                            || index == RMM_RTT_TREE_PRIMARY 
                            || index > realm.num_aux_planes;
    
    // If any failure condition is met, return error
    (rd_align_fails ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_bound_fails ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_state_fails ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_align_fails ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_bound_fails ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (index_bound_fails ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // If no failure conditions and success is possible
    && ((!rd_align_fails && !rd_bound_fails && !rd_state_fails 
         && !ipa_align_fails && !ipa_bound_fails && !index_bound_fails)
        ==> (result.is_Ok() 
             && walk.rtte.state == UNASSIGNED_NS
             && top == walk_top
             && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx as int).state == UNASSIGNED_NS))
}
```