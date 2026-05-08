pub open spec fn rmi_vsmmu_unmap_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    vsmmu_ptr: Address,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);

    // Failure conditions in priority order
    (!ImplFeatures().feat_da == FEATURE_TRUE ==> result == RMI_ERROR_NOT_SUPPORTED) && (
    !AddrIsGranuleAligned(rd) ==> result == RMI_ERROR_INPUT) && (!PaIsDelegable(rd) ==> result
        == RMI_ERROR_INPUT) && (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT) && (
    !AddrIsGranuleAligned(ipa) ==> result == RMI_ERROR_INPUT) && (!AddrIsProtected(
        old_s,
        ipa,
        realm,
    ) ==> result == RMI_ERROR_INPUT) && (!AddrIsGranuleAligned(vsmmu_ptr) ==> result
        == RMI_ERROR_INPUT) && (!PaIsDelegable(vsmmu_ptr) ==> result == RMI_ERROR_INPUT) && (
    GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> result == RMI_ERROR_INPUT) && (walk.level
        < RMM_RTT_PAGE_LEVEL ==> (result == RMI_ERROR_RTT(walk.level) && top == walk_top)) && (
    walk.rtte.state != ASSIGNED_VSMMU ==> (result == RMI_ERROR_RTT(walk.level) && top == walk_top))
        && (walk.rtte.addr != vsmmu_ptr ==> (result == RMI_ERROR_RTT(walk.level) && top
        == walk_top)) && (AddrIsAuxLive(old_s, ipa, realm) ==> result == RMI_ERROR_RTT_AUX(
        0,
    ))
    // Success conditions
     && (result == RMI_SUCCESS ==> (walk.rtte.state == UNASSIGNED && (walk.rtte.ripas == DEV
        ==> walk.rtte.ripas == DESTROYED) && top == walk_top && VsmmuAt(new_s, vsmmu_ptr).state
        == VSMMU_INACTIVE))
}