```verus
pub open spec fn RMI_RTT_AUX_DESTROY_spec(
    s: S,
    rd: Address,
    ipa: Address,
    level: int,
    index: u64,
) -> (result: Result<(Address, Address), RmiStatusCode>) {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, level - 1, index);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    
    if !AddrIsGranuleAligned(rd) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !PaIsDelegable(rd) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if GranuleAt(s, rd).state != RmmGranuleState::RD {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !RttLevelIsValid(s, realm, level) || RttLevelIsStarting(s, realm, level) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !AddrIsRttLevelAligned(ipa, level - 1) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if !AddrIsProtected(s, ipa, realm) {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if realm.rtt_tree_per_plane == RmmFeatureSupport::FEATURE_FALSE 
              || index == RMM_RTT_TREE_PRIMARY 
              || index > realm.num_aux_planes {
        Err(RmiStatusCode::RMI_ERROR_INPUT)
    } else if walk.level < level - 1 {
        Err(RmiStatusCode::RMI_ERROR_RTT_AUX)
    } else if RttEntryAt(s, RttAt(s, walk.rtt_addr), entry_idx).state != RmmRttEntryState::TABLE {
        Err(RmiStatusCode::RMI_ERROR_RTT_AUX)
    } else if RttIsLive(s, RttAt(s, RttEntryAt(s, RttAt(s, walk.rtt_addr), entry_idx).addr)) {
        Err(RmiStatusCode::RMI_ERROR_RTT_AUX)
    } else {
        Ok((RttEntryAt(s, RttAt(s, walk.rtt_addr), entry_idx).addr, walk_top))
    }
}
```

This function:
1. Extracts the realm from the RD address
2. Performs an RTT walk to find the auxiliary RTT entry
3. Checks all input validation conditions in the specified order
4. Validates the RTT walk succeeded and the entry is in the correct state
5. Ensures the RTT is not live (i.e., can be destroyed)
6. Returns the address of the destroyed RTT and the top IPA of non-live entries on success, or an error code on failure