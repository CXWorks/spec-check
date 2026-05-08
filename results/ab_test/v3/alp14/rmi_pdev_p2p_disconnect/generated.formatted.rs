pub open spec fn RMI_PDEV_P2P_DISCONNECT_spec(
    old_s: S,
    new_s: S,
    stream_ptr: Address,
    pdev_1_ptr: Address,
    pdev_2_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let stream = P2PStreamAt(old_s, stream_ptr);
    let pdev_1 = PdevAt(old_s, pdev_1_ptr);
    let pdev_2 = PdevAt(old_s, pdev_2_ptr);

    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // Failure condition: stream_align
    (!AddrIsGranuleAligned(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: stream_bound
    (!PaIsDelegable(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: stream_state
    (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_1_align
    (!AddrIsGranuleAligned(pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_1_bound
    (!PaIsDelegable(pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_1_gran_state
    (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_2_align
    (!AddrIsGranuleAligned(pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_2_bound
    (!PaIsDelegable(pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_2_gran_state
    (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: stream
    ((pdev_1.p2p_stream_valid != RMM_TRUE || pdev_1.p2p_stream != stream_ptr
        || pdev_2.p2p_stream_valid != RMM_TRUE || pdev_2.p2p_stream != stream_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) &&
    // Success condition: gran_state
    (result.is_Ok() ==> GranuleAt(new_s, stream_ptr).state == DELEGATED)
        &&
    // Success condition: pdev_1_p2p_stream_valid
    (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == RMM_FALSE)
        &&
    // Success condition: pdev_1_state
    (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).state == PDEV_COMMUNICATING)
        &&
    // Success condition: pdev_1_comm_state
    (result.is_Ok() ==> PdevAt(new_s, pdev_1_ptr).comm_state == DEV_COMM_PENDING)
        &&
    // Success condition: pdev_2_p2p_stream_valid
    (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == RMM_FALSE)
        &&
    // Success condition: pdev_2_state
    (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).state == PDEV_COMMUNICATING)
        &&
    // Success condition: pdev_2_comm_state
    (result.is_Ok() ==> PdevAt(new_s, pdev_2_ptr).comm_state == DEV_COMM_PENDING)
}