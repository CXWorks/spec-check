```verus
pub open spec fn RMI_RTT_FOLD_spec(s: S, rd: Address, ipa: Address, level: int) -> bool {
  let realm = RealmAt(s, rd);
  let walk = RttWalk(s, realm, ipa, level - 1, RMM_RTT_TREE_PRIMARY);
  let entry_idx = RttEntryIndex(s, ipa, walk.level);
  let fold_pre = RttFold(s, RttAt(s, walk.rtte.addr));
  
  // Failure conditions
  let rd_align_fail = !AddrIsGranuleAligned(s, rd);
  let rd_bound_fail = !PaIsDelegable(s, rd);
  let rd_state_fail = GranuleAt(s, rd).state != RD;
  let level_bound_fail = !RttLevelIsValid(s, realm, level) || RttLevelIsStarting(s, realm, level);
  let ipa_align_fail = !AddrIsRttLevelAligned(s, ipa, level - 1);
  let ipa_bound_fail = (ipa as int) >= (1 << realm.ipa_width);
  let rtt_walk_fail = walk.level < level - 1;
  let rtte_state_fail = walk.rtte.state != TABLE;
  let rtt_homo_fail = !RttIsHomogeneous(s, RttAt(s, walk.rtte.addr));
  let aux_ref_fail = AddrIsAuxRef(s, ipa, realm);
  
  // Success conditions
  let rtte_state_success = walk.rtte.state == fold_pre.state;
  let rtte_addr_success = (fold_pre.state != UNASSIGNED && fold_pre.state != UNASSIGNED_NS) ==> walk.rtte.addr == fold_pre.addr;
  let rtte_attr_prot_success = fold_pre.state == ASSIGNED ==> (RttMemAttrEqual(walk.rtte, fold_pre, RTT_PROTECTED) && RttS2APEqual(walk.rtte, fold_pre, S2AP_INDIRECT));
  let rtte_attr_unprot_success = fold_pre.state == ASSIGNED_NS ==> (RttMemAttrEqual(walk.rtte, fold_pre, RTT_UNPROTECTED) && RttS2APEqual(walk.rtte, fold_pre, realm.rtt_s2ap_encoding));
  let rtte_ripas_success = AddrIsProtected(s, ipa, realm) ==> walk.rtte.ripas == fold_pre.ripas;
  let rtt_state_success = GranuleAt(s, walk.rtte.addr).state == DELEGATED;
  
  // Condition ordering constraints
  let ordering1 = (rd_bound_fail || rd_state_fail) ==> (rtt_walk_fail || rtte_state_fail || rtt_homo_fail || aux_ref_fail);
  let ordering2 = (level_bound_fail || ipa_bound_fail) ==> (rtt_walk_fail || rtte_state_fail);
  
  (rd_align_fail || rd_bound_fail || rd_state_fail || level_bound_fail || ipa_align_fail || ipa_bound_fail || rtt_walk_fail || rtte_state_fail || rtt_homo_fail || aux_ref_fail) ||
  (rtte_state_success && rtte_addr_success && rtte_attr_prot_success && rtte_attr_unprot_success && rtte_ripas_success && rtt_state_success && ordering1 && ordering2)
}
```