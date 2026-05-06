```verus
pub open spec fn RMI_PDEV_SET_PUBKEY_spec(s: S, pdev_ptr: Address, params_ptr: Address) -> (result: Result<(), RmiStatusCode>) {
    let pdev = PdevAt(s, pdev_ptr);
    let params = RmiPublicKeyParamsAt(s, params_ptr);
    
    // Failure conditions in priority order
    if !ImplFeatures(s).feat_da {
        Err(RMI_ERROR_NOT_SUPPORTED)
    } else if !AddrIsGranuleAligned(pdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !PaIsDelegable(pdev_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if GranuleAt(s, pdev_ptr).state != PDEV {
        Err(RMI_ERROR_INPUT)
    } else if !AddrIsGranuleAligned(params_ptr) {
        Err(RMI_ERROR_INPUT)
    } else if !GranuleAccessPermitted(s, params_ptr, PAS_NS) {
        Err(RMI_ERROR_INPUT)
    } else if params.key_len > 1024 {
        Err(RMI_ERROR_INPUT)
    } else if params.metadata_len > 1024 {
        Err(RMI_ERROR_INPUT)
    } else if !IsKeyValid(s, params) {
        Err(RMI_ERROR_INPUT)
    } else if !IsMetadataValid(s, params) {
        Err(RMI_ERROR_INPUT)
    } else if pdev.state != PDEV_NEEDS_KEY {
        Err(RMI_ERROR_DEVICE)
    } else {
        // Success conditions
        Ok(())
    }
}
```

**Notes:**
- The function follows the failure condition ordering specified in the command specification
- `ImplFeatures(s)` and `GranuleAt(s, ...)` are assumed spec functions for accessing implementation features and granule state
- `IsKeyValid(s, params)` and `IsMetadataValid(s, params)` are placeholder spec functions representing the cryptographic validation checks described in the specification
- Success is reached when all validation checks pass
- The function uses `Result<(), RmiStatusCode>` to match the return type pattern of other RMI command specifications