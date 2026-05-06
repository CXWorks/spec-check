```rust
pub open spec fn RMI_RTT_AUX_MAP_PROTECTED_spec(
    s: S,
    rd: Address,
    ipa: Address,
    index: u64,
) -> (result: Result<(RmiRttEntryState, RmiRipas), RmiCommandReturnCode>, s_post: S)
{
    let realm = RealmAt(s, rd);
    let walk_pri = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let walk_aux = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, index);
    let entry_idx = RttEntryIndex(s, ipa, walk_aux.level);

    // Failure conditions (in priority order)
    if !AddrIsGranuleAligned(rd) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(rd) {
        (Err(RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, rd).state != RD {
        (Err(RMI_ERROR_INPUT), s)
    } else if !AddrIsGranuleAligned(ipa) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !AddrIsProtected(s, ipa, realm) {
        (Err(RMI_ERROR_INPUT), s)
    } else if realm.rtt_tree_per_plane == FEATURE_FALSE
        || index == RMM_RTT_TREE_PRIMARY
        || index > realm.num_aux_planes {
        (Err(RMI_ERROR_INPUT), s)
    } else if walk_pri.rtte.state != ASSIGNED
        && walk_pri.rtte.state != ASSIGNED_DEV
        && walk_pri.rtte.state != ASSIGNED_VSMMU {
        (Err(RMI_ERROR_RTT), s)
    } else if walk_pri.rtte.state == ASSIGNED && walk_pri.rtte.ripas != RAM {
        (Err(RMI_ERROR_RTT), s)
    } else if walk_pri.rtte.state == ASSIGNED_DEV && walk_pri.rtte.ripas != DEV {
        (Err(RMI_ERROR_RTT), s)
    } else if walk_aux.rtte.state == AUX_DESTROYED {
        (Err(RMI_ERROR_RTT_AUX), s)
    } else if walk_aux.level < walk_pri.level {
        (Err(RMI_ERROR_RTT_AUX), s)
    } else {
        // Success conditions
        let s_post = s;
        let expected_addr = walk_pri.rtte.addr + (entry_idx as int * RttLevelSize(s, walk_aux.level));
        
        (Ok((
            RttEntryStateToRmi(s, ASSIGNED),
            RipasToRmi(s, walk_pri.rtte.ripas),
        )), s_post)
    }
}
```