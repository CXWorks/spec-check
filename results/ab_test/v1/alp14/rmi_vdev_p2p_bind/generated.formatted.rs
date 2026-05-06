```rust
pub open spec fn RMI_VDEV_P2P_BIND_spec(
    s: S,
    stream_ptr: Address,
    rd: Address,
    rec_ptr: Address,
    pdev_1_ptr: Address,
    pdev_2_ptr: Address,
    vdev_1_ptr: Address,
    vdev_2_ptr: Address,
) -> Result<(), RmiStatusCode> {
    let realm = RealmAt(rd);
    let rec = RecAt(rec_ptr);
    let stream = P2PStreamAt(stream_ptr);
    let pdev_1 = PdevAt(pdev_1_ptr);
    let pdev_2 = PdevAt(pdev_2_ptr);
    let vdev_1 = VdevAt(vdev_1_ptr);
    let vdev_2 = VdevAt(vdev_2_ptr);

    // Check da_supp: ImplFeatures().feat_da != FEATURE_TRUE
    if !ImplFeaturesFeatDA(s) {
        return Err(RmiStatusCode::RMI_ERROR_NOT_SUPPORTED);
    }

    // Check rd_align: !AddrIsGranuleAligned(rd)
    if !AddrIsGranuleAligned(rd) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check rd_bound: !PaIsDelegable(rd)
    if !PaIsDelegable(rd) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check rd_state: GranuleAt(rd).state != RD
    if GranuleAt(s, rd).state != RmmGranuleState::RD {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check rec_align: !AddrIsGranuleAligned(rec_ptr)
    if !AddrIsGranuleAligned(rec_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check rec_bound: !PaIsDelegable(rec_ptr)
    if !PaIsDelegable(rec_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check rec_gran_state: GranuleAt(rec_ptr).state != REC
    if GranuleAt(s, rec_ptr).state != RmmGranuleState::REC {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check rec_state: rec.state == REC_RUNNING
    if rec.state == RmmRecState::REC_RUNNING {
        return Err(RmiStatusCode::RMI_ERROR_REC);
    }

    // Check rec_owner: rec.owner != rd
    if rec.owner != rd {
        return Err(RmiStatusCode::RMI_ERROR_REC);
    }

    // Check stream_align: !AddrIsGranuleAligned(stream_ptr)
    if !AddrIsGranuleAligned(stream_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check stream_bound: !PaIsDelegable(stream_ptr)
    if !PaIsDelegable(stream_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check stream_state: GranuleAt(stream_ptr).state != P2P_STREAM
    if GranuleAt(s, stream_ptr).state != RmmGranuleState::P2P_STREAM {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_1_align: !AddrIsGranuleAligned(pdev_1_ptr)
    if !AddrIsGranuleAligned(pdev_1_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_1_bound: !PaIsDelegable(pdev_1_ptr)
    if !PaIsDelegable(pdev_1_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_1_gran_state: GranuleAt(pdev_1_ptr).state != PDEV
    if GranuleAt(s, pdev_1_ptr).state != RmmGranuleState::PDEV {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_1_stream: (pdev_1.p2p_stream_valid != RMM_TRUE || pdev_1.p2p_stream != stream_ptr)
    if pdev_1.p2p_stream_valid != RmmBool::RMM_TRUE || pdev_1.p2p_stream != stream_ptr {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_2_align: !AddrIsGranuleAligned(pdev_2_ptr)
    if !AddrIsGranuleAligned(pdev_2_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_2_bound: !PaIsDelegable(pdev_2_ptr)
    if !PaIsDelegable(pdev_2_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_2_gran_state: GranuleAt(pdev_2_ptr).state != PDEV
    if GranuleAt(s, pdev_2_ptr).state != RmmGranuleState::PDEV {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check pdev_2_stream: (pdev_2.p2p_stream_valid != RMM_TRUE || pdev_2.p2p_stream != stream_ptr)
    if pdev_2.p2p_stream_valid != RmmBool::RMM_TRUE || pdev_2.p2p_stream != stream_ptr {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check vdev_1_align: !AddrIsGranuleAligned(vdev_1_ptr)
    if !AddrIsGranuleAligned(vdev_1_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check vdev_1_bound: !PaIsDelegable(vdev_1_ptr)
    if !PaIsDelegable(vdev_1_ptr) {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check vdev_1_gran_state: GranuleAt(vdev_1_ptr).state != VDEV
    if GranuleAt(s, vdev_1_ptr).state != RmmGranuleState::VDEV {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check vdev_1_realm: vdev_1.realm != rd
    if vdev_1.realm != rd {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check vdev_1_pdev: vdev_1.pdev != pdev_1_ptr
    if vdev_1.pdev != pdev_1_ptr {
        return Err(RmiStatusCode::RMI_ERROR_INPUT);
    }

    // Check vdev_1_comm: vdev_1.comm_state != DEV_COMM_IDLE
    if vdev_1.comm_state != RmmDevCommState::DEV_COMM_IDLE {
        return Err(RmiStatusCode::RMI_ERROR_DEVICE);
    }

    // Check vdev_1_attest_info: !