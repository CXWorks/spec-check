pub open spec fn rmi_pdev_get_state_spec(
    result: RmiCommandReturnCode,
    state: RmiPdevState,
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);

    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(
        result,
        RMI_ERROR_NOT_SUPPORTED,
    ))
    // Failure condition: pdev_align
     && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Failure condition: pdev_bound
     && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Failure condition: pdev_gran_state
     && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Success condition: state
     && (ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(old_s, pdev_ptr)
        && PaIsDelegable(old_s, pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV ==> state
        == pdev.state)
    // No footprint: state unchanged
     && (new_s == old_s)
}