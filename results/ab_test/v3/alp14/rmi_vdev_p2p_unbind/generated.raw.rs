pub open spec fn rmi_vdev_p2p_unbind_spec(result: RmiCommandReturnCode, stream_ptr: Address, rd: Address, rec_ptr: Address, pdev_1_ptr: Address, pdev_2_ptr: Address, vdev_1_ptr: Address, vdev_2_ptr: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(rd);
    let rec = RecAt(rec_ptr);
    let stream = P2PStreamAt(stream_ptr);
    let pdev_1 = PdevAt(pdev_1_ptr);
    let pdev_2 = PdevAt(pdev_2_ptr);
    let vdev_1 = VdevAt(vdev_1_ptr);
    let vdev_2 = VdevAt(vdev_2_ptr);
    (old_s.impl_features().feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (!AddrIsGranuleAligned(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((pdev_1.p2p_stream_valid != RMM_TRUE || pdev_1.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((pdev_2.p2p_stream_valid != RMM_TRUE || pdev_2.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_1_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_1.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_1.pdev != pdev_1_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_1.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_1.p2p_bound != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_1.p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_1.p2p_peer != vdev_2.vdev_id ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!AddrIsGranuleAligned(vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_2_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_2.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_2.pdev != pdev_2_ptr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_2.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_2.p2p_bound != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_2.p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_2.p2p_peer != vdev_1.vdev_id ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((old_s.impl_features().feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(rd)
        && PaIsDelegable(rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(rec_ptr)
        && PaIsDelegable(rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == REC
        && rec.state != REC_RUNNING
        && rec.owner == rd
        && AddrIsGranuleAligned(stream_ptr)
        && PaIsDelegable(stream_ptr)
        && GranuleAt(old_s, stream_ptr).state == P2P_STREAM
        && AddrIsGranuleAligned(pdev_1_ptr)
        && PaIsDelegable(pdev_1_ptr)
        && GranuleAt(old_s, pdev_1_ptr).state == PDEV
        && pdev_1.p2p_stream_valid == RMM_TRUE
        && pdev_1.p2p_stream == stream_ptr
        && AddrIsGranuleAligned(pdev_2_ptr)
        && PaIsDelegable(pdev_2_ptr)
        && GranuleAt(old_s, pdev_2_ptr).state == PDEV
        && pdev_2.p2p_stream_valid == RMM_TRUE
        && pdev_2.p2p_stream == stream_ptr
        && AddrIsGranuleAligned(vdev_1_ptr)
        && PaIsDelegable(vdev_1_ptr)
        && GranuleAt(old_s, vdev_1_ptr).state == VDEV
        && vdev_1.realm == rd
        && vdev_1.pdev == pdev_1_ptr
        && vdev_1.comm_state == DEV_COMM_IDLE
        && vdev_1.p2