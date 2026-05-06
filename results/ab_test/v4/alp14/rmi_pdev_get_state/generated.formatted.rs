pub open spec fn RMI_PDEV_GET_STATE_spec(
    s: S,
    pdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
    state: RmiPdevState,
) -> bool {
    let pdev = PdevAt(s, pdev_ptr);
    ((!ImplFeatures(s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && (
    !AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        pdev_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(s, pdev_ptr).state != PDEV
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (ImplFeatures(s).feat_da
        && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && GranuleAt(s, pdev_ptr).state
        == PDEV ==> (result.is_Ok() && state == pdev.state)))
}