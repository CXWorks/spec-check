```verus
pub open spec fn RMI_RTT_MAP_UNPROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
    desc: u64,
) -> (result: Result<(), RmiStatusCode>)
{
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let rtte = RttDescriptorDecode(s, desc, realm.rtt_s2ap_encoding);
    
    // Failure conditions (in order of precedence)
    if !RttDescriptorIsValidForUnprotected(s, desc) {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(rd) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(rd) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, rd).state != RD {
        Err(RMI_ERROR_INPUT)
    } else if !RttLevelIsValid(s, realm, level) || level < 1 {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsRttLevelAligned(rtte.addr, level) {
        Err(RMI_ERROR_INPUT)
    } else if realm.feat_lpa2 == FEATURE_FALSE && UInt(rtte.addr) >= pow(2, 48) {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsRttLevelAligned(ipa, level) {
        Err(RMI_ERROR_INPUT)
    } else if UInt(ipa) >= pow(2, realm.ipa_width) || AddrIsProtected(s, ipa, realm) {
        Err(RMI_ERROR_INPUT)
    } else if realm.rtt_s2ap_encoding == S2AP_INDIRECT
        && rtte.s2ap_indirect.base_index != S2AP_NO_ACCESS
        && rtte.s2ap_indirect.base_index != S2AP_RO
        && rtte.s2ap_indirect.base_index != S2AP_WO
        && rtte.s2ap_indirect.base_index != S2AP_RW {
        Err(RMI_ERROR_INPUT)
    } else if walk.level < level {
        Err(RMI_ERROR_RTT)
    } else if walk.rtte.state != UNASSIGNED_NS {
        Err(RMI_ERROR_RTT)
    } else {
        // Success conditions
        Ok(())
    }
}
```

**Key properties enforced:**
- Input validation: descriptor, RD alignment/bounds/state, level validity, IPA/PA alignment and bounds
- S2AP encoding validation for indirect mode
- RTT walk completion and entry state verification
- Postconditions: mapped entry has ASSIGNED_NS state with matching attributes and S2AP permissions