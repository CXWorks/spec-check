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
    let vdev = VdevAt(new_s, vdev_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let pa_top = ToAddress(UInt(walk.rtte.addr) + RttLevelSize(old_s, walk.level));
    let rtte_state_pre = walk.rtte.state;
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    ((!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(vdev_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    vdev_pre.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!RttLevelIsValid(
        old_s,
        realm,
        level,
    ) || level < 2) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsRttLevelAligned(ipa, level)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsProtected(old_s, ipa, realm)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (walk.level < level ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && top == walk_top)) && (walk.rtte.state != ASSIGNED_DEV ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && top == walk_top)) && ((!GranulesAllVdevUnvalidated(old_s, walk.rtte.addr, pa_top)
        && !GranulesAllVdevValidated(old_s, walk.rtte.addr, pa_top, vdev_pre)) ==> (ResultEqual(
        result,
        RMI_ERROR_RTT,
    ) && top == walk_top)) && ((result == RMI_SUCCESS) ==> (GranulesAllState(
        new_s,
        walk.rtte.addr,
        pa_top,
        DELEGATED,
    ) && GranulesAllVdevUnvalidated(new_s, walk.rtte.addr, pa_top) && vdev.num_map
        == vdev_pre.num_map - (RttLevelSize(old_s, level) << RMM_GRANULE_SIZE_ORDER) && RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx as int,
    ).state == UNASSIGNED && ((walk.rtte.ripas == DEV) ==> (RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        entry_idx as int,
    ).ripas == DESTROYED)) && pa == walk.rtte.addr && top == walk_top)))
}