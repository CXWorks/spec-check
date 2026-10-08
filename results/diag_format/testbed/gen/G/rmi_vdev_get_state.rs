pub open spec fn rmi_vdev_get_state_spec(vdev_ptr: Address, result: Result<(), RmiStatusCode>, state: RmiVdevState, old_s: S, new_s: S) -> bool {
  (result.is_Err() && result.get_Err_0() == RMI_ERROR_NOT_SUPPORTED ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW)
  && (result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW)
  && (result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW)
  && (result.is_Err() && result.get_Err_0() == RMI_ERROR_INPUT ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_NEW)
  && ((result.is_Ok())
    ==> state == VdevAt(new_s, vdev_ptr).vdev_state)
  && ((!(result.is_Err() && result.get_Err_0() == RMI_ERROR_NOT_SUPPORTED) &&
       result.is_Ok())
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
}