pub open spec fn RMI_PDEV_ABORT_spec(old_s: S, new_s: S, pdev_ptr: Address, result: RmiCommandReturnCode) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_state_pre = pdev.state;
    
    ((!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((pdev_state_pre != PDEV_NEW && pdev_state_pre != PDEV_HAS_KEY && pdev_state_pre != PDEV_COMMUNICATING) ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    ((ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV && (pdev_state_pre == PDEV_NEW || pdev_state_pre == PDEV_HAS_KEY || pdev_state_pre == PDEV_COMMUNICATING)) ==> 
        ((pdev_state_pre == PDEV_COMMUNICATING ==> (PdevAt(new_s, pdev_ptr).state == PDEV_READY && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_IDLE)) &&
         (pdev_state_pre != PDEV_COMMUNICATING ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))))
}