```rust
pub open spec fn RMI_VSMMU_UNMAP_spec(s: S, rd: Address, ipa: Address, vsmmu_ptr: Address) -> (result: Result<Address, RmiStatusCode>, top: Address)
{
    let realm = RealmAt(rd);
    let walk = RttWalk(s, realm, ipa, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(s, ipa, walk.level);
    let walk_top = RttSkipNonLiveEntries(s, RttAt(s, walk.rtt_addr), walk.level, ipa);
    let vsmmu = VsmmuAt(s, vsmmu_ptr);
    
    // Failure conditions (in order of precedence)
    if !ImplFeatures().feat_da {
        (Err(RMI_ERROR_NOT_SUPPORTED), 0)
    } else if !AddrIsGranuleAligned(rd) {
        (Err(RMI_ERROR_INPUT), 0)
    } else if !PaIsDelegable(rd) {
        (Err(RMI_ERROR_INPUT), 0)
    } else if GranuleAt(rd).state != RD {
        (Err(RMI_ERROR_INPUT), 0)
    } else if !AddrIsGranuleAligned(ipa) {
        (Err(RMI_ERROR_INPUT), 0)
    } else if !AddrIsProtected(ipa, realm) {
        (Err(RMI_ERROR_INPUT), 0)
    } else if !AddrIsGranuleAligned(vsmmu_ptr) {
        (Err(RMI_ERROR_INPUT), 0)
    } else if !PaIsDelegable(vsmmu_ptr) {
        (Err(RMI_ERROR_INPUT), 0)
    } else if GranuleAt(vsmmu_ptr).state != VSMMU {
        (Err(RMI_ERROR_INPUT), 0)
    } else if walk.level < RMM_RTT_PAGE_LEVEL {
        (Err(RMI_ERROR_RTT(walk.level)), walk_top)
    } else if walk.rtte.state != ASSIGNED_VSMMU {
        (Err(RMI_ERROR_RTT(walk.level)), walk_top)
    } else if walk.rtte.addr != vsmmu_ptr {
        (Err(RMI_ERROR_RTT(walk.level)), walk_top)
    } else if AddrIsAuxLive(ipa, realm) {
        (Err(RMI_ERROR_RTT_AUX(0)), walk_top)
    } else {
        // Success conditions
        let updated_rtte = RmmRttEntry { state: UNASSIGNED, ripas: if walk.rtte.ripas == DEV then DESTROYED else walk.rtte.ripas, ..walk.rtte };
        let updated_vsmmu = RmmVsmmu { state: VSMMU_INACTIVE, ..vsmmu };
        (Ok(walk_top), walk_top)
    }
}
```