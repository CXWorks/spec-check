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
) -> Result<(RmmVdev, RmmVdev), RmiStatusCode> {
    let realm = RealmAt(s, rd);
    let rec = RecAt(s, rec_ptr);
    let stream = P2PStreamAt(s, stream_ptr);
    let pdev_1 = PdevAt(s, pdev_1_ptr);
    let pdev_2 = PdevAt(s, pdev_2_ptr);
    let vdev_1 = VdevAt(s, vdev_1_ptr);
    let vdev_2 = VdevAt(s, vdev_2_ptr);

    // Failure condition: da_supp
    if ImplFeatures(s).feat_da != FEATURE_TRUE {
        return Err(RMI_ERROR_NOT_SUPPORTED);
    }

    // Failure condition: rd_align
    if !AddrIsGranuleAligned(s, rd) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: rd_bound
    if !PaIsDelegable(s, rd) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: rd_state
    if GranuleAt(s, rd).state != RD {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: rec_align
    if !AddrIsGranuleAligned(s, rec_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: rec_bound
    if !PaIsDelegable(s, rec_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: rec_gran_state
    if GranuleAt(s, rec_ptr).state != REC {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: rec_state
    if rec.state == REC_RUNNING {
        return Err(RMI_ERROR_REC);
    }

    // Failure condition: rec_owner
    if rec.owner != rd {
        return Err(RMI_ERROR_REC);
    }

    // Failure condition: stream_align
    if !AddrIsGranuleAligned(s, stream_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: stream_bound
    if !PaIsDelegable(s, stream_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: stream_state
    if GranuleAt(s, stream_ptr).state != P2P_STREAM {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_1_align
    if !AddrIsGranuleAligned(s, pdev_1_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_1_bound
    if !PaIsDelegable(s, pdev_1_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_1_gran_state
    if GranuleAt(s, pdev_1_ptr).state != PDEV {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_1_stream
    if pdev_1.p2p_stream_valid != RMM_TRUE || pdev_1.p2p_stream != stream_ptr {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_2_align
    if !AddrIsGranuleAligned(s, pdev_2_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_2_bound
    if !PaIsDelegable(s, pdev_2_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_2_gran_state
    if GranuleAt(s, pdev_2_ptr).state != PDEV {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: pdev_2_stream
    if pdev_2.p2p_stream_valid != RMM_TRUE || pdev_2.p2p_stream != stream_ptr {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_1_align
    if !AddrIsGranuleAligned(s, vdev_1_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_1_bound
    if !PaIsDelegable(s, vdev_1_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_1_gran_state
    if GranuleAt(s, vdev_1_ptr).state != VDEV {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_1_realm
    if vdev_1.realm != rd {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_1_pdev
    if vdev_1.pdev != pdev_1_ptr {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_1_comm
    if vdev_1.comm_state != DEV_COMM_IDLE {
        return Err(RMI_ERROR_DEVICE);
    }

    // Failure condition: vdev_1_p2p_bound
    if vdev_1.p2p_bound != FEATURE_TRUE {
        return Err(RMI_ERROR_DEVICE);
    }

    // Failure condition: vdev_1_p2p_stream
    if vdev_1.p2p_stream != stream_ptr {
        return Err(RMI_ERROR_DEVICE);
    }

    // Failure condition: vdev_1_p2p_peer
    if vdev_1.p2p_peer != vdev_2.vdev_id {
        return Err(RMI_ERROR_DEVICE);
    }

    // Failure condition: vdev_2_align
    if !AddrIsGranuleAligned(s, vdev_2_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_2_bound
    if !PaIsDelegable(s, vdev_2_ptr) {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_2_gran_state
    if GranuleAt(s, vdev_2_ptr).state != VDEV {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_2_realm
    if vdev_2.realm != rd {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_2_pdev
    if vdev_2.pdev != pdev_2_ptr {
        return Err(RMI_ERROR_INPUT);
    }

    // Failure condition: vdev_2_comm
    if vdev_2.comm_state != DEV_COMM_IDLE {
        return Err(RMI_ERROR_DEVICE);
    }

    // Failure condition: vdev_2_p2p_bound
    if vdev_2.p2p_bound !=