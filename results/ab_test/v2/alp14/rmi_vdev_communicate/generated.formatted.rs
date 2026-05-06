pub open spec fn rmi_vdev_communicate_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    vdev_ptr: Address,
    data_ptr: Address,
) -> bool {
    // Failure conditions
    let da_supp_fail = old_s.impl_features().feat_da != FEATURE_TRUE ==> ResultEqual(
        result,
        RMI_ERROR_NOT_SUPPORTED,
    );

    let pdev_align_fail = !AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let pdev_bound_fail = !PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let pdev_gran_state_fail = GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let vdev_align_fail = !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let vdev_bound_fail = !PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let vdev_gran_state_fail = GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let data_align_fail = !AddrIsGranuleAligned(data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);

    let data_pas_fail = !GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let data = RmiDevCommDataAt(old_s, data_ptr);

    let req_align_fail = !AddrIsGranuleAligned(data.enter.req_addr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let req_pas_fail = !GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let resp_align_fail = !AddrIsGranuleAligned(data.enter.resp_addr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let resp_pas_fail = !GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS)
        ==> ResultEqual(result, RMI_ERROR_INPUT);

    let resp_len_fail = data.enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let pdev = PdevAt(old_s, pdev_ptr);

    let vdev_pdev_fail = vdev_pre.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE);

    let comm_state_fail = vdev_pre.comm_state == DEV_COMM_IDLE ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    );

    // Success conditions
    let vdev = VdevAt(new_s, vdev_ptr);
    let new_comm_state = DeviceCommunicate1(old_s, vdev_pre);

    let comm_state_succ = vdev.comm_state == new_comm_state;

    let error_succ = (new_comm_state == DEV_COMM_ERROR) ==> vdev.vdev_state == VDEV_ERROR;

    let unlock_state_succ = (new_comm_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_UNLOCK)
        ==> vdev.vdev_state == VDEV_UNLOCKED;

    let lock_state_succ = (new_comm_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK)
        ==> vdev.vdev_state == VDEV_LOCKED;

    let lock_nonce_succ = (new_comm_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK)
        ==> vdev.attest_info.lock_nonce == VdevGenerateNonce(old_s, vdev_pre);

    let start_state_succ = (new_comm_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_START)
        ==> vdev.vdev_state == VDEV_STARTED;

    let meas_nonce_succ = (new_comm_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_MEAS)
        ==> vdev.attest_info.meas_nonce == VdevGenerateNonce(old_s, vdev_pre);

    let report_nonce_succ = (new_comm_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_REPORT)
        ==> vdev.attest_info.report_nonce == VdevGenerateNonce(old_s, vdev_pre);

    let op_succ = (new_comm_state == DEV_COMM_IDLE) ==> vdev.op == VDEV_OP_NONE;

    // Combine all conditions
    da_supp_fail && pdev_align_fail && pdev_bound_fail && pdev_gran_state_fail && vdev_align_fail
        && vdev_bound_fail && vdev_gran_state_fail && data_align_fail && data_pas_fail
        && req_align_fail && req_pas_fail && resp_align_fail && resp_pas_fail && resp_len_fail
        && vdev_pdev_fail && comm_state_fail && comm_state_succ && error_succ && unlock_state_succ
        && lock_state_succ && lock_nonce_succ && start_state_succ && meas_nonce_succ
        && report_nonce_succ && op_succ
}