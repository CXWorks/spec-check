pub open spec fn RMI_VDEV_GET_MEASUREMENTS_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    params_ptr: Address,
    result: RmiCommandReturnCode,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);
    let params = RmiVdevMeasureParamsAt(old_s, params_ptr);

    ((!ImplFeatures(old_s).feat_da || ImplFeatures(old_s).feat_da == FEATURE_FALSE) ==> ResultEqual(
        result,
        RMI_ERROR_NOT_SUPPORTED,
    )) && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        rd,
    ).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(
        old_s,
        vdev_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((vdev.vdev_state != VDEV_LOCKED
        && vdev.vdev_state != VDEV_STARTED) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (
    vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (
    !AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && ((params.indices[0] == 1 || params.indices[255] == 1) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (result.is_Ok() ==> (VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_MEAS && VdevAt(
        new_s,
        vdev_ptr,
    ).comm_state == DEV_COMM_PENDING))
}