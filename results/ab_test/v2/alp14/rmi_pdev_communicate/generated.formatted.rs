pub open spec fn rmi_pdev_communicate_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    data_ptr: Address,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_state_pre = PdevAt(old_s, pdev_ptr).state;
    let data = RmiDevCommDataAt(old_s, data_ptr);
    let comm_state_new = DeviceCommunicate1(old_s, pdev);

    // Failure conditions
    (ImplFeatures().feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && (
    !AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        pdev_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, pdev_ptr).state != PDEV
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(data_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(
        old_s,
        data_ptr,
        PAS_NS,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(data.enter.req_addr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(
        old_s,
        data.enter.req_addr,
        PAS_NS,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(data.enter.resp_addr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(
        old_s,
        data.enter.resp_addr,
        PAS_NS,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (data.enter.resp_len > RMM_GRANULE_SIZE
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((pdev.comm_state == DEV_COMM_IDLE
        || pdev.comm_state == DEV_COMM_ERROR) ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    ))
    // Success conditions
     && (result.is_Ok() ==> (
    // comm_state
    PdevAt(new_s, pdev_ptr).comm_state == comm_state_new
    // error
     && ((comm_state_new == DEV_COMM_ERROR && pdev_state_pre != PDEV_STOPPING) ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_ERROR)
    // new
     && ((comm_state_new == DEV_COMM_IDLE && pdev_state_pre == PDEV_NEW) ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_NEEDS_KEY)
    // has_key
     && ((comm_state_new == DEV_COMM_IDLE && pdev_state_pre == PDEV_HAS_KEY) ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_READY)
    // ready
     && ((comm_state_new == DEV_COMM_IDLE && pdev_state_pre == PDEV_READY) ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_READY)
    // stopped
     && ((comm_state_new != DEV_COMM_ACTIVE && pdev_state_pre == PDEV_STOPPING) ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_STOPPED)
    // communicating
     && ((comm_state_new == DEV_COMM_IDLE && pdev_state_pre == PDEV_COMMUNICATING) ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_READY)
    // ide_resetting
     && ((comm_state_new == DEV_COMM_IDLE && pdev_state_pre == PDEV_IDE_RESETTING) ==> PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_READY)))
}