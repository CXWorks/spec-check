pub open spec fn rmi_pdev_stop_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING || PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(old_s, pdev_ptr).num_vdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> (ImplFeatures(old_s).feat_da == FEATURE_FALSE))
    && (result.is_Ok() ==> (PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
}