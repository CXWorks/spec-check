pub open spec fn RMI_VDEV_START_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);

    ((!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> ResultEqual(
        result,
        RMI_ERROR_NOT_SUPPORTED(),
    )) && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT())) && (
    !PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT())) && (GranuleAt(
        old_s,
        rd,
    ).state as int != RD() as int ==> ResultEqual(result, RMI_ERROR_INPUT())) && (
    !AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT())) && (
    !PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT())) && (GranuleAt(
        old_s,
        vdev_ptr,
    ).state as int != VDEV() as int ==> ResultEqual(result, RMI_ERROR_INPUT())) && (
    vdev.realm as int != rd as int ==> ResultEqual(result, RMI_ERROR_INPUT())) && (
    vdev.vdev_state as int != VDEV_LOCKED() as int ==> ResultEqual(result, RMI_ERROR_DEVICE())) && (
    vdev.comm_state as int != DEV_COMM_IDLE() as int ==> ResultEqual(result, RMI_ERROR_DEVICE()))
        && (result.is_Ok() ==> (VdevAt(new_s, vdev_ptr).op as int == VDEV_OP_START() as int
        && VdevAt(new_s, vdev_ptr).comm_state as int == DEV_COMM_PENDING() as int)))
}