pub open spec fn RMI_PDEV_GET_STATE_spec(
    s: S,
    pdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
    state: u8,
) -> bool {
    let pdev = PdevAt(s, pdev_ptr);

    (ImplFeatures(s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && (
    AddrIsGranuleAligned(s, pdev_ptr) || ResultEqual(result, RMI_ERROR_INPUT)) && (PaIsDelegable(
        s,
        pdev_ptr,
    ) || ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(s, pdev_ptr).state == PDEV
        || ResultEqual(result, RMI_ERROR_INPUT)) && (result.is_Ok() ==> state == pdev.state)
}