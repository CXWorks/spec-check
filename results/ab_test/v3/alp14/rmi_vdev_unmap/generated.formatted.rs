pub open spec fn rmi_vdev_unmap_spec(
    result: Result<(Address, Address), (RmiStatusCode, int)>,
    rd: Address,
    vdev_ptr: Address,
    ipa: Address,
    level: int,
    old_s: S,
    new_s: S,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let pa_top = ToAddress(UInt(walk.rtte.addr) + RttLevelSize(old_s, walk.level));
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);

    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(rd)
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
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsProtected(ipa, realm) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (walk.level < level ==> (ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1
        == walk.level && result.get_Err_0().1 > ipa)) && (walk.rtte.state != ASSIGNED_DEV ==> (
    ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1 == walk.level)) && ((
    !GranulesAllVdevUnvalidated(old_s, walk.rtte.addr, pa_top) && !GranulesAllVdevValidated(
        old_s,
        walk.rtte.addr,
        pa_top,
        vdev_pre,
    )) ==> (ResultEqual(result, RMI_ERROR_RTT) && result.get_Err_0().1 == walk.level)) && (
    AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(vdev_ptr) && PaIsDelegable(vdev_ptr) && GranuleAt(
        old_s,
        vdev_ptr,
    ).state == VDEV && vdev_pre.realm == rd && RttLevelIsValid(old_s, realm, level) && level >= 2
        && AddrIsRttLevelAligned(ipa, level) && AddrIsProtected(ipa, realm) && walk.level >= level
        && walk.rtte.state == ASSIGNED_DEV && (GranulesAllVdevUnvalidated(
        old_s,
        walk.rtte.addr,
        pa_top,
    ) || GranulesAllVdevValidated(old_s, walk.rtte.addr, pa_top, vdev_pre)) ==> (result.is_Ok()
        && result.get_Ok_0().0 == walk.rtte.addr && result.get_Ok_0().1 == walk_top
        && GranulesAllState(new_s, walk.rtte.addr, pa_top, DELEGATED) && GranulesAllVdevUnvalidated(
        new_s,
        walk.rtte.addr,
        pa_top,
    ) && VdevAt(new_s, vdev_ptr).num_map == vdev_pre.num_map - (RttLevelSize(old_s, level)
        << RMM_GRANULE_SIZE_ORDER) && RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        RttEntryIndex(old_s, ipa, walk.level),
    ).state == UNASSIGNED && (walk.rtte.ripas == DEV ==> RttEntryAt(
        new_s,
        RttAt(new_s, walk.rtt_addr),
        RttEntryIndex(old_s, ipa, walk.level),
    ).ripas == DESTROYED)))
}