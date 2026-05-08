pub open spec fn RMI_VDEV_GET_STATE_spec(
    s: S,
    vdev_ptr: Address,
    result: Result<(), RmiStatusCode>,
    state: u8,
) -> bool {
    let vdev = VdevAt(s, vdev_ptr);
    (if !ImplFeatures_feat_da(s) {
        ResultEqual(result, RmiStatusCode::RMI_ERROR_NOT_SUPPORTED)
    } else if !AddrIsGranuleAligned(vdev_ptr) {
        ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)
    } else if !PaIsDelegable(vdev_ptr) {
        ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)
    } else if GranuleAt(s, vdev_ptr).state != RmmGranuleState::VDEV {
        ResultEqual(result, RmiStatusCode::RMI_ERROR_INPUT)
    } else {
        result.is_Ok() && state == vdev.vdev_state
    })
}