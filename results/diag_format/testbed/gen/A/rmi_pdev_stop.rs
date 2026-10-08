pub open spec fn rmi_pdev_stop_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
         && (!AddrIsGranuleAligned(old_s, pdev_ptr)
             || !PaIsDelegable(old_s, pdev_ptr)
             || GranuleAt(old_s, pdev_ptr).state != PDEV))
        ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
         && AddrIsGranuleAligned(old_s, pdev_ptr)
         && PaIsDelegable(old_s, pdev_ptr)
         && GranuleAt(old_s, pdev_ptr).state == PDEV
         && (PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING
             || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING
             || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED
             || PdevAt(old_s, pdev_ptr).num_vdevs != 0))
        ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
         && AddrIsGranuleAligned(old_s, pdev_ptr)
         && PaIsDelegable(old_s, pdev_ptr)
         && GranuleAt(old_s, pdev_ptr).state == PDEV
         && PdevAt(old_s, pdev_ptr).state != PDEV_COMMUNICATING
         && PdevAt(old_s, pdev_ptr).state != PDEV_STOPPING
         && PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED
         && PdevAt(old_s, pdev_ptr).num_vdevs == 0)
        ==> (result.is_Ok()
             && PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING
             && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
    && (result.is_Err()
        ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state
             && PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state))
}
