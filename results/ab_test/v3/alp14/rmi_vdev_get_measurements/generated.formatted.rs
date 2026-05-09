pub open spec fn rmi_vdev_get_measurements_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    vdev_ptr: Address,
    params_ptr: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(old_s, rd).state != RD
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!AddrIsGranuleAligned(vdev_ptr)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((VdevAt(
        old_s,
        vdev_ptr,
    ).vdev_state != VDEV_LOCKED && VdevAt(old_s, vdev_ptr).vdev_state != VDEV_STARTED)
        ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (VdevAt(old_s, vdev_ptr).comm_state
        != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (!AddrIsGranuleAligned(
        params_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!GranuleAccessPermitted(params_ptr, PAS_NS)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((RmiVdevMeasureParamsAt(
        old_s,
        params_ptr,
    ).indices[0] == 1u64 || RmiVdevMeasureParamsAt(old_s, params_ptr).indices[255] == 1u64)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(vdev_ptr) && PaIsDelegable(vdev_ptr) && GranuleAt(
        old_s,
        vdev_ptr,
    ).state == VDEV && VdevAt(old_s, vdev_ptr).realm == rd && (VdevAt(old_s, vdev_ptr).vdev_state
        == VDEV_LOCKED || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_STARTED) && VdevAt(
        old_s,
        vdev_ptr,
    ).comm_state == DEV_COMM_IDLE && AddrIsGranuleAligned(params_ptr) && GranuleAccessPermitted(
        params_ptr,
        PAS_NS,
    ) && RmiVdevMeasureParamsAt(old_s, params_ptr).indices[0] == 0u64 && RmiVdevMeasureParamsAt(
        old_s,
        params_ptr,
    ).indices[255] == 0u64) ==> (result.is_Ok() && VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_MEAS
        && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING))
}