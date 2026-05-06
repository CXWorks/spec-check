```verus
pub open spec fn RMI_PDEV_DESTROY_spec(old_s: S, new_s: S, pdev_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let pdev_pre = PdevAt(old_s, pdev_ptr);
    
    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    
    // Failure condition: pdev_align
    (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: pdev_bound
    (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: pdev_gran_state
    (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: pdev_state
    (pdev_pre.state != PDEV_STOPPED ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Failure condition: pdev_p2p
    (pdev_pre.p2p_stream_valid == RMM_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    // Success conditions (when no failures occur)
    (result.is_Ok() ==> (
        GranuleAt(new_s, pdev_ptr).state == DELEGATED &&
        AuxStateEqual32(old_s, new_s, pdev_pre.aux, pdev_pre.num_aux, DELEGATED)
    ))
}
```