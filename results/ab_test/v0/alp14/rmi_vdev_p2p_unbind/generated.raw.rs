```verus
pub open spec fn RMI_VDEV_P2P_UNBIND_spec(
    s: S,
    stream_ptr: Address,
    rd: Address,
    rec_ptr: Address,
    pdev_1_ptr: Address,
    pdev_2_ptr: Address,
    vdev_1_ptr: Address,
    vdev_2_ptr: Address,
    result: Result<(), RmiStatusCode>
) -> bool {
    let realm = RealmAt(s, rd);
    let rec = RecAt(s, rec_ptr);
    let stream = P2PStreamAt(s, stream_ptr);
    let pdev_1 = PdevAt(s, pdev_1_ptr);
    let pdev_2 = PdevAt(s, pdev_2_ptr);
    let vdev_1 = VdevAt(s, vdev_1_ptr);
    let vdev_2 = VdevAt(s, vdev_2_ptr);

    (ImplFeatures(s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    (!AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC)) &&
    (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC)) &&
    (!AddrIsGranuleAligned(s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, stream_ptr).state != P2P_STREAM ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, pdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, pdev_1_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((pdev_1.p2p_stream_valid != RMM_TRUE || pdev_1.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, pdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, pdev_2_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((pdev_2.p2p_stream_valid != RMM_TRUE || pdev_2.p2p_stream != stream_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(s, vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, vdev_1_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, vdev_1_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (vdev_1.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (vdev_1.pdev != pdev_1_ptr ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (vdev_1.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (vdev_1.p2p_bound != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (vdev_1.p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (vdev_1.p2p_peer != vdev_2.vdev_id ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (!AddrIsGranuleAligned(s, vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, vdev_2_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, vdev_2_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (vdev_2.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (vdev_2.pdev != pdev_2_ptr ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (vdev_2.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (vdev_2.p2p_bound != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (vdev_2.p2p_stream != stream_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (vdev_2.p2p_peer != vdev_1.vdev_id ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (result.is_Ok() ==> (
        vdev_1.op == VDEV_OP_P2P_UNBIND &&
        vdev_1.comm_state == DEV_COMM_PENDING &&
        vdev_1.p2p_bound == FEATURE_FALSE &&
        vdev_2.op == VDEV_OP_P2P_UNBIND &&
        vdev_2.comm_state == DEV_COMM_PENDING &&
        vdev_2.p2p_bound == FEATURE_FALSE
    ))
}
```