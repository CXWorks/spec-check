pub open spec fn RMI_VSMMU_DESTROY_spec(s: S, rd: Address, vsmmu_ptr: Address) -> (result: RmiCommandReturnCode, s_post: S)
    ensures
        // Failure condition: da_supp
        (!ImplFeatures().feat_da != FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED),
        
        // Failure condition: rd_align
        (!AddrIsGranuleAligned(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT),
        
        // Failure condition: rd_bound
        (!PaIsDelegable(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT),
        
        // Failure condition: rd_state
        (GranuleAt(s, rd).state != RD) ==> ResultEqual(result, RMI_ERROR_INPUT),
        
        // Failure condition: vsmmu_align
        (!AddrIsGranuleAligned(vsmmu_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT),
        
        // Failure condition: vsmmu_bound
        (!PaIsDelegable(vsmmu_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT),
        
        // Failure condition: vsmmu_state
        (GranuleAt(s, vsmmu_ptr).state != VSMMU) ==> ResultEqual(result, RMI_ERROR_INPUT),
        
        // Failure condition: vsmmu_live
        (VsmmuIsLive(s, vsmmu_ptr)) ==> ResultEqual(result, RMI_ERROR_DEVICE),
        
        // Success condition: gran_state
        (result.is_Ok()) ==> (GranuleAt(s_post, vsmmu_ptr).state == DELEGATED),
        
        // Success condition: num_vsmmus
        (result.is_Ok()) ==> (RealmAt(s_post, rd).num_vsmmus == RealmAt(s, rd).num_vsmmus - 1),
;