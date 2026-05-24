pub open spec fn rmi_pdev_p2p_disconnect_spec(result: RmiCommandReturnCode, stream_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, old_s: S, new_s: S) -> bool {
    let stream = P2PStreamAt(old_s, stream_ptr);
    let pdev_1 = PdevAt(old_s, pdev_1_ptr);
    let pdev_2 = PdevAt(old_s, pdev_2_ptr);
    
    (!ImplFeatures(old_s).feat_da.is_equal_true() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, stream_ptr).state != GranuleState::P2PStream ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_1_ptr).state != GranuleState::Pdev ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_2_ptr).state != GranuleState::Pdev ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((pdev_1.p2p_stream_valid != RmmBool::RmmTrue
        || pdev_1.p2p_stream != stream_ptr
        || pdev_2.p2p_stream_valid != RmmBool::RmmTrue
        || pdev_2.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da.is_equal_true()
        && AddrIsGranuleAligned(old_s, stream_ptr)
        && PaIsDelegable(old_s, stream_ptr)
        && GranuleAt(old_s, stream_ptr).state == GranuleState::P2PStream
        && AddrIsGranuleAligned(old_s, pdev_1_ptr)
        && PaIsDelegable(old_s, pdev_1_ptr)
        && GranuleAt(old_s, pdev_1_ptr).state == GranuleState::Pdev
        && AddrIsGranuleAligned(old_s, pdev_2_ptr)
        && PaIsDelegable(old_s, pdev_2_ptr)
        && GranuleAt(old_s, pdev_2_ptr).state == GranuleState::Pdev
        && pdev_1.p2p_stream_valid == RmmBool::RmmTrue
        && pdev_1.p2p_stream == stream_ptr
        && pdev_2.p2p_stream_valid == RmmBool::RmmTrue
        && pdev_2.p2p_stream == stream_ptr)
      ==> (result.is_Ok()
        && GranuleAt(new_s, stream_ptr).state == GranuleState::Delegated
        && PdevAt(new_s, pdev_1_ptr).p2p_stream_valid == RmmBool::RmmFalse
        && PdevAt(new_s, pdev_1_ptr).state == RmmPdevState::PdevCommunicating
        && PdevAt(new_s, pdev_1_ptr).comm_state == RmmDevCommState::DevCommPending
        && PdevAt(new_s, pdev_2_ptr).p2p_stream_valid == RmmBool::RmmFalse
        && PdevAt(new_s, pdev_2_ptr).state == RmmPdevState::PdevCommunicating
        && PdevAt(new_s, pdev_2_ptr).comm_state == RmmDevCommState::DevCommPending))
}