pub open spec fn rmi_vsmmu_unmap_spec(
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    vsmmu_ptr: Address,
    top: Address
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);
    
    (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT) && top == walk_top))
    && (walk.rtte.state != ASSIGNED_VSMMU ==> (ResultEqual(result, RMI_ERROR_RTT) && top == walk_top))
    && (walk.rtte.addr != vsmmu_ptr ==> (ResultEqual(result, RMI_ERROR_RTT) && top == walk_top))
    && (AddrIsAuxLive(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
         && AddrIsGranuleAligned(rd)
         && PaIsDelegable(rd)
         && GranuleAt(old_s, rd).state == RD
         && AddrIsGranuleAligned(ipa)
         && AddrIsProtected(old_s, ipa, realm)
         && AddrIsGranuleAligned(vsmmu_ptr)
         && PaIsDelegable(vsmmu_ptr)
         && GranuleAt(old_s, vsmmu_ptr).state == VSMMU
         && walk.level >= RMM_RTT_PAGE_LEVEL
         && walk.rtte.state == ASSIGNED_VSMMU
         && walk.rtte.addr == vsmmu_ptr
         && !AddrIsAuxLive(old_s, ipa, realm))
        ==> (result.is_Ok()
             && walk.rtte.state == UNASSIGNED
             && (walk.rtte.ripas == DEV ==> walk.rtte.ripas == DESTROYED)
             && top == walk_top
             && VsmmuAt(new_s, vsmmu_ptr).state == VSMMU_INACTIVE))
}