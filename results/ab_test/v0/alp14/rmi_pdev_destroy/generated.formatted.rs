pub open spec fn RMI_PDEV_DESTROY_spec(s: S, pdev_ptr: Address) -> bool {
    let pdev_pre = PdevAt(s, pdev_ptr);
    let result_da_supp = ImplFeatures(s).feat_da != FEATURE_TRUE ==> ResultEqual(
        result,
        RMI_ERROR_NOT_SUPPORTED,
    );
    let result_pdev_align = !AddrIsGranuleAligned(s, pdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let result_pdev_bound = !PaIsDelegable(s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let result_pdev_gran_state = GranuleAt(s, pdev_ptr).state != PDEV ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let result_pdev_state = pdev_pre.state != PDEV_STOPPED ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    );
    let result_pdev_p2p = pdev_pre.p2p_stream_valid == RMM_TRUE ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    );

    let success_gran_state = result.is_Ok() ==> GranuleAt(s, pdev_ptr).state == DELEGATED;
    let success_aux_state = result.is_Ok() ==> AuxStateEqual32(
        s,
        pdev_pre.aux,
        pdev_pre.num_aux,
        DELEGATED,
    );

    (ImplFeatures(s).feat_da != FEATURE_TRUE ==> result_da_supp) && (ImplFeatures(s).feat_da
        == FEATURE_TRUE ==> ((!AddrIsGranuleAligned(s, pdev_ptr) ==> result_pdev_align) && (
    AddrIsGranuleAligned(s, pdev_ptr) ==> ((!PaIsDelegable(s, pdev_ptr) ==> result_pdev_bound) && (
    PaIsDelegable(s, pdev_ptr) ==> ((GranuleAt(s, pdev_ptr).state != PDEV
        ==> result_pdev_gran_state) && (GranuleAt(s, pdev_ptr).state == PDEV ==> ((pdev_pre.state
        != PDEV_STOPPED ==> result_pdev_state) && (pdev_pre.state == PDEV_STOPPED ==> ((
    pdev_pre.p2p_stream_valid == RMM_TRUE ==> result_pdev_p2p) && (pdev_pre.p2p_stream_valid
        != RMM_TRUE ==> (success_gran_state && success_aux_state))))))))))))
}