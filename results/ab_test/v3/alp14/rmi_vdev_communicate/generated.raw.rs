pub open spec fn rmi_vdev_communicate_spec(result: RmiCommandReturnCode, pdev_ptr: Address, vdev_ptr: Address, data_ptr: Address, old_s: S, new_s: S) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let vdev = VdevAt(new_s, vdev_ptr);
    let data = RmiDevCommDataAt(old_s, data_ptr);
    let comm_result = DeviceCommunicate1(old_s, vdev_pre);
    
    (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, data.enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, data.enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (data.enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_pre.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev_pre.comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (ImplFeatures(old_s).feat_da && AddrIsGranuleAligned(old_s, pdev_ptr) && PaIsDelegable(old_s, pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV && AddrIsGranuleAligned(old_s, vdev_ptr) && PaIsDelegable(old_s, vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV && AddrIsGranuleAligned(old_s, data_ptr) && GranuleAccessPermitted(old_s, data_ptr, PAS_NS) && AddrIsGranuleAligned(old_s, data.enter.req_addr) && GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS) && AddrIsGranuleAligned(old_s, data.enter.resp_addr) && GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS) && data.enter.resp_len <= RMM_GRANULE_SIZE && vdev_pre.pdev == pdev_ptr && vdev_pre.comm_state != DEV_COMM_IDLE
        ==> (result == RMI_SUCCESS
            && vdev.comm_state == comm_result
            && (comm_result == DEV_COMM_ERROR ==> vdev.vdev_state == VDEV_ERROR)
            && ((comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_UNLOCK) ==> vdev.vdev_state == VDEV_UNLOCKED)
            && ((comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK) ==> vdev.vdev_state == VDEV_LOCKED)
            && ((comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK) ==> vdev.attest_info.lock_nonce == VdevGenerateNonce(old_s, vdev_pre))
            && ((comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_START) ==> vdev.vdev_state == VDEV_STARTED)
            && ((comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_MEAS) ==> vdev.attest_info.meas_nonce == VdevGenerateNonce(old_s, vdev_pre))
            && ((comm_result == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_REPORT) ==> vdev.attest_info.report_nonce == VdevGenerateNonce(old_s, vdev_pre))
            && (comm_result == DEV_COMM_IDLE ==> vdev.op == VDEV_OP_NONE)))
}