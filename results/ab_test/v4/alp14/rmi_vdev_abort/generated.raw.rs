```verus
pub open spec fn RMI_VDEV_ABORT_spec(old_s: S, new_s: S, vdev_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    let vdev = VdevAt(old_s, vdev_ptr);
    (
        (!ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED())) &&
        (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        (GranuleAt(old_s, vdev_ptr).state != RmmGranuleState::VDEV ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        (vdev.comm_state == RmmDevCommState::DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE())) &&
        (
            ImplFeatures(old_s).feat_da.is_FEATURE_TRUE() &&
            AddrIsGranuleAligned(vdev_ptr) &&
            PaIsDelegable(vdev_ptr) &&
            GranuleAt(old_s, vdev_ptr).state == RmmGranuleState::VDEV &&
            vdev.comm_state != RmmDevCommState::DEV_COMM_IDLE
        ) ==> (
            result.is_Ok() &&
            VdevAt(new_s, vdev_ptr).vdev_state == RmmVdevState::VDEV_ERROR &&
            VdevAt(new_s, vdev_ptr).comm_state == RmmDevCommState::DEV_COMM_IDLE
        )
    )
}
```