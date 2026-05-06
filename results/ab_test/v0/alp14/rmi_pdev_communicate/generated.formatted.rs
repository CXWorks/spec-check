pub open spec fn RMI_PDEV_COMMUNICATE_spec(
    s: S,
    pdev_ptr: Address,
    data_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let pdev_state_pre = PdevAt(s, pdev_ptr).state;
    let data = RmiDevCommDataAt(s, data_ptr);
    let comm_state_new = DeviceCommunicate1(s, pdev);

    // Failure condition: da_supp
    let cond_da_supp = !ImplFeatures(s).feat_da && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED);

    // Failure condition: pdev_align
    let cond_pdev_align = !AddrIsGranuleAligned(s, pdev_ptr) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Failure condition: pdev_bound
    let cond_pdev_bound = !PaIsDelegable(s, pdev_ptr) && ResultEqual(result, RMI_ERROR_INPUT);

    // Failure condition: pdev_gran_state
    let cond_pdev_gran_state = GranuleAt(s, pdev_ptr).state != RmmGranuleState::PDEV && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Failure condition: data_align
    let cond_data_align = !AddrIsGranuleAligned(s, data_ptr) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Failure condition: data_pas
    let cond_data_pas = !GranuleAccessPermitted(s, data_ptr, RmmPas::PAS_NS) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Failure condition: req_align
    let cond_req_align = !AddrIsGranuleAligned(s, data.enter.req_addr) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Failure condition: req_pas
    let cond_req_pas = !GranuleAccessPermitted(s, data.enter.req_addr, RmmPas::PAS_NS)
        && ResultEqual(result, RMI_ERROR_INPUT);

    // Failure condition: resp_align
    let cond_resp_align = !AddrIsGranuleAligned(s, data.enter.resp_addr) && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Failure condition: resp_pas
    let cond_resp_pas = !GranuleAccessPermitted(s, data.enter.resp_addr, RmmPas::PAS_NS)
        && ResultEqual(result, RMI_ERROR_INPUT);

    // Failure condition: resp_len
    let cond_resp_len = data.enter.resp_len > RMM_GRANULE_SIZE && ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );

    // Failure condition: comm_state
    let cond_comm_state = (pdev.comm_state == RmmDevCommState::DEV_COMM_IDLE || pdev.comm_state
        == RmmDevCommState::DEV_COMM_ERROR) && ResultEqual(result, RMI_ERROR_DEVICE);

    // Success condition: comm_state updated
    let cond_success_comm_state = pdev.comm_state == comm_state_new;

    // Success condition: error
    let cond_success_error = (comm_state_new == RmmDevCommState::DEV_COMM_ERROR && pdev_state_pre
        != RmmPdevState::PDEV_STOPPING) ==> (pdev.state == RmmPdevState::PDEV_ERROR);

    // Success condition: new
    let cond_success_new = (comm_state_new == RmmDevCommState::DEV_COMM_IDLE && pdev_state_pre
        == RmmPdevState::PDEV_NEW) ==> (pdev.state == RmmPdevState::PDEV_NEEDS_KEY);

    // Success condition: has_key
    let cond_success_has_key = (comm_state_new == RmmDevCommState::DEV_COMM_IDLE && pdev_state_pre
        == RmmPdevState::PDEV_HAS_KEY) ==> (pdev.state == RmmPdevState::PDEV_READY);

    // Success condition: ready
    let cond_success_ready = (comm_state_new == RmmDevCommState::DEV_COMM_IDLE && pdev_state_pre
        == RmmPdevState::PDEV_READY) ==> (pdev.state == RmmPdevState::PDEV_READY);

    // Success condition: stopped
    let cond_success_stopped = (comm_state_new != RmmDevCommState::DEV_COMM_ACTIVE && pdev_state_pre
        == RmmPdevState::PDEV_STOPPING) ==> (pdev.state == RmmPdevState::PDEV_STOPPED);

    // Success condition: communicating
    let cond_success_communicating = (comm_state_new == RmmDevCommState::DEV_COMM_IDLE
        && pdev_state_pre == RmmPdevState::PDEV_COMMUNICATING) ==> (pdev.state
        == RmmPdevState::PDEV_READY);

    // Success condition: ide_resetting
    let cond_success_ide_resetting = (comm_state_new == RmmDevCommState::DEV_COMM_IDLE
        && pdev_state_pre == RmmPdevState::PDEV_IDE_RESETTING) ==> (pdev.state
        == RmmPdevState::PDEV_READY);

    // Failure conditions with ordering
    let failure_path = cond_da_supp || cond_pdev_align || cond_pdev_bound || cond_pdev_gran_state
        || cond_data_align || cond_data_pas || cond_req_align || cond_req_pas || cond_resp_align
        || cond_resp_pas || cond_resp_len || cond_comm_state;

    // Success path
    let success_path = result.is_Ok() && cond_success_comm_state && cond_success_error
        && cond_success_new && cond_success_has_key && cond_success_ready && cond_success_stopped
        && cond_success_communicating && cond_success_ide_resetting;

    failure_path || success_path
}