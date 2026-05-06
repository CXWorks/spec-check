```verus
pub open spec fn RMI_PDEV_IDE_RESET_spec(old_s: S, new_s: S, pdev_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    
    // Failure condition: da_supp
    ((!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE()) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED())) &&
    
    // Failure conditions ordered after da_supp
    (ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> (
        // Failure condition: pdev_align
        ((!AddrIsGranuleAligned(old_s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        
        // Failure condition: pdev_bound
        ((!PaIsDelegable(old_s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        
        // Failure condition: pdev_gran_state
        ((GranuleAt(old_s, pdev_ptr).state != PDEV()) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        
        // Ordered after pdev_gran_state
        ((GranuleAt(old_s, pdev_ptr).state == PDEV()) ==> (
            // Failure condition: pdev_conn
            ((!pdev.ncoh_ide.is_IDE_TRUE()) ==> ResultEqual(result, RMI_ERROR_DEVICE())) &&
            
            // Failure condition: pdev_state
            ((pdev.state != PDEV_READY()) ==> ResultEqual(result, RMI_ERROR_DEVICE())) &&
            
            // Success conditions
            ((pdev.ncoh_ide.is_IDE_TRUE() && pdev.state == PDEV_READY()) ==> (
                result.is_Ok() &&
                PdevAt(new_s, pdev_ptr).state == PDEV_IDE_RESETTING() &&
                PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING()
            ))
        ))
    ))
}
```