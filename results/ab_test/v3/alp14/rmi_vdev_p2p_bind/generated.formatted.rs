pub open spec fn rmi_vdev_p2p_bind_spec(
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    stream_ptr: Address,
    pdev_1_ptr: Address,
    pdev_2_ptr: Address,
    vdev_1_ptr: Address,
    vdev_2_ptr: Address
) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let stream = P2PStreamAt(old_s, stream_ptr);
    let pdev_1 = PdevAt(old_s, pdev_1_ptr);
    let pdev_2 = PdevAt(old_s, pdev_2_ptr);
    let vdev_1 = VdevAt(old_s, vdev_1_ptr);
    let vdev_2 = VdevAt(old_s, vdev_2_ptr);
    
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (!AddrIsGranuleAligned(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((pdev_1.p2p_stream_valid != RMM_TRUE || pdev_1.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((pdev_2.p2p_stream_valid != RMM_TRUE || pdev_2.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_1_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_1.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_1.pdev != pdev_1_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_1.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!VdevAttestInfoEqual(old_s, vdev_1.attest_info, rec.vdev_attest_info_1) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_1.p2p_bound != FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!AddrIsGranuleAligned(old_s, vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_2_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_2.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_2.pdev != pdev_2_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_2.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!VdevAttestInfoEqual(old_s, vdev_2.attest_info, rec.vdev_attest_info_2) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_2.p2p_bound != FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (result.is_Ok() ==> (
        VdevAt(new_s, vdev_1_ptr).op == VDEV_OP_P2P_BIND
        && VdevAt(new_s, vdev_1_ptr).comm_state == DEV_COMM_PENDING
        && VdevAt(new_s, vdev_1_ptr).p2p_bound == FEATURE_TRUE
        && VdevAt(new_s, vdev_1_ptr).p2p_stream == stream_ptr
        && VdevAt(new_s, vdev_1_ptr).p2p_peer == VdevAt(old_s, vdev_2_ptr).vdev_id
        && VdevAt(new_s, vdev_2_ptr).op == VDEV_OP_P2P_BIND
        && VdevAt(new_s, vdev_2_ptr).comm_state == DEV_COMM_PENDING
        && VdevAt(new_s, vdev_2_ptr).p2p_bound == FEATURE_TRUE
        && VdevAt(new_s, vdev_2_ptr).p2p_stream == stream_ptr
        && VdevAt(new_s, vdev_2_ptr).p2p_peer == VdevAt(old_s, vdev_1_ptr).vdev_id
    ))
}