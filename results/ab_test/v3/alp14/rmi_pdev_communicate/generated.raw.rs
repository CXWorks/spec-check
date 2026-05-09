pub open spec fn rmi_pdev_communicate_spec(result: Result<(), RmiStatusCode>, pdev_ptr: Address, data_ptr: Address, old_s: S, new_s: S) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_state_pre = PdevAt(old_s, pdev_ptr).state;
    let data = RmiDevCommDataAt(old_s, data_ptr);
    let comm_result = DeviceCommunicate1(old_s, pdev);
    
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
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
    && ((pdev.comm_state == DEV_COMM_IDLE || pdev.comm_state == DEV_COMM_ERROR) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(pdev_ptr)
        && PaIsDelegable(pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && AddrIsGranuleAligned(data_ptr)
        && GranuleAccessPermitted(old_s, data_ptr, PAS_NS)
        && AddrIsGranuleAligned(data.enter.req_addr)
        && GranuleAccessPermitted(old_s, data.enter.req_addr, PAS_NS)
        && AddrIsGranuleAligned(data.enter.resp_addr)
        && GranuleAccessPermitted(old_s, data.enter.resp_addr, PAS_NS)
        && data.enter.resp_len <= RMM_GRANULE_SIZE
        && pdev.comm_state != DEV_COMM_IDLE && pdev.comm_state != DEV_COMM_ERROR)
        ==> (result.is_Ok()
            && PdevAt(new_s, pdev_ptr).comm_state == comm_result
            && ((comm_result == DEV_COMM_ERROR && pdev_state_pre != PDEV_STOPPING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR)
            && ((comm_result == DEV_COMM_IDLE && pdev_state_pre == PDEV_NEW) ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEEDS_KEY)
            && ((comm_result == DEV_COMM_IDLE && pdev_state_pre == PDEV_HAS_KEY) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
            && ((comm_result == DEV_COMM_IDLE && pdev_state_pre == PDEV_READY) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
            && ((comm_result != DEV_COMM_ACTIVE && pdev_state_pre == PDEV_STOPPING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPED)
            && ((comm_result == DEV_COMM_IDLE && pdev_state_pre == PDEV_COMMUNICATING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
            && ((comm_result == DEV_COMM_IDLE && pdev_state_pre == PDEV_IDE_RESETTING) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)))
}