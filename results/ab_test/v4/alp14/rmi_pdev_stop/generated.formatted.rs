pub open spec fn RMI_PDEV_STOP_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let da_supp_fail = ImplFeatures(old_s).feat_da != FEATURE_TRUE;
    let pdev_align_fail = !AddrIsGranuleAligned(pdev_ptr);
    let pdev_bound_fail = !PaIsDelegable(pdev_ptr);
    let pdev_gran_state_fail = GranuleAt(old_s, pdev_ptr).state != PDEV;
    let pdev_state_fail = pdev.state == PDEV_COMMUNICATING || pdev.state == PDEV_STOPPING
        || pdev.state == PDEV_STOPPED;
    let num_vdevs_fail = pdev.num_vdevs != 0;

    (da_supp_fail ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && (!da_supp_fail
        && pdev_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!da_supp_fail
        && pdev_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!da_supp_fail
        && pdev_gran_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!da_supp_fail
        && !pdev_gran_state_fail && pdev_state_fail ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (
    !da_supp_fail && !pdev_gran_state_fail && num_vdevs_fail ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    )) && (!da_supp_fail && !pdev_align_fail && !pdev_bound_fail && !pdev_gran_state_fail
        && !pdev_state_fail && !num_vdevs_fail ==> (result.is_Ok() && PdevAt(new_s, pdev_ptr).state
        == PDEV_STOPPING && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
}