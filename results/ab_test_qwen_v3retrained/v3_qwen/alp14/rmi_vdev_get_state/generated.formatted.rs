pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: Address, result: RmiCommandReturnCode, state: RmiVdevState, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> result == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsDelegable(old_s, vdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> result == RMI_ERROR_INPUT)
  && (result == RMI_SUCCESS ==> state == VdevAt(new_s, vdev_ptr).vdev_state)
  && ((!ImplFeatures(old_s).feat_da != FEATURE_TRUE &&
        AddrIsGranuleAligned(old_s, vdev_ptr) &&
        PaIsDelegable(old_s, vdev_ptr) &&
        !(GranuleAt(old_s, vdev_ptr).state != VDEV))
    ==> result == RMI_SUCCESS)
  && (result != RMI_SUCCESS
    ==> state == state)
}