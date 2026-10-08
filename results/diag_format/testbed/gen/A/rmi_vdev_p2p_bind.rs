pub open spec fn rmi_vdev_p2p_bind_spec(stream_ptr: Address, rd: Address, rec_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, vdev_1_ptr: Address, vdev_2_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let feat_ok = ImplFeatures(old_s).feat_da == FEATURE_TRUE;
    let rd_ok = AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD;
    let rec_gran_ok = AddrIsGranuleAligned(old_s, rec_ptr) && PaIsDelegable(old_s, rec_ptr) && GranuleAt(old_s, rec_ptr).state == REC;
    let stream_ok = AddrIsGranuleAligned(old_s, stream_ptr) && PaIsDelegable(old_s, stream_ptr) && GranuleAt(old_s, stream_ptr).state == P2P_STREAM;
    let pdev_1_gran_ok = AddrIsGranuleAligned(old_s, pdev_1_ptr) && PaIsDelegable(old_s, pdev_1_ptr) && GranuleAt(old_s, pdev_1_ptr).state == PDEV;
    let pdev_2_gran_ok = AddrIsGranuleAligned(old_s, pdev_2_ptr) && PaIsDelegable(old_s, pdev_2_ptr) && GranuleAt(old_s, pdev_2_ptr).state == PDEV;
    let vdev_1_gran_ok = AddrIsGranuleAligned(old_s, vdev_1_ptr) && PaIsDelegable(old_s, vdev_1_ptr) && GranuleAt(old_s, vdev_1_ptr).state == VDEV;
    let vdev_2_gran_ok = AddrIsGranuleAligned(old_s, vdev_2_ptr) && PaIsDelegable(old_s, vdev_2_ptr) && GranuleAt(old_s, vdev_2_ptr).state == VDEV;
    let pdev_1_stream_bad = pdev_1_gran_ok && (PdevAt(old_s, pdev_1_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_1_ptr).p2p_stream != stream_ptr);
    let pdev_2_stream_bad = pdev_2_gran_ok && (PdevAt(old_s, pdev_2_ptr).p2p_stream_valid != RMM_TRUE || PdevAt(old_s, pdev_2_ptr).p2p_stream != stream_ptr);
    let vdev_1_input_bad = vdev_1_gran_ok && (VdevAt(old_s, vdev_1_ptr).realm != rd || VdevAt(old_s, vdev_1_ptr).pdev != pdev_1_ptr);
    let vdev_2_input_bad = vdev_2_gran_ok && (VdevAt(old_s, vdev_2_ptr).realm != rd || VdevAt(old_s, vdev_2_ptr).pdev != pdev_2_ptr);
    let input_fail = !rd_ok || !rec_gran_ok || !stream_ok || !pdev_1_gran_ok || !pdev_2_gran_ok || !vdev_1_gran_ok || !vdev_2_gran_ok
        || pdev_1_stream_bad || pdev_2_stream_bad || vdev_1_input_bad || vdev_2_input_bad;
    let rec_fail = rec_gran_ok && (RecAt(old_s, rec_ptr).state == REC_RUNNING || (rd_ok && RecAt(old_s, rec_ptr).owner != rd));
    let vdev_1_dev_fail = vdev_1_gran_ok && (VdevAt(old_s, vdev_1_ptr).comm_state != DEV_COMM_IDLE
        || (rec_gran_ok && !VdevAttestInfoEqual1(VdevAt(old_s, vdev_1_ptr).attest_info, RecAt(old_s, rec_ptr).vdev_attest_info_1))
        || VdevAt(old_s, vdev_1_ptr).p2p_bound != FEATURE_FALSE);
    let vdev_2_dev_fail = vdev_2_gran_ok && (VdevAt(old_s, vdev_2_ptr).comm_state != DEV_COMM_IDLE
        || (rec_gran_ok && !VdevAttestInfoEqual1(VdevAt(old_s, vdev_2_ptr).attest_info, RecAt(old_s, rec_ptr).vdev_attest_info_2))
        || VdevAt(old_s, vdev_2_ptr).p2p_bound != FEATURE_FALSE);
    let dev_fail = vdev_1_dev_fail || vdev_2_dev_fail;
    (!feat_ok ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((feat_ok && (input_fail || rec_fail || dev_fail)) ==>
        ((input_fail && ResultEqual(result, RMI_ERROR_INPUT))
         || (rec_fail && ResultEqual(result, RMI_ERROR_REC))
         || (dev_fail && ResultEqual(result, RMI_ERROR_DEVICE))))
    && (result.is_Err() ==> (
        VdevAt(new_s, vdev_1_ptr).op == VdevAt(old_s, vdev_1_ptr).op
        && VdevAt(new_s, vdev_1_ptr).comm_state == VdevAt(old_s, vdev_1_ptr).comm_state
        && VdevAt(new_s, vdev_1_ptr).p2p_bound == VdevAt(old_s, vdev_1_ptr).p2p_bound
        && VdevAt(new_s, vdev_1_ptr).p2p_stream == VdevAt(old_s, vdev_1_ptr).p2p_stream
        && VdevAt(new_s, vdev_1_ptr).p2p_peer == VdevAt(old_s, vdev_1_ptr).p2p_peer
        && VdevAt(new_s, vdev_2_ptr).op == VdevAt(old_s, vdev_2_ptr).op
        && VdevAt(new_s, vdev_2_ptr).comm_state == VdevAt(old_s, vdev_2_ptr).comm_state
        && VdevAt(new_s, vdev_2_ptr).p2p_bound == VdevAt(old_s, vdev_2_ptr).p2p_bound
        && VdevAt(new_s, vdev_2_ptr).p2p_stream == VdevAt(old_s, vdev_2_ptr).p2p_stream
        && VdevAt(new_s, vdev_2_ptr).p2p_peer == VdevAt(old_s, vdev_2_ptr).p2p_peer))
    && ((feat_ok && !input_fail && !rec_fail && !dev_fail) ==> (
        result.is_Ok()
        && VdevAt(new_s, vdev_1_ptr).op == VDEV_OP_P2P_BIND
        && VdevAt(new_s, vdev_1_ptr).comm_state == DEV_COMM_PENDING
        && VdevAt(new_s, vdev_1_ptr).p2p_bound == FEATURE_TRUE
        && VdevAt(new_s, vdev_1_ptr).p2p_stream == stream_ptr
        && VdevAt(new_s, vdev_1_ptr).p2p_peer == VdevAt(old_s, vdev_2_ptr).vdev_id
        && VdevAt(new_s, vdev_2_ptr).op == VDEV_OP_P2P_BIND
        && VdevAt(new_s, vdev_2_ptr).comm_state == DEV_COMM_PENDING
        && VdevAt(new_s, vdev_2_ptr).p2p_bound == FEATURE_TRUE
        && VdevAt(new_s, vdev_2_ptr).p2p_stream == stream_ptr
        && VdevAt(new_s, vdev_2_ptr).p2p_peer == VdevAt(old_s, vdev_1_ptr).vdev_id))
}
