pub open spec fn RMI_PDEV_ABORT_spec(
    old_s: S,
    new_s: S,
    pdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_state_pre = pdev.state;
    let pdev_new = PdevAt(new_s, pdev_ptr);

    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // Failure condition: pdev_align
    (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_bound
    (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_gran_state
    (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: pdev_state
    ((pdev_state_pre != PDEV_NEW && pdev_state_pre != PDEV_HAS_KEY && pdev_state_pre
        != PDEV_COMMUNICATING) ==> ResultEqual(result, RMI_ERROR_DEVICE))
        &&
    // Success condition: comm
    (pdev_state_pre == PDEV_COMMUNICATING ==> (result.is_Ok() && pdev_new.state == PDEV_READY
        && pdev_new.comm_state == DEV_COMM_IDLE))
        &&
    // Success condition: not_comm
    (pdev_state_pre != PDEV_COMMUNICATING ==> (result.is_Ok() && pdev_new.comm_state
        == DEV_COMM_PENDING))
}