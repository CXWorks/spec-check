pub open spec fn RMI_VDEV_GET_MEASUREMENTS_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    params_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);
    let params = RmiVdevMeasureParamsAt(old_s, params_ptr);

    // Failure condition: da_supp
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_bound
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: rd_state
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_align
    (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_bound
    (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_gran_state
    (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_realm
    (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: vdev_state
    ((vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED) ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    )) &&
    // Failure condition: comm_state
    (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
        &&
    // Failure condition: params_align
    (!AddrIsGranuleAligned(params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: params_pas
    (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Failure condition: indices
    ((params.indices[0] == 1 || params.indices[255] == 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Success conditions (when no failure condition applies)
    (result.is_Ok() ==> (VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_MEAS && VdevAt(
        new_s,
        vdev_ptr,
    ).comm_state == DEV_COMM_PENDING))
}