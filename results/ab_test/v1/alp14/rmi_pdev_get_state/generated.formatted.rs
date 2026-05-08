```rust
pub open spec fn RMI_PDEV_GET_STATE_spec(s: S, pdev_ptr: Address) -> (RmiCommandReturnCode, RmiPdevState) {
    let pdev = PdevAt(s, pdev_ptr);
    
    if !ImplFeatures(s).feat_da.is_FEATURE_TRUE() {
        (RMI_ERROR_NOT_SUPPORTED(), arbitrary())
    } else if !AddrIsGranuleAligned(pdev_ptr) {
        (RMI_ERROR_INPUT(), arbitrary())
    } else if !PaIsDelegable(pdev_ptr) {
        (RMI_ERROR_INPUT(), arbitrary())
    } else if GranuleAt(s, pdev_ptr).state != PDEV() {
        (RMI_ERROR_INPUT(), arbitrary())
    } else {
        (RMI_SUCCESS(), pdev.state)
    }
}
```

This function:
1. Extracts the PDEV at `pdev_ptr`
2. Checks failure conditions in order of precedence:
   - DA support feature check (highest precedence)
   - Granule alignment check
   - Physical address delegability check
   - Granule state validation check
3. Returns appropriate error codes for each failure condition
4. On success, returns `RMI_SUCCESS()` with the PDEV's state

The output bits X1[63:8] are MBZ (must be zero), which is handled implicitly by the `RmiPdevState` type definition.