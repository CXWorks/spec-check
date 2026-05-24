pub open spec fn rmi_vdev_abort_spec(result: RmiCommandReturnCode, vdev_ptr: Address, old_s: S, new_s: S) -> bool {
    let vdev = VdevAt(old_s, vdev_ptr);
    (
        (!ImplFeatures(old_s).feat_da_is_true() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
        && (vdev.comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
        && (
            (ImplFeatures(old_s).feat_da_is_true()
            && AddrIsGranuleAligned(vdev_ptr)
            && PaIsDelegable(vdev_ptr)
            && GranuleAt(old_s, vdev_ptr).state == VDEV
            && vdev.comm_state != DEV_COMM_IDLE)
            ==> (result.is_Ok() && VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
        )
    )
}