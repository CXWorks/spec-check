pub open spec fn rmi_pdev_set_pubkey_spec(result: RmiCommandReturnCode, pdev_ptr: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let params = RmiPublicKeyParamsAt(old_s, params_ptr);
    
    (!ImplFeatures(old_s).feat_da_is_true ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != GRANULE_STATE_PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (params.key_len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (params.metadata_len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev.state != PDEV_STATE_NEEDS_KEY ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (
      (ImplFeatures(old_s).feat_da_is_true
       && AddrIsGranuleAligned(old_s, pdev_ptr)
       && PaIsDelegable(old_s, pdev_ptr)
       && GranuleAt(old_s, pdev_ptr).state == GRANULE_STATE_PDEV
       && AddrIsGranuleAligned(old_s, params_ptr)
       && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
       && params.key_len <= 1024
       && params.metadata_len <= 1024
       && pdev.state == PDEV_STATE_NEEDS_KEY)
      ==> (result.is_Ok()
           && PdevAt(new_s, pdev_ptr).state == PDEV_STATE_HAS_KEY
           && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_STATE_PENDING)
    )
}