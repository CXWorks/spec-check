pub open spec fn rmi_vdev_lock_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && ((!(AddrIsDelegablePhysicalAddress(old_s, rd)) ||
       !(GranuleAt(old_s, rd).state == RmmGranuleState::RD))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!(AddrIsDelegablePhysicalAddress(old_s, vdev_ptr)) ||
       !(GranuleAt(old_s, vdev_ptr).state == RmmGranuleState::VDEV))
    ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && VdevAt(new_s, vdev_ptr).vdev_state == VDEV_LOCKED ==> ResultEqual(result, RMI_SUCCESS))
  && (result.is_Ok() && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING ==> ResultEqual(result, RMI_SUCCESS))
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && ((!(ImplFeatures(old_s).feat_da == FEATURE_FALSE) &&
       AddrIsDelegablePhysicalAddress(old_s, rd) &&
       GranuleAt(old_s, rd).state == RmmGranuleState::RD &&
       AddrIsDelegablePhysicalAddress(old_s, vdev_ptr) &&
       GranuleAt(old_s, vdev_ptr).state == RmmGranuleState::VDEV &&
       VdevAt(old_s, vdev_ptr).vdev_state == VDEV_UNLOCKED &&
       VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
}