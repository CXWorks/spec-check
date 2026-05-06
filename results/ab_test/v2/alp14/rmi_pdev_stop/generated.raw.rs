```verus
pub open spec fn rmi_pdev_stop_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, pdev_ptr: Address) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    
    // Failure conditions with ordering
    ((!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> result.is_RmiError() && result.get_RmiError_code() == RMI_ERROR_NOT_SUPPORTED)
    && (!AddrIsGranuleAligned(pdev_ptr) ==> result.is_RmiError() && result.get_RmiError_code() == RMI_ERROR_INPUT)
    && (!PaIsDelegable(pdev_ptr) ==> result.is_RmiError() && result.get_RmiError_code() == RMI_ERROR_INPUT)
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> result.is_RmiError() && result.get_RmiError_code() == RMI_ERROR_INPUT)
    && ((pdev.state == PDEV_COMMUNICATING || pdev.state == PDEV_STOPPING || pdev.state == PDEV_STOPPED) 
        ==> result.is_RmiError() && result.get_RmiError_code() == RMI_ERROR_DEVICE)
    && (pdev.num_vdevs != 0 ==> result.is_RmiError() && result.get_RmiError_code() == RMI_ERROR_DEVICE))
    
    // Success conditions
    && (ImplFeatures(old_s).feat_da.is_FEATURE_TRUE()
        && AddrIsGranuleAligned(pdev_ptr)
        && PaIsDelegable(pdev_ptr)
        && GranuleAt(old_s, pdev_ptr).state == PDEV
        && pdev.state != PDEV_COMMUNICATING
        && pdev.state != PDEV_STOPPING
        && pdev.state != PDEV_STOPPED
        && pdev.num_vdevs == 0
        ==> result.is_ok()
            && PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING
            && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
}
```