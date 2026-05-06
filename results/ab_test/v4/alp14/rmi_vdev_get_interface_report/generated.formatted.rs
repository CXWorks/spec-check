pub open spec fn RMI_VDEV_GET_INTERFACE_REPORT_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);

    ((!ImplFeatures(old_s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && ((
    !AddrIsGranuleAligned(rd)) ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PaIsDelegable(rd))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((GranuleAt(old_s, rd).state != RD)
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!AddrIsGranuleAligned(vdev_ptr))
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && ((!PaIsDelegable(vdev_ptr)) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && ((GranuleAt(old_s, vdev_ptr).state != VDEV) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    (vdev.realm != rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (((vdev.vdev_state
        != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED)) ==> ResultEqual(
        result,
        RMI_ERROR_DEVICE,
    )) && ((vdev.comm_state != DEV_COMM_IDLE) ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (
    result.is_Ok() ==> (VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT && VdevAt(
        new_s,
        vdev_ptr,
    ).comm_state == DEV_COMM_PENDING))
}