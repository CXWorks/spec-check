pub open spec fn RMI_PDEV_P2P_CONNECT_spec(
    s: S,
    stream_ptr: Address,
    pdev_1_ptr: Address,
    pdev_2_ptr: Address,
    ide_sid: u64,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let stream = P2PStreamAt(s, stream_ptr);
    let pdev_1 = PdevAt(s, pdev_1_ptr);
    let pdev_2 = PdevAt(s, pdev_2_ptr);

    // Check all failure conditions and return early if any are met
    if !ImplFeatures(s).feat_da {
        return ResultEqual(result, RMI_ERROR_NOT_SUPPORTED);
    }
    if !AddrIsGranuleAligned(stream_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !PaIsDelegableDram(stream_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if GranuleAt(s, stream_ptr).state != DELEGATED {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !AddrIsGranuleAligned(pdev_1_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !PaIsDelegable(pdev_1_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if GranuleAt(s, pdev_1_ptr).state != PDEV {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if pdev_1.state != PDEV_READY {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if pdev_1.p2p_enabled != FEATURE_TRUE {
        return ResultEqual(result, RMI_ERROR_DEVICE);
    }
    if pdev_1.p2p_stream_valid == RMM_TRUE {
        return ResultEqual(result, RMI_ERROR_DEVICE);
    }
    if !AddrIsGranuleAligned(pdev_2_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if !PaIsDelegable(pdev_2_ptr) {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if GranuleAt(s, pdev_2_ptr).state != PDEV {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if pdev_2.state != PDEV_READY {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    if pdev_2.p2p_enabled != FEATURE_TRUE {
        return ResultEqual(result, RMI_ERROR_DEVICE);
    }
    if pdev_2.p2p_stream_valid == RMM_TRUE {
        return ResultEqual(result, RMI_ERROR_DEVICE);
    }
    if ide_sid > 255 {
        return ResultEqual(result, RMI_ERROR_INPUT);
    }
    // Success case: verify all post-conditions

    result.is_Ok() && GranuleAt(s, stream_ptr).state == P2P_STREAM && pdev_1.p2p_stream_valid
        == RMM_TRUE && pdev_1.p2p_stream == stream_ptr && pdev_1.state == PDEV_COMMUNICATING
        && pdev_1.comm_state == DEV_COMM_PENDING && pdev_2.p2p_stream_valid == RMM_TRUE
        && pdev_2.p2p_stream == stream_ptr && pdev_2.state == PDEV_COMMUNICATING
        && pdev_2.comm_state == DEV_COMM_PENDING
}