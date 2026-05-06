```rust
pub open spec fn RMI_RTT_AUX_UNMAP_UNPROTECTED_spec(s: S, rd: Address, ipa: Address, index: u64) -> (result: Result<Address, RmiStatusCode>) {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, realm.rtt_level_start, index);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions
    if !AddrIsGranuleAligned(rd) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(rd) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, rd).state != RD {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsRttLevelAligned(ipa, realm.rtt_level_start) {
        Err(RMI_ERROR_INPUT)
    } else if UInt(ipa) >= (1u64 << realm.ipa_width) || AddrIsProtected(s, ipa, realm) {
        Err(RMI_ERROR_INPUT)
    } else if realm.rtt_tree_per_plane == FEATURE_FALSE || index == RMM_RTT_TREE_PRIMARY || index > realm.num_aux_planes {
        Err(RMI_ERROR_INPUT)
    }
    // Success condition: walk.rtte.state == UNASSIGNED_NS and return walk_top
    else if walk.rtte.state == UNASSIGNED_NS {
        Ok(walk_top)
    } else {
        Err(RMI_ERROR_RTT)
    }
}
```