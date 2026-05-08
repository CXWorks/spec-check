pub open spec fn RMI_VDEV_COMMUNICATE_spec(
    s: S,
    pdev_ptr: Address,
    vdev_ptr: Address,
    data_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let vdev_pre = VdevAt(s, vdev_ptr);
    let vdev = VdevAt(s, vdev_ptr);
    let data = RmiDevCommDataAt(s, data_ptr);

    let da_supp_check = ImplFeatures(s).feat_da != FEATURE_TRUE;
    let pdev_align_check = !AddrIsGranuleAligned(s, pdev_ptr);
    let pdev_bound_check = !PaIsDelegable(s, pdev_ptr);
    let pdev_gran_state_check = GranuleAt(s, pdev_ptr).state != GRANULE_STATE_PDEV;
    let vdev_align_check = !AddrIsGranuleAligned(s, vdev_ptr);
    let vdev_bound_check = !PaIsDelegable(s, vdev_ptr);
    let vdev_gran_state_check = GranuleAt(s, vdev_ptr).state != GRANULE_STATE_VDEV;
    let data_align_check = !AddrIsGranuleAligned(s, data_ptr);
    let data_pas_check = !GranuleAccessPermitted(s, data_ptr, PAS_NS);
    let req_align_check = !AddrIsGranuleAligned(s, data.enter.req_addr);
    let req_pas_check = !GranuleAccessPermitted(s, data.enter.req_addr, PAS_NS);
    let resp_align_check = !AddrIsGranuleAligned(s, data.enter.resp_addr);
    let resp_pas_check = !GranuleAccessPermitted(s, data.enter.resp_addr, PAS_NS);
    let resp_len_check = data.enter.resp_len > RMM_GRANULE_SIZE;
    let vdev_pdev_check = vdev_pre.pdev != pdev_ptr;
    let comm_state_check = vdev_pre.comm_state == DEV_COMM_IDLE;
    let device_comm_result = DeviceCommunicate1(s, vdev_pre);
    let device_comm_error = device_comm_result == DEV_COMM_ERROR;
    let device_comm_idle = device_comm_result == DEV_COMM_IDLE;

    (da_supp_check ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && (!da_supp_check ==> ((
    pdev_align_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!pdev_align_check ==> ((
    pdev_bound_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!pdev_bound_check ==> ((
    pdev_gran_state_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!pdev_gran_state_check ==> (
    (vdev_align_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!vdev_align_check ==> ((
    vdev_bound_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!vdev_bound_check ==> ((
    vdev_gran_state_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!vdev_gran_state_check ==> (
    (data_align_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!data_align_check ==> ((
    data_pas_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!data_pas_check ==> ((
    req_align_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!req_align_check ==> ((
    req_pas_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!req_pas_check ==> ((
    resp_align_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!resp_align_check ==> ((
    resp_pas_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!resp_pas_check ==> ((
    resp_len_check ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!resp_len_check ==> ((
    vdev_pdev_check ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (!vdev_pdev_check ==> ((
    comm_state_check ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (!comm_state_check ==> ((
    vdev.comm_state == device_comm_result) && (device_comm_error ==> vdev.vdev_state
        == VDEV_STATE_ERROR) && ((device_comm_idle && vdev_pre.op == VDEV_OP_UNLOCK)
        ==> vdev.vdev_state == VDEV_STATE_UNLOCKED) && ((device_comm_idle && vdev_pre.op
        == VDEV_OP_LOCK) ==> vdev.vdev_state == VDEV_STATE_LOCKED) && ((device_comm_idle
        && vdev_pre.op == VDEV_OP_LOCK) ==> vdev.attest_info.lock_nonce == VdevGenerateNonce(
        s,
        vdev_pre,
    )) && ((device_comm_idle && vdev_pre.op == VDEV_OP_START) ==> vdev.vdev_state
        == VDEV_STATE_STARTED) && ((device_comm_idle && vdev_pre.op == VDEV_OP_GET_MEAS)
        ==> vdev.attest_info.meas_nonce == VdevGenerateNonce(s, vdev_pre)) && ((device_comm_idle
        && vdev_pre.op == VDEV_OP_GET_REPORT) ==> vdev.attest_info.report_nonce
        == VdevGenerateNonce(s, vdev_pre)) && (device_comm_idle ==> vdev.op == VDEV_OP_NONE)
        && result.is_Ok()))))))))))))))))))))))))))))))))
}