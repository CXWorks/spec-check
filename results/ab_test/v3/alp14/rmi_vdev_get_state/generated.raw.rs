pub open spec fn rmi_vdev_get_state_spec(result: RmiCommandReturnCode, state: RmiVdevState, vdev_ptr: Address, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da == FEATURE_TRUE
      && AddrIsGranuleAligned(vdev_ptr)
      && PaIsDelegable(vdev_ptr)
      && GranuleAt(old_s, vdev_ptr).state == VDEV)
      ==> (!ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)
        && !ResultEqual(result, RMI_ERROR_INPUT)
        && Equal(state, VdevAt(old_s, vdev_ptr).vdev_state)
        && new_s == old_s))
}