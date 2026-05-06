pub open spec fn RMI_PDEV_GET_STATE_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
    state: RmiPdevState,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);

    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // Failure condition: pdev_align
    (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_bound
    (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_gran_state
    (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Success condition: state
    (result.is_Ok() ==> state == pdev.state) &&
    // State does not change
    (new_s == old_s)
}