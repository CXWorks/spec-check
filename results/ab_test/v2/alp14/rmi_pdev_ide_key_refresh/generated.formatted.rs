pub open spec fn rmi_pdev_ide_key_refresh_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    coh: RmiPdevCoherent,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);

    // Failure conditions in order
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        pdev_ptr,
    ).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((coh == RMI_NCOH && pdev.ncoh_ide
        != IDE_TRUE) || (coh == RMI_COH && pdev.coh_ide != IDE_TRUE)) ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    )) && (pdev.state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
        &&
    // Success conditions
    (ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV && ((coh
        == RMI_NCOH && pdev.ncoh_ide == IDE_TRUE) || (coh == RMI_COH && pdev.coh_ide == IDE_TRUE))
        && pdev.state == PDEV_READY ==> (PdevAt(new_s, pdev_ptr).state == PDEV_COMMUNICATING
        && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING && result == RMI_SUCCESS))
}