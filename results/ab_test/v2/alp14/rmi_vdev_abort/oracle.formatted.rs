pub open spec fn rmi_vdev_abort_spec(
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (
    !PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(
        old_s,
        vdev_ptr,
    ).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT)) && (VdevAt(old_s, vdev_ptr).comm_state
        == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE)) && (result.is_Ok() ==> VdevAt(
        new_s,
        vdev_ptr,
    ).vdev_state == VDEV_ERROR) && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state
        == DEV_COMM_IDLE) && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE)
        && AddrIsGranuleAligned(old_s, vdev_ptr) && PaIsDelegable(old_s, vdev_ptr) && !(GranuleAt(
        old_s,
        vdev_ptr,
    ).state != VDEV) && !(VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE)) ==> result.is_Ok())
        && (result.is_Err() ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(
        old_s,
        vdev_ptr,
    ).vdev_state) && (result.is_Err() ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(
        old_s,
        vdev_ptr,
    ).comm_state)
}