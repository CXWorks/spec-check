```verus
pub open spec fn rmi_vdev_p2p_unbind_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    stream_ptr: Address,
    pdev_1_ptr: Address,
    pdev_2_ptr: Address,
    vdev_1_ptr: Address,
    vdev_2_ptr: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let stream = P2PStreamAt(old_s, stream_ptr);
    let pdev_1 = PdevAt(old_s, pdev_1_ptr);
    let pdev_2 = PdevAt(old_s, pdev_2_ptr);
    let vdev_1 = VdevAt(old_s, vdev_1_ptr);
    let vdev_2 = VdevAt(old_s, vdev_2_ptr);

    // Failure conditions in order of precedence
    (!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==>
        ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    
    (ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> (
        (!AddrIsGranuleAligned(old_s, rd) ==>
            ResultEqual(result, RMI_ERROR_INPUT)) &&
        (AddrIsGranuleAligned(old_s, rd) ==> (
            (!PaIsDelegable(old_s, rd) ==>
                ResultEqual(result, RMI_ERROR_INPUT)) &&
            (PaIsDelegable(old_s, rd) ==> (
                (GranuleAt(old_s, rd).state != RD() ==>
                    ResultEqual(result, RMI_ERROR_INPUT)) &&
                (GranuleAt(old_s, rd).state == RD() ==> (
                    (!AddrIsGranuleAligned(old_s, rec_ptr) ==>
                        ResultEqual(result, RMI_ERROR_INPUT)) &&
                    (AddrIsGranuleAligned(old_s, rec_ptr) ==> (
                        (!PaIsDelegable(old_s, rec_ptr) ==>
                            ResultEqual(result, RMI_ERROR_INPUT)) &&
                        (PaIsDelegable(old_s, rec_ptr) ==> (
                            (GranuleAt(old_s, rec_ptr).state != REC() ==>
                                ResultEqual(result, RMI_ERROR_INPUT)) &&
                            (GranuleAt(old_s, rec_ptr).state == REC() ==> (
                                (rec.state == REC_RUNNING() ==>
                                    ResultEqual(result, RMI_ERROR_REC)) &&
                                (rec.state != REC_RUNNING() ==> (
                                    (rec.owner != rd ==>
                                        ResultEqual(result, RMI_ERROR_REC)) &&
                                    (rec.owner == rd ==> (
                                        (!AddrIsGranuleAligned(old_s, stream_ptr) ==>
                                            ResultEqual(result, RMI_ERROR_INPUT)) &&
                                        (AddrIsGranuleAligned(old_s, stream_ptr) ==> (
                                            (!PaIsDelegable(old_s, stream_ptr) ==>
                                                ResultEqual(result, RMI_ERROR_INPUT)) &&
                                            (PaIsDelegable(old_s, stream_ptr) ==> (
                                                (GranuleAt(old_s, stream_ptr).state != P2P_STREAM() ==>
                                                    ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                (GranuleAt(old_s, stream_ptr).state == P2P_STREAM() ==> (
                                                    (!AddrIsGranuleAligned(old_s, pdev_1_ptr) ==>
                                                        ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                    (AddrIsGranuleAligned(old_s, pdev_1_ptr) ==> (
                                                        (!PaIsDelegable(old_s, pdev_1_ptr) ==>
                                                            ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                        (PaIsDelegable(old_s, pdev_1_ptr) ==> (
                                                            (GranuleAt(old_s, pdev_1_ptr).state != PDEV() ==>
                                                                ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                            (GranuleAt(old_s, pdev_1_ptr).state == PDEV() ==> (
                                                                ((pdev_1.p2p_stream_valid != RMM_TRUE()) || (pdev_1.p2p_stream != stream_ptr) ==>
                                                                    ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                ((pdev_1.p2p_stream_valid == RMM_TRUE()) && (pdev_1.p2p_stream == stream_ptr) ==> (
                                                                    (!AddrIsGranuleAligned(old_s, pdev_2_ptr) ==>
                                                                        ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                    (AddrIsGranuleAligned(old_s, pdev_2_ptr) ==> (
                                                                        (!PaIsDelegable(old_s, pdev_2_ptr) ==>
                                                                            ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                        (PaIsDelegable(old_s, pdev_2_ptr) ==> (
                                                                            (GranuleAt(old_s, pdev_2_ptr).state != PDEV() ==>
                                                                                ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                            (GranuleAt(old_s, pdev_2_ptr).state == PDEV() ==> (
                                                                                ((pdev_2.p2p_stream_valid != RMM_TRUE()) || (pdev_2.p2p_stream != stream_ptr) ==>
                                                                                    ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                                ((pdev_2.p2p_stream_valid == RMM_TRUE()) && (pdev_2.p2p_stream == stream_ptr) ==> (
                                                                                    (!AddrIsGranuleAligned(old_s, vdev_1_ptr) ==>
                                                                                        ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                                    (AddrIsGranuleAligned(old_s, vdev_1_ptr) ==> (
                                                                                        (!PaIsDelegable(old_s, vdev_1_ptr) ==>
                                                                                            ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                                        (PaIsDelegable(old_s, vdev_1_ptr) ==> (
                                                                                            (GranuleAt(old_s, vdev_1_ptr).state != VDEV() ==>
                                                                                                ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                                            (GranuleAt(old_s, vdev_1_ptr).state == VDEV() ==> (
                                                                                                (vdev_1.realm != rd ==>
                                                                                                    ResultEqual(result, RMI_ERROR_INPUT)) &&
                                                                                                (vdev_1.realm == rd ==> (
                                                                                                    (vdev_1.pdev != pdev_1_ptr ==>
                                                                                                        ResultEqual(result, RMI_ERROR