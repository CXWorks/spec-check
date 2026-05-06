pub open spec fn RMI_VDEV_UNMAP_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    ipa: Address,
    level: int,
    result: RmiCommandReturnCode,
    pa: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let pa_top = ToAddress(UInt(walk.rtte.addr) + RttLevelSize(old_s, walk.level));
    let rtte_state_pre = walk.rtte.state;
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

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
    let vdev_realm_fail = vdev_pre.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT);
    let level_bound_fail = (!RttLevelIsValid(old_s, realm, level) || level < 2) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let ipa_bound_fail = !AddrIsProtected(ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let rtt_walk_fail = walk.level < level ==> (ResultEqual(result, RMI_ERROR_RTT, walk.level)
        && top == walk_top);
    let rtte_state_fail = walk.rtte.state != ASSIGNED_DEV ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
        walk.level,
    ) && top == walk_top);
    let vdev_mapping_fail = (!GranulesAllVdevUnvalidated(old_s, walk.rtte.addr, pa_top)
        && !GranulesAllVdevValidated(old_s, walk.rtte.addr, pa_top, vdev_pre)) ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
        walk.level,
    ) && top == walk_top);

    // Success conditions
    let state_success = (result.is_Ok() ==> GranulesAllState(
        new_s,
        walk.rtte.addr,
        pa_top,
        DELEGATED,
    ));
    let unvalidated_success = (result.is_Ok() ==> GranulesAllVdevUnvalidated(
        new_s,
        walk.rtte.addr,
        pa_top,
    ));
    let num_map_success = (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).num_map == vdev_pre.num_map
        - (RttLevelSize(old_s, level) << RMM_GRANULE_SIZE_ORDER));
    let rtte_state_success = (result.is_Ok() ==> walk.rtte.state == UNASSIGNED);
    let ripas_dev_success = (walk.rtte.ripas == DEV && result.is_Ok() ==> walk.rtte.ripas
        == DESTROYED);
    let pa_success = (result.is_Ok() ==> pa == walk.rtte.addr);
    let top_success = (result.is_Ok() ==> top == walk_top);

    rd_align_fail && rd_bound_fail && rd_state_fail && vdev_align_fail && vdev_bound_fail
        && vdev_gran_state_fail && vdev_realm_fail && level_bound_fail && ipa_align_fail
        && ipa_bound_fail && rtt_walk_fail && rtte_state_fail && vdev_mapping_fail && state_success
        && unvalidated_success && num_map_success && rtte_state_success && ripas_dev_success
        && pa_success && top_success
}