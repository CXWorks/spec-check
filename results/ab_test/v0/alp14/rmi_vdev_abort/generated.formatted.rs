pub open spec fn RMI_VDEV_ABORT_spec(
    s: S,
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let vdev = VdevAt(s, vdev_ptr);

    // Failure condition: da_supp
    (!(ImplFeatures(s).feat_da == FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // Failure condition: vdev_align
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_bound
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_gran_state
    (GranuleAt(s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: comm_state
    (vdev.comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
        &&
    // Success conditions
    (result.is_Ok() ==> (vdev.vdev_state == VDEV_ERROR && vdev.comm_state == DEV_COMM_IDLE))
}