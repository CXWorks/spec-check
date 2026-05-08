pub open spec fn RMI_PDEV_IDE_RESET_spec(s: S, pdev_ptr: Address) -> Result<(), RmiStatusCode> {
    if !ImplFeatures(s).feat_da.is_FEATURE_TRUE() {
        Err(RMI_ERROR_NOT_SUPPORTED)
    } else if !AddrIsGranuleAligned(pdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(pdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, pdev_ptr).state != PDEV {
        Err(RMI_ERROR_INPUT)
    } else {
        let pdev = PdevAt(s, pdev_ptr);
        if pdev.ncoh_ide != IDE_TRUE {
            Err(RMI_ERROR_DEVICE)
        } else if pdev.state != PDEV_READY {
            Err(RMI_ERROR_DEVICE)
        } else {
            Ok(())
        }
    }
}