pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiVdevState, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> Equal(state, VdevAt(new_s, vdev_ptr).vdev_state))
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsTracked(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV))
    ==> result.is_Ok())
}