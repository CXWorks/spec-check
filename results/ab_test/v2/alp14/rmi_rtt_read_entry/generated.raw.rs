```verus
pub open spec fn rmi_rtt_read_entry_spec(
    result: RmiCommandReturnCode,
    walk_level: u64,
    state: u8,
    desc: u64,
    ripas: u8,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    level: i64
) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(realm, ipa, level as int, RMM_RTT_TREE_PRIMARY);
    let rtte = RttDescriptorDecode(desc, realm.rtt_s2ap_encoding);
    
    // Failure conditions
    (!AddrIsGranuleAligned(rd) ==> result == RMI_ERROR_INPUT) &&
    (!PaIsDelegable(rd) ==> result == RMI_ERROR_INPUT) &&
    (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT) &&
    (!RttLevelIsValid(realm, level as int) ==> result == RMI_ERROR_INPUT) &&
    (!AddrIsRttLevelAligned(ipa, level as int) ==> result == RMI_ERROR_INPUT) &&
    (UInt(ipa) >= pow(2, realm.ipa_width) ==> result == RMI_ERROR_INPUT) &&
    
    // Success conditions
    (AddrIsGranuleAligned(rd) && 
     PaIsDelegable(rd) &&
     GranuleAt(old_s, rd).state == RD &&
     RttLevelIsValid(realm, level as int) &&
     AddrIsRttLevelAligned(ipa, level as int) &&
     UInt(ipa) < pow(2, realm.ipa_width) ==>
        result == RMI_SUCCESS &&
        walk_level == walk.level as u64 &&
        state == RttEntryStateToRmi(old_s, walk.rtte.state) as u8 &&
        
        // state_invalid condition
        ((walk.rtte.state == UNASSIGNED || walk.rtte.state == UNASSIGNED_NS) ==>
            (rtte.attr_unprot == Zeros(3) &&
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == Zeros(ADDRESS_WIDTH))) &&
        
        // state_prot condition
        ((walk.rtte.state == ASSIGNED ||
          walk.rtte.state == ASSIGNED_DEV ||
          walk.rtte.state == ASSIGNED_VSMMU ||
          walk.rtte.state == TABLE) ==>
            (rtte.attr_unprot == Zeros(3) &&
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == walk.rtte.addr)) &&
        
        // state_unprot condition
        (walk.rtte.state == ASSIGNED_NS ==>
            (rtte.attr_unprot == walk.rtte.attr_unprot &&
             rtte.s2ap_indirect.base_index == walk.rtte.s2ap_indirect.base_index &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == walk.rtte.s2ap_direct.read &&
             rtte.s2ap_direct.write == walk.rtte.s2ap_direct.write &&
             rtte.addr == walk.rtte.addr)) &&
        
        // state_io condition
        (walk.rtte.state == ASSIGNED_DEV ==>
            (rtte.attr_unprot == Zeros(3) &&
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == walk.rtte.addr)) &&
        
        // state_vsmmu condition
        (walk.rtte.state == ASSIGNED_VSMMU ==>
            (rtte.attr_unprot == Zeros(3) &&
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == walk.rtte.addr)) &&
        
        // ripas_prot condition
        ((walk.rtte.state == UNASSIGNED || walk.rtte.state == ASSIGNED) ==>
            ripas == RipasToRmi(old_s, walk.rtte.ripas) as u8) &&
        
        // ripas_unprot condition
        ((walk.rtte.state == UNASSIGNED_NS || walk.rtte.state == ASSIGNED_NS) ==>
            ripas == RMI_EMPTY as u8))
}
```