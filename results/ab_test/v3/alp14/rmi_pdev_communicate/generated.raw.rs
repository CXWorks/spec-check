```verus
pub open spec fn rmi_pdev_communicate_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S, pdev_ptr: Address, data_ptr: Address) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_state_pre = PdevAt(old_s, pdev_ptr).state;
    let data = RmiDevCommDataAt(old_s, data_ptr);
    let new_comm_state = DeviceCommunicate1(old_s, pdev);
    
    // Failure condition: da_supp
    (!(ImplFeatures().feat_da == FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    // Failure conditions: pdev_align, pdev_bound, pdev_gran_state, data_align, data_pas, req_align, req_pas, resp_align, resp_pas, resp_len
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(data.enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(data.enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (data.enter.resp_len > RMM_GRANULE_SIZE ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: comm_state
    && ((pdev.comm_state == DEV_COMM_IDLE || pdev.comm_state == DEV_COMM_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    // Success conditions
    && (result.is_Ok() ==> (
        PdevAt(new_s, pdev_ptr).comm_state == new_comm_state
        && (
            (new_comm_state == DEV_COMM_ERROR && pdev_state_pre != PDEV_STOPPING)
            ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR
        )
        && (
            (new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_NEW)
            ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEEDS_KEY
        )
        && (
            (new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_HAS_KEY)
            ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY
        )
        && (
            (new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_READY)
            ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY
        )
        && (
            (new_comm_state != DEV_COMM_ACTIVE && pdev_state_pre == PDEV_STOPPING)
            ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPED
        )
        && (
            (new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_COMMUNICATING)
            ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY
        )
        && (
            (new_comm_state == DEV_COMM_IDLE && pdev_state_pre == PDEV_IDE_RESETTING)
            ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY
        )
    ))
}
```