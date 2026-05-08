pub open spec fn RMI_PDEV_DESTROY_spec(old_s: S, new_s: S, pdev_ptr: Address, result: RmiCommandReturnCode) -> bool {
    let pdev_pre = PdevAt(old_s, pdev_ptr);
    (
        (!ImplFeatures(old_s).feat_da.eq_FEATURE_TRUE() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED())) &&
        (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        (GranuleAt(old_s, pdev_ptr).state != PDEV() ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        (pdev_pre.state != PDEV_STOPPED() ==> ResultEqual(result, RMI_ERROR_DEVICE())) &&
        (pdev_pre.p2p_stream_valid == RMM_TRUE() ==> ResultEqual(result, RMI_ERROR_DEVICE())) &&
        (
            result.is_Ok() ==>
            ImplFeatures(old_s).feat_da.eq_FEATURE_TRUE() &&
            AddrIsGranuleAligned(pdev_ptr) &&
            PaIsDelegable(pdev_ptr) &&
            GranuleAt(old_s, pdev_ptr).state == PDEV() &&
            pdev_pre.state == PDEV_STOPPED() &&
            pdev_pre.p2p_stream_valid == RMM_FALSE() &&
            GranuleAt(new_s, pdev_ptr).state == DELEGATED() &&
            AuxStateEqual32(old_s, new_s, pdev_pre.aux, pdev_pre.num_aux, DELEGATED())
        )
    )
}