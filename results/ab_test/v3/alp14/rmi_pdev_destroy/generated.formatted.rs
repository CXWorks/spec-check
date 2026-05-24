pub open spec fn rmi_pdev_destroy_spec(result: RmiCommandReturnCode, pdev_ptr: Address, old_s: S, new_s: S) -> bool {
    let pdev_pre = PdevAt(old_s, pdev_ptr);
    (
        (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (pdev_pre.state != PDEV_STOPPED ==> ResultEqual(result, RMI_ERROR_DEVICE))
        && (pdev_pre.p2p_stream_valid == RMM_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
        && (
            (ImplFeatures(old_s).feat_da == FEATURE_TRUE
            && AddrIsGranuleAligned(pdev_ptr)
            && PaIsDelegable(pdev_ptr)
            && GranuleAt(old_s, pdev_ptr).state == PDEV
            && pdev_pre.state == PDEV_STOPPED
            && pdev_pre.p2p_stream_valid != RMM_TRUE)
            ==> (result.is_Ok()
                && GranuleAt(new_s, pdev_ptr).state == DELEGATED
                && AuxStateEqual32(old_s, pdev_pre.aux, pdev_pre.num_aux, DELEGATED))
        )
    )
}