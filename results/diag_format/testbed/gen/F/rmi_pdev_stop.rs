pub open spec fn rmi_pdev_stop_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!ImplFeatures(old_s).feat_da == RmmFeature::FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && ((pdev_ptr as int) % (RmmGranuleSize(old_s) as int) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAt(old_s, pdev_ptr).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(old_s, pdev_ptr).num_vdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING)
    && (PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
}