```verus
pub open spec fn RMI_PDEV_COMMUNICATE_spec(
    s: S,
    pdev_ptr: Address,
    data_ptr: Address,
    result: RmiCommandReturnCode,
    new_s: S,
) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let pdev_state_pre = PdevAt(s, pdev_ptr).state;
    let data = RmiDevCommDataAt(s, data_ptr);
    let new_pdev = PdevAt(new_s, pdev_ptr);
    
    // Failure conditions in precedence order
    (!ImplFeatures(s).feat_da.is_true() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    &&
    (ImplFeatures(s).feat_da.is_true() ==> (
        (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (GranuleAt(s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (!AddrIsGranuleAligned(data_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (!GranuleAccessPermitted(s, data_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (!AddrIsGranuleAligned(data.enter.req_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (!GranuleAccessPermitted(s, data.enter.req_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (!AddrIsGranuleAligned(data.enter.resp_addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (!GranuleAccessPermitted(s, data.enter.resp_addr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        (data.enter.resp_len > RMM_GRANULE_SIZE() ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
        ((pdev.comm_state == DEV_COMM_IDLE || pdev.comm_state == DEV_COMM_ERROR) ==> 
            ResultEqual(result, RMI_ERROR_DEVICE))
        &&
        // Success conditions
        (((pdev.comm_state != DEV_COMM_IDLE && pdev.comm_state != DEV_COMM_ERROR) && 
          result.is_Ok()) ==> (
            new_pdev.comm_state == DeviceCommunicate1(s, pdev)
            &&
            ((DeviceCommunicate1(s, pdev) == DEV_COMM_ERROR && pdev_state_pre != PDEV_STOPPING) ==> 
                new_pdev.state == PDEV_ERROR)
            &&
            ((DeviceCommunicate1(s, pdev) == DEV_COMM_IDLE && pdev_state_pre == PDEV_NEW) ==> 
                new_pdev.state == PDEV_NEEDS_KEY)
            &&
            ((DeviceCommunicate1(s, pdev) == DEV_COMM_IDLE && pdev_state_pre == PDEV_HAS_KEY) ==> 
                new_pdev.state == PDEV_READY)
            &&
            ((DeviceCommunicate1(s, pdev) == DEV_COMM_IDLE && pdev_state_pre == PDEV_READY) ==> 
                new_pdev.state == PDEV_READY)
            &&
            ((DeviceCommunicate1(s, pdev) != DEV_COMM_ACTIVE && pdev_state_pre == PDEV_STOPPING) ==> 
                new_pdev.state == PDEV_STOPPED)
            &&
            ((DeviceCommunicate1(s, pdev) == DEV_COMM_IDLE && pdev_state_pre == PDEV_COMMUNICATING) ==> 
                new_pdev.state == PDEV_READY)
            &&
            ((DeviceCommunicate1(s, pdev) == DEV_COMM_IDLE && pdev_state_pre == PDEV_IDE_RESETTING) ==> 
                new_pdev.state == PDEV_READY)
        ))
    ))
}
```