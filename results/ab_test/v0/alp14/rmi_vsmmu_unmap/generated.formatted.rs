pub open spec fn RMI_VSMMU_UNMAP_spec(s: S, rd: Address, ipa: Address, vsmmu_ptr: Address) -> bool {
    let realm = RealmAt(s, rd);
    let walk = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    let vsmmu = VsmmuAt(s, vsmmu_ptr);

    // Failure conditions (in priority order)
    let da_supp_fail = !ImplFeatures(s).feat_da;
    let rd_align_fail = !AddrIsGranuleAligned(s, rd);
    let rd_bound_fail = !PaIsDelegable(s, rd);
    let rd_state_fail = GranuleAt(s, rd).state != RD;
    let ipa_align_fail = !AddrIsGranuleAligned(s, ipa);
    let ipa_bound_fail = !AddrIsProtected(s, ipa, realm);
    let vsmmu_align_fail = !AddrIsGranuleAligned(s, vsmmu_ptr);
    let vsmmu_bound_fail = !PaIsDelegable(s, vsmmu_ptr);
    let vsmmu_state_fail = GranuleAt(s, vsmmu_ptr).state != VSMMU;
    let rtt_walk_fail = walk.level < RMM_RTT_PAGE_LEVEL;
    let rtte_state_fail = walk.rtte.state != ASSIGNED_VSMMU;
    let rtte_addr_fail = walk.rtte.addr != vsmmu_ptr;
    let aux_live_fail = AddrIsAuxLive(s, ipa, realm);

    // Success conditions
    let rtte_state_success = walk.rtte.state == UNASSIGNED;
    let ripas_ram_success = walk.rtte.ripas == DEV ==> walk.rtte.ripas == DESTROYED;
    let top_success = walk_top == walk_top;
    let state_success = vsmmu.state == VSMMU_INACTIVE;

    if da_supp_fail {
        false
    } else if rd_align_fail || rd_bound_fail || rd_state_fail {
        false
    } else if ipa_align_fail {
        false
    } else if vsmmu_align_fail || vsmmu_bound_fail || vsmmu_state_fail {
        false
    } else if rtt_walk_fail || rtte_state_fail || rtte_addr_fail {
        false
    } else if aux_live_fail {
        false
    } else {
        rtte_state_success && ripas_ram_success && top_success && state_success
    }
}