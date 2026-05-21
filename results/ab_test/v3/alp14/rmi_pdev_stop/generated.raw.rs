pub open spec fn rmi_pdev_stop_spec(result: RmiCommandReturnCode, pdev_ptr: Address, old_s: S, new_s: S) -> bool {
    (!(ImplFeatures().feat_da == FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING
            || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING
            || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(old_s, pdev_ptr).num_vdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures().feat_da == FEATURE_TRUE
            && AddrIsGranuleAligned(pdev_ptr)
            && PaIsDelegable(pdev_ptr)
            && GranuleAt(old_s, pdev_ptr).state == PDEV
            && PdevAt(old_s, pdev_ptr).state != PDEV_COMMUNICATING
            && PdevAt(old_s, pdev_ptr).state != PDEV_STOPPING
            && PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED
            && PdevAt(old_s, pdev_ptr).num_vdevs == 0)
        ==> (result.is_Ok() && PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
}