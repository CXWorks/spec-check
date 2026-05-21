pub open spec fn rmi_pdev_set_pubkey_spec(result: RmiCommandReturnCode, pdev_ptr: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    (!ImplFeatures().feat_da_is_FEATURE_TRUE() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RmiPublicKeyParamsAt(old_s, params_ptr).key_len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures().feat_da_is_FEATURE_TRUE()
        && AddrIsGranuleAligned(pdev_ptr)
        && PaIsDelegable(pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiPublicKeyParamsAt(old_s, params_ptr).key_len <= 1024
        && RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len <= 1024
        && PdevAt(old_s, pdev_ptr).state == PDEV_NEEDS_KEY)
        ==> (result.is_Ok()
            && PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY
            && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
}