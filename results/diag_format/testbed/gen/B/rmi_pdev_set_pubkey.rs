pub open spec fn rmi_pdev_set_pubkey_spec(pdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!ImplFeatures(old_s).feat_da != RmmFeature::FEATURE_TRUE ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != RmmGranuleState::PDEV ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, RmmPhysicalAddressSpace::PAS_NS) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (RmiPublicKeyParamsAt(old_s, params_ptr).key_len > 1024 ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len > 1024 ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PublicKeyIsValid(RmiPublicKeyParamsAt(old_s, params_ptr).key, RmiPublicKeyParamsAt(old_s, params_ptr).key_len, RmiPublicKeyParamsAt(old_s, params_ptr).algo) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (!PublicKeyMetadataIsValid(RmiPublicKeyParamsAt(old_s, params_ptr).metadata, RmiPublicKeyParamsAt(old_s, params_ptr).metadata_len, RmiPublicKeyParamsAt(old_s, params_ptr).algo) ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT))
    && (PdevAt(old_s, pdev_ptr).state != RmmPdevState::PDEV_NEEDS_KEY ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_DEVICE))
    && (ResultEqual(result, RmiStatusCode::RMI_SUCCESS) ==> (PdevAt(new_s, pdev_ptr).state == RmmPdevState::PDEV_HAS_KEY && PdevAt(new_s, pdev_ptr).comm_state == RmmDevCommState::DEV_COMM_PENDING))
}