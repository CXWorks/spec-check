pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiVdevState, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (ResultEqual(result, RMI_SUCCESS) ==> state == VdevStateToRmi(GranuleAt(old_s, vdev_ptr).state))
}