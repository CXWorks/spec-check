pub open spec fn rmi_vdev_unmap_spec(
    result: RmiCommandReturnCode,
    pa: Address,
    top: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    ipa: Address,
    level: int,
) -> bool {
    let realm = RealmAt(rd);
    let vdev_pre = VdevAt(vdev_ptr);
    let vdev = VdevAt(vdev_ptr);
    let walk = RttWalk(realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let pa_top = ToAddress(UInt(walk.rtte.addr) + RttLevelSize(level));
    let rtte_state_pre = walk.rtte.state;
    let entry_idx = RttEntryIndex(ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(RttAt(walk.rtt_addr), walk.level, ipa);

    // Failure conditions
    let rd_align_fail = !AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_bound_fail = !PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_align_fail = !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_bound_fail = !PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_gran_state_fail = GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let vdev_realm_fail = vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT);
    let level_bound_fail = (!RttLevelIsValid(realm, level) || level < 2) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let ipa_bound_fail = !AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let rtt_walk_fail = walk.level < level ==> (ResultEqual(result, RMI_ERROR_RTT, walk.level) && (
    top == walk_top));
    let rtte_state_fail = walk.rtte.state != ASSIGNED_DEV ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
        walk.level,
    ) && (top == walk_top));
    let vdev_mapping_fail = (!GranulesAllVdevUnvalidated(old_s, walk.rtte.addr, pa_top)
        && !GranulesAllVdevValidated(old_s, walk.rtte.addr, pa_top, vdev)) ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
        walk.level,
    ) && (top == walk_top));

    // Success conditions
    let state_success = (walk.rtte.state == ASSIGNED_DEV) ==> GranulesAllState(
        new_s,
        walk.rtte.addr,
        pa_top,
        DELEGATED,
    );
    let unvalidated_success = (walk.rtte.state == ASSIGNED_DEV) ==> GranulesAllVdevUnvalidated(
        new_s,
        walk.rtte.addr,
        pa_top,
    );
    let num_map_success = (walk.rtte.state == ASSIGNED_DEV) ==> vdev.num_map == vdev_pre.num_map - (
    RttLevelSize(level) << RMM_GRANULE_SIZE_ORDER);
    let rtte_state_success = (walk.rtte.state == ASSIGNED_DEV) ==> RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx,
    ).state == UNASSIGNED;
    let ripas_dev_success = (walk.rtte.state == ASSIGNED_DEV && walk.rtte.ripas == DEV)
        ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED;
    let pa_success = (walk.rtte.state == ASSIGNED_DEV) ==> pa == walk.rtte.addr;
    let top_success = (walk.rtte.state == ASSIGNED_DEV) ==> top == walk_top;

    rd_align_fail && rd_bound_fail && rd_state_fail && vdev_align_fail && vdev_bound_fail
        && vdev_gran_state_fail && vdev_realm_fail && level_bound_fail && ipa_align_fail
        && ipa_bound_fail && rtt_walk_fail && rtte_state_fail && vdev_mapping_fail && state_success
        && unvalidated_success && num_map_success && rtte_state_success && ripas_dev_success
        && pa_success && top_success
}