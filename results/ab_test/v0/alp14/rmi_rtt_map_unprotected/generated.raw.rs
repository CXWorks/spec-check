```verus
pub open spec fn RMI_RTT_MAP_UNPROTECTED_spec(s: S, rd: Address, ipa: Address, level: int, desc: u64) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let rtte = RttDescriptorDecode(s, desc, realm.rtt_s2ap_encoding);
    
    // Failure conditions - must check in order
    if !RttDescriptorIsValidForUnprotected(s, desc) {
        return false;
    }
    if !AddrIsGranuleAligned(s, rd) {
        return false;
    }
    if !PaIsDelegable(s, rd) {
        return false;
    }
    if GranuleAt(s, rd).state != RD {
        return false;
    }
    if !RttLevelIsValid(s, realm, level) || level < 1 {
        return false;
    }
    if !AddrIsRttLevelAligned(s, rtte.addr, level) {
        return false;
    }
    if realm.feat_lpa2 == FEATURE_FALSE && UInt64(rtte.addr) >= 281474976710656u64 {
        return false;
    }
    if !AddrIsRttLevelAligned(s, ipa, level) {
        return false;
    }
    if UInt64(ipa) >= (1u64 << realm.ipa_width) || AddrIsProtected(s, ipa, realm) {
        return false;
    }
    if realm.rtt_s2ap_encoding == S2AP_INDIRECT && 
       rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS &&
       rtte.s2ap_indirect.base_index != S2AP_RO &&
       rtte.s2ap_indirect.base_index != S2AP_WO &&
       rtte.s2ap_indirect.base_index != S2AP_RW {
        return false;
    }
    if walk.level < level {
        return false;
    }
    if walk.rtte.state != UNASSIGNED_NS {
        return false;
    }
    
    // Success conditions
    walk.rtte.state == ASSIGNED_NS &&
    walk.rtte.attr_unprot == rtte.attr_unprot &&
    (realm.rtt_s2ap_encoding == S2AP_DIRECT ==> 
        (walk.rtte.s2ap_direct.read == rtte.s2ap_direct.read &&
         walk.rtte.s2ap_direct.write == rtte.s2ap_direct.write)) &&
    (realm.rtt_s2ap_encoding == S2AP_INDIRECT ==> 
        (walk.rtte.s2ap_indirect.base_index == rtte.s2ap_indirect.base_index &&
         walk.rtte.s2ap_indirect.overlay_index == 15)) &&
    walk.rtte.addr == rtte.addr
}
```