```verus
pub open spec fn RMI_PDEV_ABORT_spec(s: S, pdev_ptr: Address) -> (result: RmiCommandReturnCode) {
    let pdev = PdevAt(s, pdev_ptr);
    let pdev_state_pre = pdev.state;
    
    // Failure conditions (ordered)
    if !ImplFeatures(s).feat_da.is_FEATURE_TRUE() {
        // da_supp
        result == RMI_ERROR_NOT_SUPPORTED
    } else if !AddrIsGranuleAligned(pdev_ptr) {
        // pdev_align
        result == RMI_ERROR_INPUT
    } else if !PaIsDelegable(pdev_ptr) {
        // pdev_bound
        result == RMI_ERROR_INPUT
    } else if GranuleAt(s, pdev_ptr).state != PDEV {
        // pdev_gran_state
        result == RMI_ERROR_INPUT
    } else if pdev_state_pre != PDEV_NEW && pdev_state_pre != PDEV_HAS_KEY && pdev_state_pre != PDEV_COMMUNICATING {
        // pdev_state
        result == RMI_ERROR_DEVICE
    }
    // Success conditions
    else if pdev_state_pre == PDEV_COMMUNICATING {
        // comm
        result == RMI_SUCCESS &&
        PdevAt(s, pdev_ptr).state == PDEV_READY &&
        PdevAt(s, pdev_ptr).comm_state == DEV_COMM_IDLE
    } else {
        // not_comm
        result == RMI_SUCCESS &&
        PdevAt(s, pdev_ptr).comm_state == DEV_COMM_PENDING
    }
}
```