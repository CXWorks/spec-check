```verus
pub open spec fn RSI_VSMMU_ACTIVATE_spec(old_s: S, new_s: S, base: Address, top: Address, result: RsiCommandReturnCode, new_base: Address) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let vsmmu = VsmmuAt(old_s, walk.rtte.addr);
    
    (
        // Failure: base_align
        (!AddrIsGranuleAligned(old_s, base) ==> result == RSI_ERROR_INPUT) &&
        
        // Failure: top_align
        (!AddrIsGranuleAligned(old_s, top) ==> result == RSI_ERROR_INPUT) &&
        
        // Failure: size_valid
        (UInt(top) <= UInt(base) ==> result == RSI_ERROR_INPUT) &&
        
        // Failure: rgn_bound
        (!AddrRangeIsProtected(old_s, base, top, realm) ==> result == RSI_ERROR_INPUT) &&
        
        // Failure: rtte_state
        (walk.rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT) &&
        
        // Success conditions
        (result == RSI_OK ==>
            // ripas: address range [base, new_base) has RIPAS == DEV
            (RttEntriesInRangeRipas(new_s, walk.rtte, RMM_RTT_PAGE_LEVEL, base, new_base, DEV) &&
            
            // start: if base == vsmmu.reg_base and new_base != vsmmu.reg_top, then vsmmu.state == VSMMU_ACTIVATING
            ((base == vsmmu.reg_base && new_base != vsmmu.reg_top) ==> 
                VsmmuAt(new_s, walk.rtte.addr).state == VSMMU_ACTIVATING) &&
            
            // complete: if new_base == vsmmu.reg_top, then vsmmu.state == VSMMU_ACTIVE
            (new_base == vsmmu.reg_top ==>
                VsmmuAt(new_s, walk.rtte.addr).state == VSMMU_ACTIVE))
        )
    )
}
```