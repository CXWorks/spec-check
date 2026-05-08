```verus
pub open spec fn RMI_VDEV_COMMUNICATE_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    vdev_ptr: Address,
    data_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let vdev_new = VdevAt(new_s, vdev_ptr);
    let data = RmiDevCommDataAt(old_s, data_ptr);
    let dev_comm_new_state = DeviceCommunicate2(old_s, vdev_pre, data);
    
    // Failure conditions (ordered by precedence)
    (
        // da_supp: Feature DA not supported
        (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
        (ImplFeatures(old_s).feat_da ==> (
            // pdev_align
            (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
            &&
            (AddrIsGranuleAligned(pdev_ptr) ==> (
                // pdev_bound
                (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
                &&
                (PaIsDelegable(pdev_ptr) ==> (
                    // pdev_gran_state
                    (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
                    &&
                    (GranuleAt(old_s, pdev_ptr).state == PDEV ==> (
                        // vdev_align
                        (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
                        &&
                        (AddrIsGranuleAligned(vdev_ptr) ==> (
                            // vdev_bound
                            (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
                            &&
                            (PaIsDelegable(vdev_ptr) ==> (
                                // vdev_gran_state
                                (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
                                &&
                                (GranuleAt(old_s, vdev_ptr).state == VDEV ==> (
                                    // data_align
                                    (!AddrIsGranuleAligned(data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
                                    &&
                                    (AddrIsGranuleAligned(data_ptr) ==> (
                                        // data_pas
                                        (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
                                        &&
                                        (GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> (
                                            // req_align
                                            (!AddrIsGranuleAligned(data.enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
                                            &&
                                            (AddrIsGranuleAligned(data.enter.req_addr) ==> (
                                                // req_pas
                                                (!GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
                                                &&
                                                (GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS) ==> (
                                                    // resp_align
                                                    (!AddrIsGranuleAligned(data.enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
                                                    &&
                                                    (AddrIsGranuleAligned(data.enter.resp_addr) ==> (
                                                        // resp_pas
                                                        (!GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
                                                        &&
                                                        (GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS) ==> (
                                                            // resp_len
                                                            (data.enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
                                                            &&
                                                            (data.enter.resp_len <= RMM_GRANULE_SIZE ==> (
                                                                // vdev_pdev
                                                                (vdev_pre.pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
                                                                &&
                                                                (vdev_pre.pdev == pdev_ptr ==> (
                                                                    // comm_state
                                                                    (vdev_pre.comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
                                                                    &&
                                                                    (vdev_pre.comm_state != DEV_COMM_IDLE ==> (
                                                                        // Success conditions
                                                                        (vdev_new.comm_state == dev_comm_new_state)
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_ERROR ==> vdev_new.vdev_state == VDEV_ERROR)
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_UNLOCK ==> vdev_new.vdev_state == VDEV_UNLOCKED)
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK ==> vdev_new.vdev_state == VDEV_LOCKED)
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_LOCK ==> vdev_new.attest_info.lock_nonce == VdevGenerateNonce(old_s, vdev_pre))
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_START ==> vdev_new.vdev_state == VDEV_STARTED)
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_MEAS ==> vdev_new.attest_info.meas_nonce == VdevGenerateNonce(old_s, vdev_pre))
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_IDLE && vdev_pre.op == VDEV_OP_GET_REPORT ==> vdev_new.attest_info.report_nonce == VdevGenerateNonce(old_s, vdev_pre))
                                                                        &&
                                                                        (dev_comm_new_state == DEV_COMM_IDLE ==> vdev_new.op == VDEV_OP_NONE)
                                                                        &&
                                                                        result.is_Ok()
                                                                    ))
                                                                ))
                                                            ))
                                                        ))
                                                    ))
                                                ))
                                            ))
                                        ))
                                    ))
                                ))
                            ))