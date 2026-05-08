```verus
pub open spec fn RMI_PDEV_ABORT_spec(s: S, pdev_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let pdev_state_pre = pdev.state;
    let pdev_gran = GranuleAt(s, pdev_ptr);
    
    // Failure condition: da_supp
    (ImplFeatures(s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    
    // Failure conditions: pdev_align, pdev_bound, pdev_gran_state
    (!AddrIsGranuleAligned(s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (pdev_gran.state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: pdev_state
    ((pdev_state_pre != PDEV_NEW && pdev_state_pre != PDEV_HAS_KEY && pdev_state_pre != PDEV_COMMUNICATING) ==> 
     ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Success conditions
    (result.is_Ok() ==> (
        // comm: if pdev was PDEV_COMMUNICATING, transition to PDEV_READY with DEV_COMM_IDLE
        (pdev_state_pre == PDEV_COMMUNICATING ==> 
         pdev.state == PDEV_READY && pdev.comm_state == DEV_COMM_IDLE) &&
        
        // not_comm: if pdev was not PDEV_COMMUNICATING, comm_state becomes DEV_COMM_PENDING
        (pdev_state_pre != PDEV_COMMUNICATING ==> 
         pdev.comm_state == DEV_COMM_PENDING)
    ))
}
```