pub open spec fn rmi_vdev_get_state_spec(
    result: RmiCommandReturnCode,
    state: u8,
    old_s: S,
    new_s: S,
    vdev_ptr: Address,
) -> bool {
    let vdev = VdevAt(old_s, vdev_ptr);

    // Failure condition: da_supp
    ((!ImplFeatures(old_s).feat_da_eq(FEATURE_TRUE)) ==> ResultEqual(
        Err(result),
        RMI_ERROR_NOT_SUPPORTED,
    ))
    // Failure condition: vdev_align
     && ((!AddrIsGranuleAligned(old_s, vdev_ptr)) ==> ResultEqual(
        Err(result),
        RMI_ERROR_INPUT,
    ))
    // Failure condition: vdev_bound
     && ((!PaIsDelegable(old_s, vdev_ptr)) ==> ResultEqual(
        Err(result),
        RMI_ERROR_INPUT,
    ))
    // Failure condition: vdev_gran_state
     && ((GranuleAt(old_s, vdev_ptr).state != VDEV) ==> ResultEqual(
        Err(result),
        RMI_ERROR_INPUT,
    ))
    // Success condition: state
     && ((ImplFeatures(old_s).feat_da_eq(FEATURE_TRUE) && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV) ==> (result
        == RMI_SUCCESS && state == vdev.vdev_state))
    // No footprint
     && (new_s == old_s)
}