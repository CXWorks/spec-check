pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: UInt64, result: Result<(), RmiStatusCode>, state: UInt8, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && ((vdev_ptr % GRANULE_SIZE) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!can_be_delegated(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).state != VDEV_STATE ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> state == VdevAt(new_s, vdev_ptr).state)
  && ((!(ImplFeatures(old_s).feat_da == FEATURE_FALSE) &&
       ((vdev_ptr % GRANULE_SIZE) == 0) &&
       can_be_delegated(old_s, vdev_ptr) &&
       !(VdevAt(old_s, vdev_ptr).state != VDEV_STATE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
}