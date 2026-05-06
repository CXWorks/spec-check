```rust
pub open spec fn RMI_PDEV_DESTROY_spec(s: S, pdev_ptr: Address) -> (result: RmiCommandReturnCode, s_post: S)
    requires
        true
    ensures
        // Failure condition: da_supp
        (!ImplFeatures(s).feat_da.eq(FEATURE_TRUE) ==> result.eq(RMI_ERROR_NOT_SUPPORTED)) &&
        // Failure condition: pdev_align
        (!AddrIsGranuleAligned(pdev_ptr) ==> result.eq(RMI_ERROR_INPUT)) &&
        // Failure condition: pdev_bound
        (!PaIsDelegable(pdev_ptr) ==> result.eq(RMI_ERROR_INPUT)) &&
        // Failure condition: pdev_gran_state
        (GranuleAt(s, pdev_ptr).state != PDEV ==> result.eq(RMI_ERROR_INPUT)) &&
        // Failure condition: pdev_state
        (PdevAt(s, pdev_ptr).state != PDEV_STOPPED ==> result.eq(RMI_ERROR_DEVICE)) &&
        // Failure condition: pdev_p2p
        (PdevAt(s, pdev_ptr).p2p_stream_valid == RMM_TRUE ==> result.eq(RMI_ERROR_DEVICE)) &&
        // Success condition: gran_state
        (result.eq(RMI_SUCCESS) ==> GranuleAt(s_post, pdev_ptr).state == DELEGATED) &&
        // Success condition: aux_state
        (result.eq(RMI_SUCCESS) ==> AuxStateEqual32(s_post, PdevAt(s, pdev_ptr).aux, PdevAt(s, pdev_ptr).num_aux, DELEGATED))
;
```