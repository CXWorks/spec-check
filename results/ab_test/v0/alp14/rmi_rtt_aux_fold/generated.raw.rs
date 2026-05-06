```verus
pub open spec fn RMI_RTT_AUX_FOLD_spec(s: S, rd: Address, ipa: Address, level: int, index: u64) -> bool {
  let realm = RealmAt(s, rd);
  let walk = RttWalk(s, realm, ipa, level - 1, index);
  let entry_idx = RttEntryIndex(s, ipa, walk.level);
  let fold_pre = RttFold(s, RttAt(s, walk.rtte.addr));
  
  // Failure conditions
  let rd_align_fail = !AddrIsGranuleAligned(rd);
  let rd_bound_fail = !PaIsDelegable(rd);
  let rd_state_fail = GranuleAt(s, rd).state != RmmGranuleState::RD;
  let level_bound_fail = !RttLevelIsValid(s, realm, level) || RttLevelIsStarting(s, realm, level);
  let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level - 1);
  let ipa_bound_fail = !AddrIsProtected(ipa, realm);
  let index_bound_fail = realm.rtt_tree_per_plane == RmmFeatureValue::FEATURE_FALSE || 
                         index == RMM_RTT_TREE_PRIMARY || 
                         index > realm.num_aux_planes;
  let rtt_walk_fail = walk.level < level - 1;
  let rtte_state_fail = walk.rtte.state != RmmRttEntryState::TABLE;
  let rtt_homo_fail = !RttIsHomogeneous(s, RttAt(s, walk.rtte.addr));
  
  // Success conditions
  let rtte_state_success = walk.rtte.state == fold_pre.state;
  let rtte_addr_success = (fold_pre.state != RmmRttEntryState::UNASSIGNED && 
                           fold_pre.state != RmmRttEntryState::UNASSIGNED_NS) ==> 
                          walk.rtte.addr == fold_pre.addr;
  let rtte_attr_prot_success = fold_pre.state == RmmRttEntryState::ASSIGNED ==>
                               (RttMemAttrEqual(s, walk.rtte, fold_pre, RmmRttProtected::RTT_PROTECTED) &&
                                RttS2APEqual(s, walk.rtte, fold_pre, RmmRttS2APEncoding::S2AP_INDIRECT));
  let rtte_attr_unprot_success = fold_pre.state == RmmRttEntryState::ASSIGNED_NS ==>
                                 (RttMemAttrEqual(s, walk.rtte, fold_pre, RmmRttProtected::RTT_UNPROTECTED) &&
                                  RttS2APEqual(s, walk.rtte, fold_pre, realm.rtt_s2ap_encoding));
  let rtte_ripas_success = AddrIsProtected(ipa, realm) ==> walk.rtte.ripas == fold_pre.ripas;
  let rtt_state_success = GranuleAt(s, walk.rtte.addr).state == RmmGranuleState::DELEGATED;
  
  // Check conditions based on which failures occur
  if rd_align_fail || rd_bound_fail || rd_state_fail || 
     level_bound_fail || ipa_align_fail || ipa_bound_fail || 
     index_bound_fail {
    false // Input validation failures
  } else if rtt_walk_fail || rtte_state_fail || rtt_homo_fail {
    false // RTT operation failures
  } else {
    rtte_state_success &&
    rtte_addr_success &&
    rtte_attr_prot_success &&
    rtte_attr_unprot_success &&
    rtte_ripas_success &&
    rtt_state_success
  }
}
```