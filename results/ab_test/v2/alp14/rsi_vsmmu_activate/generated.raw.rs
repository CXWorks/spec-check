```verus
pub open spec fn rsi_vsmmu_activate_spec(
    result: RsiCommandReturnCode,
    base: Address,
    top: Address,
    new_base: Address,
    old_s: S,
    new_s: S
) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let vsmmu = VsmmuAt(old_s, walk.rtte.addr);
    
    // Failure conditions
    let base_align_fail = !AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT;
    let top_align_fail = !AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT;
    let size_valid_fail = (UInt(top) <= UInt(base)) ==> result == RSI_ERROR_INPUT;
    let rgn_bound_fail = !AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT;
    let rtte_state_fail = walk.rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT;
    
    // Success conditions
    let ripas_success = (result == RSI_OK) ==> 
        RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtte.addr), RMM_RTT_PAGE_LEVEL, base, new_base, DEV);
    
    let start_success = (result == RSI_OK && base == vsmmu.reg_base && new_base != vsmmu.reg_top) ==>
        VsmmuAt(new_s, walk.rtte.addr).state == VSMMU_ACTIVATING;
    
    let complete_success = (result == RSI_OK && new_base == vsmmu.reg_top) ==>
        VsmmuAt(new_s, walk.rtte.addr).state == VSMMU_ACTIVE;
    
    base_align_fail && top_align_fail && size_valid_fail && rgn_bound_fail && rtte_state_fail &&
    ripas_success && start_success && complete_success
}
```