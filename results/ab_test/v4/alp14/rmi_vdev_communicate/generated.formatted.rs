pub open spec fn RMI_VDEV_COMMUNICATE_spec(
    s: S,
    pdev_ptr: Address,
    vdev_ptr: Address,
    data_ptr: Address,
    result: RmiCommandReturnCode,
    vdev_post: RmmVdev,
) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let vdev_pre = VdevAt(s, vdev_ptr);
    let data = RmiDevCommDataAt(s, data_ptr);
    let dev_comm_result = DeviceCommunicate1(s, vdev_pre);

    ((!ImplFeatures(s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && ((
    !AddrIsGranuleAligned(s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !PaIsDelegable(s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(
        s,
        pdev_ptr,
    ).state != PDEV) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(
        s,
        vdev_ptr,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PaIsDelegable(s, vdev_ptr)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((GranuleAt(s, vdev_ptr).state != VDEV) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsGranuleAligned(s, data_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !GranuleAccessPermitted(s, data_ptr, PAS_NS)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !AddrIsGranuleAligned(s, data.enter.req_addr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    !GranuleAccessPermitted(s, data.enter.req_addr, PAS_NS)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((!AddrIsGranuleAligned(s, data.enter.resp_addr)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((!GranuleAccessPermitted(s, data.enter.resp_addr, PAS_NS)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((data.enter.resp_len > RMM_GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((
    vdev_pre.pdev != pdev_ptr) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && ((vdev_pre.comm_state
        == DEV_COMM_IDLE) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (result.is_Ok()
        ==> vdev_post.comm_state == dev_comm_result && ((dev_comm_result == DEV_COMM_ERROR)
        ==> vdev_post.vdev_state == VDEV_ERROR) && (((dev_comm_result == DEV_COMM_IDLE
        && vdev_pre.op == VDEV_OP_UNLOCK)) ==> vdev_post.vdev_state == VDEV_UNLOCKED) && (((
    dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK)) ==> vdev_post.vdev_state
        == VDEV_LOCKED) && (((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK))
        ==> vdev_post.attest_info.lock_nonce == VdevGenerateNonce(s, vdev_pre)) && (((
    dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_START)) ==> vdev_post.vdev_state
        == VDEV_STARTED) && (((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_MEAS))
        ==> vdev_post.attest_info.meas_nonce == VdevGenerateNonce(s, vdev_pre)) && (((
    dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_REPORT))
        ==> vdev_post.attest_info.report_nonce == VdevGenerateNonce(s, vdev_pre)) && ((
    dev_comm_result == DEV_COMM_IDLE) ==> vdev_post.op == VDEV_OP_NONE))
}