pub open spec fn rmi_pdev_abort_spec(result: RmiCommandReturnCode, old_s: S, new_s: S) -> bool {
    let pdev_ptr = old_s.cmd_input_x1;
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_state_pre = pdev.state;
    
    ((!ImplFeatures(old_s).feat_da.eq_true()) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((pdev_state_pre != PDEV_NEW && pdev_state_pre != PDEV_HAS_KEY && pdev_state_pre != PDEV_COMMUNICATING) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures(old_s).feat_da.eq_true() && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV && (pdev_state_pre == PDEV_NEW || pdev_state_pre == PDEV_HAS_KEY || pdev_state_pre == PDEV_COMMUNICATING))
        ==> (result.is_Ok() && ((pdev_state_pre == PDEV_COMMUNICATING ==> (PdevAt(new_s, pdev_ptr).state == PDEV_READY && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_IDLE)) && (pdev_state_pre != PDEV_COMMUNICATING ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))))
}