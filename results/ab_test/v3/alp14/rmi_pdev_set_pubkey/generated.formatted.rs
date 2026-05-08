pub open spec fn RMI_PDEV_SET_PUBKEY_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    params_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let params = RmiPublicKeyParamsAt(old_s, params_ptr);

    // Failure conditions in priority order
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ((!AddrIsGranuleAligned(pdev_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (params.key_len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT)) && (params.metadata_len
        > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!KeyIsValid(old_s, params)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!MetadataIsValid(old_s, params)
        ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Success path when all conditions pass
    (AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && GranuleAt(old_s, pdev_ptr).state
        == PDEV && AddrIsGranuleAligned(params_ptr) && GranuleAccessPermitted(
        old_s,
        params_ptr,
        PAS_NS,
    ) && params.key_len <= 1024 && params.metadata_len <= 1024 && KeyIsValid(old_s, params)
        && MetadataIsValid(old_s, params) ==> ((pdev.state == PDEV_NEEDS_KEY ==> (result.is_Ok()
        && PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY && PdevAt(new_s, pdev_ptr).comm_state
        == DEV_COMM_PENDING)) && (pdev.state != PDEV_NEEDS_KEY ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    ))))))
}