pub open spec fn rmi_vdev_lock_spec(rd: Rd, vdev_ptr: VdevPtr, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!is_aligned_to(old_s, rd, GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_delegable_physical_address(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAt(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_aligned_to(old_s, vdev_ptr, GRANULE_SIZE) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_delegable_physical_address(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranuleAt(old_s, vdev_ptr).state == VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).vdev_state != VDEV_UNLOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).pending_operation == VDEV_OP_LOCK)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da == FEATURE_FALSE) &&
       is_aligned_to(old_s, rd, GRANULE_SIZE) &&
       is_delegable_physical_address(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       is_aligned_to(old_s, vdev_ptr, GRANULE_SIZE) &&
       is_delegable_physical_address(old_s, vdev_ptr) &&
       GranuleAt(old_s, vdev_ptr).state == VDEV &&
       VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) &&
       !(VdevAt(old_s, vdev_ptr).vdev_state != VDEV_UNLOCKED) &&
       !(VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).pending_operation == VdevAt(old_s, vdev_ptr).pending_operation)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}