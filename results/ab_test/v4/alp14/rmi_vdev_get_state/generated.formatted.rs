pub open spec fn RMI_VDEV_GET_STATE_spec(
    s: S,
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
    state: u8,
) -> bool {
    let vdev = VdevAt(s, vdev_ptr);
    (ImplFeatures().feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) && (
    !AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        vdev_ptr,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(s, vdev_ptr).state != VDEV
        ==> ResultEqual(result, RMI_ERROR_INPUT)) && (result.is_Ok() ==> (state as int
        == vdev.vdev_state as int))
}