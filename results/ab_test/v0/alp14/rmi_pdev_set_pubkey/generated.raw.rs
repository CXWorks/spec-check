```verus
pub open spec fn RMI_PDEV_SET_PUBKEY_spec(s: S, pdev_ptr: Address, params_ptr: Address, result: RmiCommandReturnCode) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let params = RmiPublicKeyParamsAt(s, params_ptr);
    
    (
        // Failure condition: da_supp
        (!ImplFeatures(s).feat_da || ImplFeatures(s).feat_da != FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)
    ) && (
        // Failure condition: pdev_align
        (!AddrIsGranuleAligned(s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure condition: pdev_bound
        (!PaIsDelegable(s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure condition: pdev_gran_state
        (GranuleAt(s, pdev_ptr).state != PDEV) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure condition: params_align
        (!AddrIsGranuleAligned(s, params_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure condition: params_pas
        (!GranuleAccessPermitted(s, params_ptr, PAS_NS)) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure condition: key_len_oflow
        (params.key_len > 1024) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Failure condition: metadata_len_oflow
        (params.metadata_len > 1024) ==> ResultEqual(result, RMI_ERROR_INPUT)
    ) && (
        // Success conditions: when no failure conditions apply
        (ImplFeatures(s).feat_da == FEATURE_TRUE &&
         AddrIsGranuleAligned(s, pdev_ptr) &&
         PaIsDelegable(s, pdev_ptr) &&
         GranuleAt(s, pdev_ptr).state == PDEV &&
         AddrIsGranuleAligned(s, params_ptr) &&
         GranuleAccessPermitted(s, params_ptr, PAS_NS) &&
         params.key_len <= 1024 &&
         params.metadata_len <= 1024 &&
         pdev.state == PDEV_NEEDS_KEY
        ) ==> (
            result.is_Ok() &&
            pdev.state == PDEV_HAS_KEY &&
            pdev.comm_state == DEV_COMM_PENDING
        )
    )
}
```