pub open spec fn rmi_vdev_get_state_spec(result: Result<(), RmiStatusCode>, vdev_ptr: Address, state: RmiVdevState, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
        && AddrIsGranuleAligned(vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV)
        ==> result.is_Ok()
            && state == VdevAt(old_s, vdev_ptr).vdev_state
            && new_s == old_s)
}