pub open spec fn RMI_PDEV_COMMUNICATE_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    data_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_state_pre = PdevAt(old_s, pdev_ptr).state;
    let data = RmiDevCommDataAt(old_s, data_ptr);
    let pdev_new = PdevAt(new_s, pdev_ptr);
    let dev_comm_result = DeviceCommunicate1(old_s, pdev);

    ((!ImplFeatures(old_s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && ((
    !AddrIsGranuleAligned(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PaIsDelegable(
        pdev_ptr,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(old_s, pdev_ptr).state != PDEV)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(data_ptr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!GranuleAccessPermitted(
        old_s,
        data_ptr,
        PAS_NS,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(data.enter.req_addr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!GranuleAccessPermitted(
        old_s,
        data.enter.req_addr,
        PAS_NS,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(data.enter.resp_addr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!GranuleAccessPermitted(
        old_s,
        data.enter.resp_addr,
        PAS_NS,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((data.enter.resp_len > RMM_GRANULE_SIZE)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((pdev.comm_state == DEV_COMM_IDLE
        || pdev.comm_state == DEV_COMM_ERROR)) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (
    result.is_Ok() ==> (pdev_new.comm_state == DeviceCommunicate2(old_s, pdev, data) && ((
    dev_comm_result == DEV_COMM_ERROR && pdev_state_pre != PDEV_STOPPING) ==> pdev_new.state
        == PDEV_ERROR) && ((dev_comm_result == DEV_COMM_IDLE && pdev_state_pre == PDEV_NEW)
        ==> pdev_new.state == PDEV_NEEDS_KEY) && ((dev_comm_result == DEV_COMM_IDLE
        && pdev_state_pre == PDEV_HAS_KEY) ==> pdev_new.state == PDEV_READY) && ((dev_comm_result
        == DEV_COMM_IDLE && pdev_state_pre == PDEV_READY) ==> pdev_new.state == PDEV_READY) && ((
    dev_comm_result != DEV_COMM_ACTIVE && pdev_state_pre == PDEV_STOPPING) ==> pdev_new.state
        == PDEV_STOPPED) && ((dev_comm_result == DEV_COMM_IDLE && pdev_state_pre
        == PDEV_COMMUNICATING) ==> pdev_new.state == PDEV_READY) && ((dev_comm_result
        == DEV_COMM_IDLE && pdev_state_pre == PDEV_IDE_RESETTING) ==> pdev_new.state
        == PDEV_READY)))
}