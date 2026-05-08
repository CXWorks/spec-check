pub open spec fn RMI_PDEV_P2P_DISCONNECT_spec(
    old_s: S,
    new_s: S,
    stream_ptr: Address,
    pdev_1_ptr: Address,
    pdev_2_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let stream = P2PStreamAt(old_s, stream_ptr);
    let pdev_1 = PdevAt(old_s, pdev_1_ptr);
    let pdev_2 = PdevAt(old_s, pdev_2_ptr);

    ((!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        stream_ptr,
    ).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(
        pdev_1_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(pdev_1_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !AddrIsGranuleAligned(pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        pdev_2_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, pdev_2_ptr).state != PDEV
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((pdev_1.p2p_stream_valid != RMM_TRUE
        || pdev_1.p2p_stream != stream_ptr || pdev_2.p2p_stream_valid != RMM_TRUE
        || pdev_2.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    result.is_Ok() ==> (GranuleAt(new_s, stream_ptr).state == DELEGATED && PdevAt(
        new_s,
        pdev_1_ptr,
    ).p2p_stream_valid == RMM_FALSE && PdevAt(new_s, pdev_1_ptr).state == PDEV_COMMUNICATING
        && PdevAt(new_s, pdev_1_ptr).comm_state == DEV_COMM_PENDING && PdevAt(
        new_s,
        pdev_2_ptr,
    ).p2p_stream_valid == RMM_FALSE && PdevAt(new_s, pdev_2_ptr).state == PDEV_COMMUNICATING
        && PdevAt(new_s, pdev_2_ptr).comm_state == DEV_COMM_PENDING)))
}