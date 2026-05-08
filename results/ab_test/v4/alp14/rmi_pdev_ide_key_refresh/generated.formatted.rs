pub open spec fn RMI_PDEV_IDE_KEY_REFRESH_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    coh: u64,
    result: RmiCommandReturnCode,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);

    ((!ImplFeatures(old_s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && ((
    !AddrIsGranuleAligned(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PaIsDelegable(
        pdev_ptr,
    )) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(old_s, pdev_ptr).state != PDEV)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((coh == RMI_NCOH && pdev.ncoh_ide
        != IDE_TRUE) || (coh == RMI_COH && pdev.coh_ide != IDE_TRUE)) ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    )) && ((pdev.state != PDEV_READY) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (result.is_Ok()
        ==> (PdevAt(new_s, pdev_ptr).state == PDEV_COMMUNICATING && PdevAt(
        new_s,
        pdev_ptr,
    ).comm_state == DEV_COMM_PENDING))
}