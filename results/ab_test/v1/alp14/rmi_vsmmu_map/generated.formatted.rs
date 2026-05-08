# Generate the Verus specification function for RMI_VSMMU_MAP

```verus
pub open spec fn RMI_VSMMU_MAP_spec(
    s: S,
    rd: Address,
    vsmmu_ptr: Address,
    ipa: Address,
    level: int,
) -> Result<(), RmiStatusCode> {
    // Failure conditions - ordered by precedence
    
    // da_supp: DA feature must be supported
    if !ImplFeatures().feat_da {
        return Err(RMI_ERROR_NOT_SUPPORTED);
    }
    
    // rd_align: RD must be granule aligned
    if !AddrIsGranuleAligned(rd) {
        return Err(RMI_ERROR_INPUT);
    }
    
    // vsmmu_align: VSMMU must be granule aligned
    if !AddrIsGranuleAligned(vsmmu_ptr) {
        return Err(RMI_ERROR_INPUT);
    }
    
    // ipa_align: IPA must be aligned to RTT level
    if !AddrIsRttLevelAligned(ipa, level) {
        return Err(RMI_ERROR_INPUT);
    }
    
    // level_bound: Level must be valid and >= 2
    if !RttLevelIsValid(RealmAt(rd), level) || level < 2 {
        return Err(RMI_ERROR_INPUT);
    }
    
    // rd_bound: RD must be delegable
    if !PaIsDelegable(rd) {
        return Err(RMI_ERROR_INPUT);
    }
    
    // rd_state: RD granule state must be RD
    if GranuleAt(rd).state != RmmGranuleState::RD {
        return Err(RMI_ERROR_INPUT);
    }
    
    // realm_state: Realm state must be REALM_NEW
    if RealmAt(rd).state != RmmRealmState::REALM_NEW {
        return Err(RMI_ERROR_INPUT);
    }
    
    // vsmmu_bound: VSMMU must be delegable
    if !PaIsDelegable(vsmmu_ptr) {
        return Err(RMI_ERROR_INPUT);
    }
    
    // vsmmu_state: VSMMU granule state must be VSMMU
    if GranuleAt(vsmmu_ptr).state != RmmGranuleState::VSMMU {
        return Err(RMI_ERROR_INPUT);
    }
    
    let realm = RealmAt(rd);
    let vsmmu = VsmmuAt(vsmmu_ptr);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    
    // ipa_bound: IPA must be within VSMMU register range
    if UInt(ipa) < UInt(vsmmu.reg_base) {
        return Err(RMI_ERROR_INPUT);
    }
    
    // rtt_walk: Walk must reach the requested level
    if walk.level < level {
        return Err(RMI_ERROR_RTT(walk.level));
    }
    
    // rtte_state: RTTE must be UNASSIGNED
    if walk.rtte.state != RmmRttEntryState::UNASSIGNED {
        return Err(RMI_ERROR_RTT(walk.level));
    }
    
    // rtte_ripas: RTTE RIPAS must be EMPTY
    if walk.rtte.ripas != RmmRipas::EMPTY {
        return Err(RMI_ERROR_RTT(walk.level));
    }
    
    // rtte_bound: IPA range must not exceed VSMMU register top
    if UInt(ipa) + (RttLevelSize(s, walk.level) - 1) >= UInt(vsmmu.reg_top) {
        return Err(RMI_ERROR_RTT(walk.level));
    }
    
    // Success: All conditions satisfied
    Ok(())
}
```

This specification function:
- Follows the RMI_VSMMU_MAP command specification exactly
- Implements all failure conditions in the proper order as defined in the specification
- Returns appropriate error codes based on the condition that fails
- Models the context (realm, vsmmu, walk, entry_idx) as needed
- Returns `Ok(())` on successful validation