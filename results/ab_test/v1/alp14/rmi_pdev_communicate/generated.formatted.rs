pub open spec fn RMI_PDEV_COMMUNICATE_spec(
    s: S,
    pdev_ptr: Address,
    data_ptr: Address,
) -> (result: Result<(), RmiStatusCode>, s_post: S)
{
    // Failure conditions in order
    if !ImplFeatures(s).feat_da {
        (Err(RMI_ERROR_NOT_SUPPORTED), s)
    } else if !AddrIsGranuleAligned(pdev_ptr) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !PaIsDelegable(pdev_ptr) {
        (Err(RMI_ERROR_INPUT), s)
    } else if GranuleAt(s, pdev_ptr).state != GRANULE_STATE_PDEV {
        (Err(RMI_ERROR_INPUT), s)
    } else if !AddrIsGranuleAligned(data_ptr) {
        (Err(RMI_ERROR_INPUT), s)
    } else if !GranuleAccessPermitted(s, data_ptr, PAS_NS) {
        (Err(RMI_ERROR_INPUT), s)
    } else {
        let data = RmiDevCommDataAt(s, data_ptr);
        let pdev = PdevAt(s, pdev_ptr);
        let pdev_state_pre = pdev.state;

        if !AddrIsGranuleAligned(data.enter.req_addr) {
            (Err(RMI_ERROR_INPUT), s)
        } else if !GranuleAccessPermitted(s, data.enter.req_addr, PAS_NS) {
            (Err(RMI_ERROR_INPUT), s)
        } else if !AddrIsGranuleAligned(data.enter.resp_addr) {
            (Err(RMI_ERROR_INPUT), s)
        } else if !GranuleAccessPermitted(s, data.enter.resp_addr, PAS_NS) {
            (Err(RMI_ERROR_INPUT), s)
        } else if data.enter.resp_len > RMM_GRANULE_SIZE {
            (Err(RMI_ERROR_INPUT), s)
        } else if pdev.comm_state == DEV_COMM_IDLE || pdev.comm_state == DEV_COMM_ERROR {
            (Err(RMI_ERROR_DEVICE), s)
        } else {
            // Success path
            let new_comm_state = DeviceCommunicate1(s, pdev);
            
            let new_pdev_state = if new_comm_state == DEV_COMM_ERROR && pdev_state_pre != PDEV_STOPPING {
                PDEV_ERROR
            } else if new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_NEW {
                PDEV_NEEDS_KEY
            } else if new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_HAS_KEY {
                PDEV_READY
            } else if new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_READY {
                PDEV_READY
            } else if new_comm_state != DEV_COMM_ACTIVE && pdev_state_pre == PDEV_STOPPING {
                PDEV_STOPPED
            } else if new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_COMMUNICATING {
                PDEV_READY
            } else if new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_IDE_RESETTING {
                PDEV_READY
            } else {
                pdev_state_pre
            };

            let pdev_post = RmmPdev {
                state: new_pdev_state,
                comm_state: new_comm_state,
                ..pdev
            };

            let s_post = s.(pdev := pdev_post);
            
            (Ok(()), s_post)
        }
    }
}