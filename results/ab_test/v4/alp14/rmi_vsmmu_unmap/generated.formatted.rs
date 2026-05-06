pub open spec fn RMI_VSMMU_UNMAP_spec(
    old_s: S,
    new_s: S,
    fid: u64,
    rd: Address,
    ipa: Address,
    vsmmu_ptr: Address,
    result: RmiCommandReturnCode,
    top: Address,
) -> bool {
    let realm = RealmAt(rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);

    ((!ImplFeatures().feat_da.is_FEATURE_TRUE() ==> result.is_RmiCommandReturnCode_Error()
        && result.get_RmiCommandReturnCode_Error_0() == RMI_ERROR_NOT_SUPPORTED()) && (
    !AddrIsGranuleAligned(rd) ==> result.is_RmiCommandReturnCode_Error()
        && result.get_RmiCommandReturnCode_Error_0() == RMI_ERROR_INPUT()) && (!PaIsDelegable(rd)
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_INPUT()) && (GranuleAt(old_s, rd).state != RD()
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_INPUT()) && (!AddrIsGranuleAligned(ipa)
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_INPUT()) && (!AddrIsProtected(old_s, ipa, realm)
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_INPUT()) && (!AddrIsGranuleAligned(vsmmu_ptr)
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_INPUT()) && (!PaIsDelegable(vsmmu_ptr)
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_INPUT()) && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU()
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_INPUT()) && (walk.level < RMM_RTT_PAGE_LEVEL as int ==> (
    result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_RTT() && top == walk_top)) && (walk.rtte.state != ASSIGNED_VSMMU() ==> (
    result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_RTT() && top == walk_top)) && (walk.rtte.addr != vsmmu_ptr ==> (
    result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_RTT() && top == walk_top)) && (AddrIsAuxLive(old_s, ipa, realm)
        ==> result.is_RmiCommandReturnCode_Error() && result.get_RmiCommandReturnCode_Error_0()
        == RMI_ERROR_RTT_AUX()) && (result.is_RmiCommandReturnCode_Ok() ==> (walk.rtte.state
        == UNASSIGNED() && (walk.rtte.ripas == DEV() ==> walk.rtte.ripas == DESTROYED()) && top
        == walk_top && vsmmu.state == VSMMU_INACTIVE())))
}