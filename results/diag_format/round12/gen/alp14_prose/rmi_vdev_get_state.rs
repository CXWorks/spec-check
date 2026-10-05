pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiVdevState, old_s: S, new_s: S) -> bool {
  (DeviceAssignmentSupported(old_s) ==> result.is_Ok())
  && (!DeviceAssignmentSupported(old_s) ==> result.is_Err())
  && (result.is_Ok() && !((vdev_ptr) % granule_size(old_s) == 0) ==> result.is_Err())
  && (result.is_Ok() && !is_delegatable_physical_address(old_s, vdev_ptr) ==> result.is_Err())
  && (result.is_Ok() && GranuleAt(old_s, vdev_ptr).state != VDEV ==> result.is_Err())
  && (result.is_Ok() ==> state == VdevAt(new_s, vdev_ptr).state)
  && ((!(DeviceAssignmentSupported(old_s)) ||
       ((vdev_ptr) % granule_size(old_s) == 0) &&
       is_delegatable_physical_address(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
}