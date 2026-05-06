```rust
pub open spec fn RMI_RTT_READ_ENTRY_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
) -> (result: RmiCommandReturnCode, walk_level: u64, state: RmiRttEntryState, desc: u64, ripas: RmiRipas) {
    let realm = RealmAt(rd);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let rtte = RttDescriptorDecode(s, walk.rtte.desc, realm.rtt_s2ap_encoding);
    
    // Failure conditions
    if !AddrIsGranuleAligned(rd) {
        return (RMI_ERROR_INPUT, 0, RmiRttEntryState::UNASSIGNED, 0, RmiRipas::EMPTY);
    }
    if !PaIsDelegable(rd) {
        return (RMI_ERROR_INPUT, 0, RmiRttEntryState::UNASSIGNED, 0, RmiRipas::EMPTY);
    }
    if GranuleAt(s, rd).state != RmmGranuleState::RD {
        return (RMI_ERROR_INPUT, 0, RmiRttEntryState::UNASSIGNED, 0, RmiRipas::EMPTY);
    }
    if !RttLevelIsValid(s, realm, level) {
        return (RMI_ERROR_INPUT, 0, RmiRttEntryState::UNASSIGNED, 0, RmiRipas::EMPTY);
    }
    if !AddrIsRttLevelAligned(ipa, level) {
        return (RMI_ERROR_INPUT, 0, RmiRttEntryState::UNASSIGNED, 0, RmiRipas::EMPTY);
    }
    if UInt(ipa) >= (1 << realm.ipa_width) {
        return (RMI_ERROR_INPUT, 0, RmiRttEntryState::UNASSIGNED, 0, RmiRipas::EMPTY);
    }
    
    // Success conditions
    let output_state = RttEntryStateToRmi(s, walk.rtte.state);
    let output_desc = if walk.rtte.state == RmmRttEntryState::UNASSIGNED || walk.rtte.state == RmmRttEntryState::UNASSIGNED_NS {
        0
    } else if walk.rtte.state == RmmRttEntryState::ASSIGNED || walk.rtte.state == RmmRttEntryState::ASSIGNED_DEV || 
              walk.rtte.state == RmmRttEntryState::ASSIGNED_VSMMU || walk.rtte.state == RmmRttEntryState::TABLE {
        walk.rtte.addr
    } else if walk.rtte.state == RmmRttEntryState::ASSIGNED_NS {
        rtte.addr
    } else {
        0
    };
    
    let output_ripas = if walk.rtte.state == RmmRttEntryState::UNASSIGNED || walk.rtte.state == RmmRttEntryState::ASSIGNED {
        RipasToRmi(s, walk.rtte.ripas)
    } else {
        RmiRipas::EMPTY
    };
    
    (RMI_OK, walk.level as u64, output_state, output_desc, output_ripas)
}
```