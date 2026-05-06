```verus
pub open spec fn RMI_PDEV_IDE_RESET_spec(s: S, pdev_ptr: Address, result: RmiCommandReturnCode) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    let impl_features = ImplFeatures(s);
    
    (impl_features.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
    (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (pdev.ncoh_ide != IDE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    (pdev.state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE)) &&
    
    (impl_features.feat_da == FEATURE_TRUE && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && 
     GranuleAt(s, pdev_ptr).state == PDEV && pdev.ncoh_ide == IDE_TRUE && pdev.state == PDEV_READY ==>
        (result.is_Ok() && pdev.state == PDEV_IDE_RESETTING && pdev.comm_state == DEV_COMM_PENDING))
}
```