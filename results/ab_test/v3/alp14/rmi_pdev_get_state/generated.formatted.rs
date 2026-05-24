pub open spec fn rmi_pdev_get_state_spec(result: RmiCommandReturnCode, state: RmiPdevState, pdev_ptr: Address, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV) ==> (result.is_Ok() && state == PdevAt(old_s, pdev_ptr).state && old_s == new_s))
}