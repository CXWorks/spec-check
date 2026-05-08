```verus
pub open spec fn rmi_pdev_set_pubkey_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, pdev_ptr: Address, params_ptr: Address) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let params = RmiPublicKeyParamsAt(old_s, params_ptr);
    
    // Failure condition: da_supp
    (ImplFeatures().feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    
    // Failure conditions (ordering: da_supp < others)
    && (ImplFeatures().feat_da == FEATURE_TRUE ==>
        // Failure condition: pdev_align
        (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: pdev_bound
        && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: pdev_gran_state
        && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: params_align
        && (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: params_pas
        && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: key_len_oflow
        && (params.key_len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: metadata_len_oflow
        && (params.metadata_len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: key_invalid
        && (KeyIsInvalid(params) ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: metadata_invalid
        && (MetadataIsInvalid(params) ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: pdev_state (ordering: pdev_gran_state < pdev_state)
        && (GranuleAt(old_s, pdev_ptr).state == PDEV ==>
            (pdev.state != PDEV_NEEDS_KEY ==> ResultEqual(result, RMI_ERROR_DEVICE))
        )
        
        // Success conditions
        && (GranuleAt(old_s, pdev_ptr).state == PDEV && pdev.state == PDEV_NEEDS_KEY &&
            params.key_len <= 1024 && params.metadata_len <= 1024 &&
            !KeyIsInvalid(params) && !MetadataIsInvalid(params) ==>
            (result == RMI_SUCCESS &&
             PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY &&
             PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
        )
    )
}
```