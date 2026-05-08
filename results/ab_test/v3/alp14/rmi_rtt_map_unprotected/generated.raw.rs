```verus
pub open spec fn RMI_RTT_MAP_UNPROTECTED_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    desc: u64,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);

    // Failure condition: attr_valid
    (!RttDescriptorIsValidForUnprotected(old_s, desc) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rd_state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: level_bound
    ((!RttLevelIsValid(old_s, realm, level) || level < 1) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: addr_align
    (!AddrIsRttLevelAligned(rtte.addr, level) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: addr_bound
    ((realm.feat_lpa2 == FEATURE_FALSE && UInt(rtte.addr) >= 281474976710656) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: ipa_align
    (!AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: ipa_bound
    ((UInt(ipa) >= (1 << realm.ipa_width) || AddrIsProtected(ipa, realm)) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: s2ap_indirect_bound
    ((realm.rtt_s2ap_encoding == S2AP_INDIRECT &&
      rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS &&
      rtte.s2ap_indirect.base_index != S2AP_RO &&
      rtte.s2ap_indirect.base_index != S2AP_WO &&
      rtte.s2ap_indirect.base_index != S2AP_RW) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: rtt_walk
    (walk.level < level ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT) &&
    
    // Failure condition: rtte_state
    (walk.rtte.state != UNASSIGNED_NS ==> result.is_Err() && result.get_Err_0() == RMI_ERROR_RTT) &&
    
    // Success condition: all preconditions pass
    (RttDescriptorIsValidForUnprotected(old_s, desc) &&
     AddrIsGranuleAligned(rd) &&
     PaIsDelegable(rd) &&
     GranuleAt(old_s, rd).state == RD &&
     RttLevelIsValid(old_s, realm, level) &&
     level >= 1 &&
     AddrIsRttLevelAligned(rtte.addr, level) &&
     (realm.feat_lpa2 != FEATURE_FALSE || UInt(rtte.addr) < 281474976710656) &&
     AddrIsRttLevelAligned(ipa, level) &&
     UInt(ipa) < (1 << realm.ipa_width) &&
     !AddrIsProtected(ipa, realm) &&
     (realm.rtt_s2ap_encoding != S2AP_INDIRECT ||
      rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS ||
      rtte.s2ap_indirect.base_index == S2AP_RO ||
      rtte.s2ap_indirect.base_index == S2AP_WO ||
      rtte.s2ap_indirect.base_index == S2AP_RW) &&
     walk.level >= level &&
     walk.rtte.state == UNASSIGNED_NS ==>
     (result.is_Ok() &&
      RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).state == ASSIGNED_NS &&
      RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).attr_unprot == rtte.attr_unprot &&
      (realm.rtt_s2ap_encoding != S2AP_DIRECT ||
       (RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).s2ap_direct.read == rtte.s2ap_direct.read &&
        RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).s2ap_direct.write == rtte.s2ap_direct.write)) &&
      (realm.rtt_s2ap_encoding != S2AP_INDIRECT ||
       (RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).s2ap_indirect.base_index == rtte.s2ap_indirect.base_index &&
        RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).s2ap_indirect.overlay_index == 15)) &&
      RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).addr == rtte.addr))
}
```