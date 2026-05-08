pub open spec fn rmi_pdev_ide_reset_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);

    // Failure conditions
    let da_supp_fail = ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> result
        == RMI_ERROR_NOT_SUPPORTED;

    let pdev_align_fail = !AddrIsGranuleAligned(old_s, pdev_ptr) ==> result == RMI_ERROR_INPUT;
    let pdev_bound_fail = !PaIsDelegable(old_s, pdev_ptr) ==> result == RMI_ERROR_INPUT;
    let pdev_gran_state_fail = GranuleAt(old_s, pdev_ptr).state != PDEV ==> result
        == RMI_ERROR_INPUT;

    let pdev_conn_fail = pdev.ncoh_ide != IDE_TRUE ==> result == RMI_ERROR_DEVICE;
    let pdev_state_fail = pdev.state != PDEV_READY ==> result == RMI_ERROR_DEVICE;

    // Success condition
    let success = (ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(
        old_s,
        pdev_ptr,
    ) && PaIsDelegable(old_s, pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV && pdev.ncoh_ide
        == IDE_TRUE && pdev.state == PDEV_READY) ==> (result == RMI_OK && PdevAt(
        new_s,
        pdev_ptr,
    ).state == PDEV_IDE_RESETTING && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING);

    da_supp_fail && pdev_align_fail && pdev_bound_fail && pdev_gran_state_fail && pdev_conn_fail
        && pdev_state_fail && success
}