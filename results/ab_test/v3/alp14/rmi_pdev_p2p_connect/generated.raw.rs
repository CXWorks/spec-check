pub open spec fn rmi_pdev_p2p_connect_spec(result: RmiCommandReturnCode, stream_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, ide_sid: u64, old_s: S, new_s: S) -> bool {
    let stream = P2PStreamAt(old_s, stream_ptr);
    let pdev_1 = PdevAt(old_s, pdev_1_ptr);
    let pdev_2 = PdevAt(old_s, pdev_2_ptr);
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDram(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, stream_ptr).state != DELEGATED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev_1.state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev_1.p2p_enabled != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (pdev_1.p2p_stream_valid == RMM_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!AddrIsGranuleAligned(pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev_2.state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev_2.p2p_enabled != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (pdev_2.p2p_stream_valid == RMM_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (ide_sid > 255 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(stream_ptr)
        && PaIsDelegableDram(stream_ptr)
        && GranuleAt(old_s, stream_ptr).state == DELEGATED
        && AddrIsGranuleAligned(pdev_1_ptr)
        && PaIsDelegable(pdev_1_ptr)
        && GranuleAt(old_s, pdev_1_ptr).state == PDEV
        && pdev_1.state == PDEV_READY
        && pdev_1.p2p_enabled == FEATURE_TRUE
        && pdev_1.p2p_stream_valid != RMM_TRUE
        && AddrIsGranuleAligned(pdev_2_ptr)
        && PaIsDelegable(pdev_2_ptr)
        && GranuleAt(old_s, pdev_2_ptr).state == PDEV
        && pdev_2.state == PDEV_READY
        && pdev_2.p2p_enabled == FEATURE_TRUE
        && pdev_2.p2p_stream_valid != RMM_TRUE
        && ide_sid <= 255)
        ==> (result.is_Ok()
            && GranuleAt(new_s, stream_ptr).state == P2P_STREAM
            && PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == RMM_TRUE
            && PdevAt(new_s, pdev_1_ptr).p2p_stream == stream_ptr
            && PdevAt(new_s, pdev_1_ptr).state == PDEV_COMMUNICATING
            && PdevAt(new_s, pdev_1_ptr).comm_state == DEV_COMM_PENDING
            && PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == RMM_TRUE
            && PdevAt(new_s, pdev_2_ptr).p2p_stream == stream_ptr
            && PdevAt(new_s, pdev_2_ptr).state == PDEV_COMMUNICATING
            && PdevAt(new_s, pdev_2_ptr).comm_state == DEV_COMM_PENDING))
}