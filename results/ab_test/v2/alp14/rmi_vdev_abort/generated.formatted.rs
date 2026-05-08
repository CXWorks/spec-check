pub open spec fn rmi_vdev_abort_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    vdev_ptr: Address,
) -> bool {
    let vdev = VdevAt(old_s, vdev_ptr);

    // Failure conditions
    let da_supp_fail = ImplFeatures().feat_da != FEATURE_TRUE ==> ResultEqual(
        result,
        RMI_ERROR_NOT_SUPPORTED,
    );
    let vdev_align_fail = !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_bound_fail = !PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT);
    let vdev_gran_state_fail = GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    );
    let comm_state_fail = vdev.comm_state == DEV_COMM_IDLE ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    );

    // Success conditions
    let state_success = result.is_Ok() ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR;
    let comm_state_success = result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE;

    // Ordering: [da_supp] < [vdev_align, vdev_bound, vdev_gran_state] < [comm_state]
    da_supp_fail && (ImplFeatures().feat_da == FEATURE_TRUE ==> (vdev_align_fail && vdev_bound_fail
        && vdev_gran_state_fail)) && (ImplFeatures().feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(vdev_ptr) && PaIsDelegable(vdev_ptr) && GranuleAt(
        old_s,
        vdev_ptr,
    ).state == VDEV ==> comm_state_fail) && state_success && comm_state_success
}