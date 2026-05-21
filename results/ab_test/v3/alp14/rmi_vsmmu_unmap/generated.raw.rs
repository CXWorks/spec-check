pub open spec fn rmi_vsmmu_unmap_spec(result: RmiCommandReturnCode, top: Address, rd: Address, ipa: Address, vsmmu_ptr: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);
    
    (!ImplFeatures(old_s).feat_da_eq_true() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT) && (top == walk_top)))
    && (walk.rtte.state != ASSIGNED_VSMMU ==> (ResultEqual(result, RMI_ERROR_RTT) && (top == walk_top)))
    && (walk.rtte.addr != vsmmu_ptr ==> (ResultEqual(result, RMI_ERROR_RTT) && (top == walk_top)))
    && (AddrIsAuxLive(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    && ((ImplFeatures(old_s).feat_da_eq_true() && AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && AddrIsGranuleAligned(old_s, ipa) && AddrIsProtected(old_s, ipa, realm) && AddrIsGranuleAligned(old_s, vsmmu_ptr) && PaIsDelegable(old_s, vsmmu_ptr) && GranuleAt(old_s, vsmmu_ptr).state == VSMMU && walk.level >= RMM_RTT_PAGE_LEVEL && walk.rtte.state == ASSIGNED_VSMMU && walk.rtte.addr == vsmmu_ptr && !AddrIsAuxLive(old_s, ipa, realm)) ==> (result == RMI_SUCCESS && RttEntryAt(old_s, RttAt(old_s, walk.rtt_addr), entry_idx).state == UNASSIGNED && top == walk_top && vsmmu.state == VSMMU_INACTIVE && (walk.rtte.ripas == DEV ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED)))
}