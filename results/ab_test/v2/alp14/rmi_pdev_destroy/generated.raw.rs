```verus
pub open spec fn rmi_pdev_destroy_spec(result: RmiCommandReturnCode, old_s: S, new_s: S, pdev_ptr: Address) -> bool {
    // Failure condition: da_supp
    ((!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE()) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    // Failure condition: pdev_align
    && ((!AddrIsGranuleAligned(old_s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: pdev_bound
    && ((!PaIsDelegable(old_s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: pdev_gran_state
    && ((GranuleAt(old_s, pdev_ptr).state != PDEV) ==> ResultEqual(result, RMI_ERROR_INPUT))
    // Failure condition: pdev_state
    && ((PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    // Failure condition: pdev_p2p
    && ((PdevAt(old_s, pdev_ptr).p2p_stream_valid == RMM_TRUE) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    // Success conditions
    && ((ImplFeatures(old_s).feat_da.is_FEATURE_TRUE()
         && AddrIsGranuleAligned(old_s, pdev_ptr)
         && PaIsDelegable(old_s, pdev_ptr)
         && GranuleAt(old_s, pdev_ptr).state == PDEV
         && PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED
         && PdevAt(old_s, pdev_ptr).p2p_stream_valid != RMM_TRUE)
        ==> (result.is_Ok()
             && GranuleAt(new_s, pdev_ptr).state == DELEGATED
             && AuxStateEqual32(old_s, PdevAt(old_s, pdev_ptr).aux, PdevAt(old_s, pdev_ptr).num_aux, DELEGATED)))
}
```