```verus
pub open spec fn RMI_PDEV_STOP_spec(s: S, pdev_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let impl_features = ImplFeatures(s);
    
    (impl_features.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    (!AddrIsGranuleAligned(s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((pdev.state == PDEV_COMMUNICATING || pdev.state == PDEV_STOPPING || pdev.state == PDEV_STOPPED) ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (pdev.num_vdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (result.is_Ok() ==> pdev.state == PDEV_STOPPING) &&
    (result.is_Ok() ==> pdev.comm_state == DEV_COMM_PENDING)
}
```