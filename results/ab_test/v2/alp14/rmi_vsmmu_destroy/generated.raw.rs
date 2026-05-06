```verus
pub open spec fn rmi_vsmmu_destroy_spec(result: RmiCommandReturnCode, rd: Address, vsmmu_ptr: Address, old_s: S, new_s: S) -> bool {
    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> 
        ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    
    // Failure condition: rd_align
    && (!AddrIsGranuleAligned(old_s, rd) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: rd_bound
    && (!PaIsDelegable(old_s, rd) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: rd_state
    && (GranuleAt(old_s, rd).state != RD ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: vsmmu_align
    && (!AddrIsGranuleAligned(old_s, vsmmu_ptr) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: vsmmu_bound
    && (!PaIsDelegable(old_s, vsmmu_ptr) ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: vsmmu_state
    && (GranuleAt(old_s, vsmmu_ptr).state != VSMMU ==> 
        ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: vsmmu_live
    && (VsmmuIsLive(old_s, vsmmu_ptr) ==> 
        ResultEqual(result, RMI_ERROR_DEVICE))
    
    // Success conditions
    && (result.is_Ok() ==> 
        // gran_state: VSMMU granule transitions to DELEGATED
        GranuleAt(new_s, vsmmu_ptr).state == DELEGATED
        // num_vsmmus: realm vsmmu count decremented by 1
        && RealmAt(new_s, rd).num_vsmmus == RealmAt(old_s, rd).num_vsmmus - 1)
}
```