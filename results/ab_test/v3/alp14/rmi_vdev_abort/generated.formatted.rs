pub open spec fn RMI_VDEV_ABORT_spec(
    old_s: S,
    new_s: S,
    vdev_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let vdev = VdevAt(old_s, vdev_ptr);

    // da_supp: ImplFeatures().feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // vdev_align: !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // vdev_bound: !PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // vdev_gran_state: GranuleAt(vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)
    (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // comm_state: vdev.comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)
    (vdev.comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
        &&
    // Success conditions: if no failure conditions apply
    ((ImplFeatures(old_s).feat_da == FEATURE_TRUE && AddrIsGranuleAligned(vdev_ptr)
        && PaIsDelegable(vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV && vdev.comm_state
        != DEV_COMM_IDLE) ==> (
    // state: vdev.vdev_state == VDEV_ERROR
    VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR
        &&
    // comm_state: vdev.comm_state == DEV_COMM_IDLE
    VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE
        &&
    // Result is success
    result.is_Ok()))
}