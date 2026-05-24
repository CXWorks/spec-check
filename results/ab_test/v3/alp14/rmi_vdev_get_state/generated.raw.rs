pub open spec fn rmi_vdev_get_state_spec(result: RmiCommandReturnCode, vdev_ptr: Address, state: u64, old_s: S, new_s: S) -> bool {
    let vdev = VdevAt(old_s, vdev_ptr);
    (!ImplFeatures(old_s).feat_da.equals(FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da.equals(FEATURE_TRUE) && AddrIsGranuleAligned(old_s, vdev_ptr) && PaIsDelegable(old_s, vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV) ==> (result.is_Ok() && (state as int & 0xFF) == (vdev.vdev_state as int)))
}