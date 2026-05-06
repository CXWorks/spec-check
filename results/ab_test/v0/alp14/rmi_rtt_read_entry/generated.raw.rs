```verus
pub open spec fn RMI_RTT_READ_ENTRY_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
    walk_level: int,
    state: RmiRttEntryState,
    desc: u64,
    ripas: RmiRipas,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let rtte = RttDescriptorDecode(s, desc, realm.rtt_s2ap_encoding);
    
    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(rd) && ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(s, rd).state != RD && ResultEqual(result, RMI_ERROR_INPUT);
    let level_bound_fail = !RttLevelIsValid(s, realm, level) && ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level) && ResultEqual(result, RMI_ERROR_INPUT);
    let ipa_bound_fail = (ipa as int) >= (1 << realm.ipa_width) && ResultEqual(result, RMI_ERROR_INPUT);
    
    let any_fail = rd_align_fail || rd_bound_fail || rd_state_fail || level_bound_fail || ipa_align_fail || ipa_bound_fail;
    
    // Success conditions
    let walk_level_ok = walk_level == walk.level;
    let state_ok = state == RttEntryStateToRmi(s, walk.rtte.state);
    
    let state_invalid_ok = (walk.rtte.state == UNASSIGNED || walk.rtte.state == UNASSIGNED_NS) ==>
        (rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == RMM_FALSE &&
         rtte.s2ap_direct.write == RMM_FALSE &&
         rtte.addr == 0);
    
    let state_prot_ok = (walk.rtte.state == ASSIGNED || walk.rtte.state == ASSIGNED_DEV || 
                         walk.rtte.state == ASSIGNED_VSMMU || walk.rtte.state == TABLE) ==>
        (rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == RMM_FALSE &&
         rtte.s2ap_direct.write == RMM_FALSE &&
         rtte.addr == walk.rtte.addr);
    
    let state_unprot_ok = (walk.rtte.state == ASSIGNED_NS) ==>
        (rtte.attr_unprot == walk.rtte.attr_unprot &&
         rtte.s2ap_indirect.base_index == walk.rtte.s2ap_indirect.base_index &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == walk.rtte.s2ap_direct.read &&
         rtte.s2ap_direct.write == walk.rtte.s2ap_direct.write &&
         rtte.addr == walk.rtte.addr);
    
    let state_io_ok = (walk.rtte.state == ASSIGNED_DEV) ==>
        (rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == RMM_FALSE &&
         rtte.s2ap_direct.write == RMM_FALSE &&
         rtte.addr == walk.rtte.addr);
    
    let state_vsmmu_ok = (walk.rtte.state == ASSIGNED_VSMMU) ==>
        (rtte.attr_unprot == 0 &&
         rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
         rtte.s2ap_indirect.overlay_index == 0 &&
         rtte.s2ap_direct.read == RMM_FALSE &&
         rtte.s2ap_direct.write == RMM_FALSE &&
         rtte.addr == walk.rtte.addr);
    
    let ripas_prot_ok = (walk.rtte.state == UNASSIGNED || walk.rtte.state == ASSIGNED) ==>
        (ripas == RipasToRmi(s, walk.rtte.ripas));
    
    let ripas_unprot_ok = (walk.rtte.state == UNASSIGNED_NS || walk.rtte.state == ASSIGNED_NS) ==>
        (ripas == RMI_EMPTY);
    
    (any_fail || (result.is_Ok() && walk_level_ok && state_ok && state_invalid_ok && state_prot_ok && 
                  state_unprot_ok && state_io_ok && state_vsmmu_ok && ripas_prot_ok && ripas_unprot_ok))
}
```