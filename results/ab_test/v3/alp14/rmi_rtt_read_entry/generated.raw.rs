```verus
pub open spec fn RMI_RTT_READ_ENTRY_spec(
    old_s: S,
    rd: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    walk_level: u64,
    state: u8,
    desc: u64,
    ripas: u8,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let rtte = RttDescriptorDecode(old_s, desc, realm.rtt_s2ap_encoding);

    (
        // Failure: rd_align
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        // Failure: rd_bound
        (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        // Failure: rd_state
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        // Failure: level_bound
        (!RttLevelIsValid(old_s, realm, level) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        // Failure: ipa_align
        (!AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        // Failure: ipa_bound
        (UInt(ipa) >= (1 << realm.ipa_width) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        // Success: walk_level
        (result.is_Ok() ==> walk_level == walk.level as u64) &&
        // Success: state
        (result.is_Ok() ==> state == RttEntryStateToRmi(old_s, walk.rtte.state) as u8) &&
        // Success: state_invalid
        ((walk.rtte.state == UNASSIGNED || walk.rtte.state == UNASSIGNED_NS) ==> 
            (rtte.attr_unprot == 0 && 
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == 0)) &&
        // Success: state_prot
        ((walk.rtte.state == ASSIGNED || walk.rtte.state == ASSIGNED_DEV || 
          walk.rtte.state == ASSIGNED_VSMMU || walk.rtte.state == TABLE) ==>
            (rtte.attr_unprot == 0 &&
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == walk.rtte.addr)) &&
        // Success: state_unprot
        (walk.rtte.state == ASSIGNED_NS ==>
            (rtte.attr_unprot == walk.rtte.attr_unprot &&
             rtte.s2ap_indirect.base_index == walk.rtte.s2ap_indirect.base_index &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == walk.rtte.s2ap_direct.read &&
             rtte.s2ap_direct.write == walk.rtte.s2ap_direct.write &&
             rtte.addr == walk.rtte.addr)) &&
        // Success: state_io
        (walk.rtte.state == ASSIGNED_DEV ==>
            (rtte.attr_unprot == 0 &&
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == walk.rtte.addr)) &&
        // Success: state_vsmmu
        (walk.rtte.state == ASSIGNED_VSMMU ==>
            (rtte.attr_unprot == 0 &&
             rtte.s2ap_indirect.base_index == S2AP_NO_ACCESS &&
             rtte.s2ap_indirect.overlay_index == 0 &&
             rtte.s2ap_direct.read == RMM_FALSE &&
             rtte.s2ap_direct.write == RMM_FALSE &&
             rtte.addr == walk.rtte.addr)) &&
        // Success: ripas_prot
        ((walk.rtte.state == UNASSIGNED || walk.rtte.state == ASSIGNED) ==>
            ripas as int == RipasToRmi(old_s, walk.rtte.ripas) as int) &&
        // Success: ripas_unprot
        ((walk.rtte.state == UNASSIGNED_NS || walk.rtte.state == ASSIGNED_NS) ==>
            ripas as int == RMI_EMPTY as int)
    )
}
```