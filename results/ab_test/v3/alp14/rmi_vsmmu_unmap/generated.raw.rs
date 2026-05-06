```verus
pub open spec fn RMI_VSMMU_UNMAP_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    ipa: Address,
    vsmmu_ptr: Address,
    result: Result<(), RmiStatusCode>,
    top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(old_s, RttAt(old_s, walk.rtt_addr), walk.level, ipa);
    let vsmmu = VsmmuAt(old_s, vsmmu_ptr);

    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    // Failure condition: rd_align
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: rd_bound
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: rd_state
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: ipa_align
    && (!AddrIsGranuleAligned(old_s, ipa) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: ipa_bound
    && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: vsmmu_align
    && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: vsmmu_bound
    && (!PaIsDelegable(old_s, vsmmu_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: vsmmu_state
    && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: rtt_walk
    && (walk.level < RMM_RTT_PAGE_LEVEL ==> (ResultEqual(result, RMI_ERROR_RTT) && (top == walk_top)))
    // Failure condition: rtte_state
    && (walk.rtte.state != ASSIGNED_VSMMU ==> (ResultEqual(result, RMI_ERROR_RTT) && (top == walk_top)))
    // Failure condition: rtte_addr
    && (walk.rtte.addr != vsmmu_ptr ==> (ResultEqual(result, RMI_ERROR_RTT) && (top == walk_top)))
    // Failure condition: aux_live
    && (AddrIsAuxLive(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_RTT_AUX))
    // Success condition: rtte_state
    && (result.is_Ok() ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == UNASSIGNED)
    // Success condition: ripas_ram
    && (result.is_Ok() && walk.rtte.ripas == DEV ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).ripas == DESTROYED)
    // Success condition: top
    && (result.is_Ok() ==> top == walk_top)
    // Success condition: state
    && (result.is_Ok() ==> VsmmuAt(new_s, vsmmu_ptr).state == VSMMU_INACTIVE)
}
```