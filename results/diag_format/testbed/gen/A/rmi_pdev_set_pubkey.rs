pub open spec fn rmi_pdev_set_pubkey_spec(pdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    ((ImplFeatures(old_s).feat_da == FEATURE_FALSE)
        ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && (!AddrIsGranuleAligned(old_s, pdev_ptr)
            || !PaIsDelegable(old_s, pdev_ptr)
            || GranuleAt(old_s, pdev_ptr).state != PDEV))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && (!AddrIsGranuleAligned(old_s, params_ptr)
            || !GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
            || RmiPublicKeyParamsAt(old_s, params_ptr).key_len > 1024
            || RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len > 1024)
        && PdevAt(old_s, pdev_ptr).state == PDEV_NEEDS_KEY)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && (!AddrIsGranuleAligned(old_s, params_ptr)
            || !GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
            || RmiPublicKeyParamsAt(old_s, params_ptr).key_len > 1024
            || RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len > 1024)
        && PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY)
        ==> (ResultEqual(result, RMI_ERROR_INPUT) || ResultEqual(result, RMI_ERROR_DEVICE)))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiPublicKeyParamsAt(old_s, params_ptr).key_len <= 1024
        && RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len <= 1024
        && PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY)
        ==> (ResultEqual(result, RMI_ERROR_DEVICE) || ResultEqual(result, RMI_ERROR_INPUT)))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiPublicKeyParamsAt(old_s, params_ptr).key_len <= 1024
        && RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len <= 1024
        && PdevAt(old_s, pdev_ptr).state == PDEV_NEEDS_KEY)
        ==> (result.is_Ok() || ResultEqual(result, RMI_ERROR_INPUT)))
    && (result.is_Ok() ==> (
        ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(old_s, params_ptr)
        && GranuleAccessPermitted(old_s, params_ptr, PAS_NS)
        && RmiPublicKeyParamsAt(old_s, params_ptr).key_len <= 1024
        && RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len <= 1024
        && PdevAt(old_s, pdev_ptr).state == PDEV_NEEDS_KEY
        && PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY
        && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
    && ((result.is_Err()
        && ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV)
        ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state
            && PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state))
}
