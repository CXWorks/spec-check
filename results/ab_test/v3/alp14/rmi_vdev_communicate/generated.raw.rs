pub open spec fn rmi_vdev_communicate_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, pdev_ptr: Address, vdev_ptr: Address, data_ptr: Address) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let vdev = VdevAt(new_s, vdev_ptr);
    let data = RmiDevCommDataAt(old_s, data_ptr);
    let dev_comm_result = DeviceCommunicate1(old_s, vdev_pre);
    
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(data.enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(data.enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (data.enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_pre.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_pre.comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(pdev_ptr)
        && PaIsDelegable(pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(vdev_ptr)
        && PaIsDelegable(vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && AddrIsGranuleAligned(data_ptr)
        && GranuleAccessPermitted(old_s, data_ptr, PAS_NS)
        && AddrIsGranuleAligned(data.enter.req_addr)
        && GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS)
        && AddrIsGranuleAligned(data.enter.resp_addr)
        && GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS)
        && data.enter.resp_len <= RMM_GRANULE_SIZE
        && vdev_pre.pdev == pdev_ptr
        && vdev_pre.comm_state != DEV_COMM_IDLE)
        ==> (result.is_Ok()
            && vdev.comm_state == dev_comm_result
            && (dev_comm_result == DEV_COMM_ERROR ==> vdev.vdev_state == VDEV_ERROR)
            && ((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_UNLOCK) ==> vdev.vdev_state == VDEV_UNLOCKED)
            && ((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK) ==> vdev.vdev_state == VDEV_LOCKED)
            && ((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK) ==> vdev.attest_info.lock_nonce == VdevGenerateNonce(old_s, vdev_pre))
            && ((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_START) ==> vdev.vdev_state == VDEV_STARTED)
            && ((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_MEAS) ==> vdev.attest_info.meas_nonce == VdevGenerateNonce(old_s, vdev_pre))
            && ((dev_comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_REPORT) ==> vdev.attest_info.report_nonce == VdevGenerateNonce(old_s, vdev_pre))
            && (dev_comm_result == DEV_COMM_IDLE ==> vdev.op == VDEV_OP_NONE)))
}