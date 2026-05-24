pub open spec fn rmi_pdev_stop_spec(result: RmiCommandReturnCode, pdev_ptr: Address, old_s: S, new_s: S) -> bool {
    // Failure: DA not supported
    ((!ImplFeatures(old_s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && // Failure: pdev_ptr not granule aligned
    ((!AddrIsGranuleAligned(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && // Failure: pdev_ptr not delegable
    ((!PaIsDelegable(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && // Failure: granule at pdev_ptr is not PDEV state
    ((GranuleAt(old_s, pdev_ptr).state != PDEV) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && // Failure: pdev in communicating/stopping/stopped state
    (((PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING
        || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING
        || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED)) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && // Failure: pdev has non-zero num_vdevs
    ((PdevAt(old_s, pdev_ptr).num_vdevs != 0) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && // Success: all preconditions met
    ((ImplFeatures(old_s).feat_da
        && AddrIsGranuleAligned(pdev_ptr)
        && PaIsDelegable(pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && PdevAt(old_s, pdev_ptr).state != PDEV_COMMUNICATING
        && PdevAt(old_s, pdev_ptr).state != PDEV_STOPPING
        && PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED
        && PdevAt(old_s, pdev_ptr).num_vdevs == 0)
      ==> (result.is_Ok()
        && PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING
        && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
}