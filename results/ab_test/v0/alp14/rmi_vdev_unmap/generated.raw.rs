```verus
pub open spec fn RMI_VDEV_UNMAP_spec(
    s: S,
    rd: Address,
    vdev_ptr: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    pa: Address,
    top: Address
) -> bool {
    let realm = RealmAt(s, rd);
    let vdev_pre = VdevAt(s, vdev_ptr);
    let walk = RttWalk(s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let pa_top = ToAddress(UInt(walk.rtte.addr) + RttLevelSize(s, walk.level));
    let rtte_state_pre = walk.rtte.state;
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    
    // Failure conditions - in order of precedence
    let rd_align_fail = !AddrIsGranuleAligned(s, rd);
    let rd_bound_fail = !PaIsDelegable(s, rd);
    let rd_state_fail = GranuleAt(s, rd).state != RD;
    let vdev_align_fail = !AddrIsGranuleAligned(s, vdev_ptr);
    let vdev_bound_fail = !PaIsDelegable(s, vdev_ptr);
    let vdev_gran_state_fail = GranuleAt(s, vdev_ptr).state != VDEV;
    let vdev_realm_fail = vdev_pre.realm != rd;
    let level_bound_fail = !RttLevelIsValid(s, realm, level) || level < 2;
    let ipa_align_fail = !AddrIsRttLevelAligned(s, ipa, level);
    let ipa_bound_fail = !AddrIsProtected(s, ipa, realm);
    let rtt_walk_fail = walk.level < level;
    let rtte_state_fail = walk.rtte.state != ASSIGNED_DEV;
    let vdev_mapping_fail = !GranulesAllVdevUnvalidated(s, walk.rtte.addr, pa_top) && 
                            !GranulesAllVdevValidated(s, walk.rtte.addr, pa_top, vdev_pre);

    // Check input alignment and bounds first
    if rd_align_fail || vdev_align_fail || ipa_align_fail {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if rd_bound_fail || vdev_bound_fail || ipa_bound_fail {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if rd_state_fail || vdev_gran_state_fail {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if vdev_realm_fail || level_bound_fail {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if rtt_walk_fail || rtte_state_fail || vdev_mapping_fail {
        result.is_Err() && result.get_Err_0().0 == RMI_ERROR_RTT && 
        result.get_Err_0().1 == walk.level &&
        top == walk_top
    } else {
        // Success path
        result.is_Ok() &&
        GranulesAllState(s, walk.rtte.addr, pa_top, DELEGATED) &&
        GranulesAllVdevUnvalidated(s, walk.rtte.addr, pa_top) &&
        (exists vdev_post: RmmVdev :: vdev_post == VdevAt(s, vdev_ptr) &&
         vdev_post.num_map == vdev_pre.num_map - (RttLevelSize(s, level) << RMM_GRANULE_SIZE_ORDER)) &&
        walk.rtte.state == UNASSIGNED &&
        (walk.rtte.ripas == DEV ==> walk.rtte.ripas == DESTROYED) &&
        pa == walk.rtte.addr &&
        top == walk_top
    }
}
```