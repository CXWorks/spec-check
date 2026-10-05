pub open spec fn rmi_vdev_start_spec(rd: PhysicalAddress, vdev_ptr: PhysicalAddress, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (VdevAt(old_s, vdev_ptr).vdev_op != VDEV_OP_START ==> VdevAt(new_s, vdev_ptr).vdev_op == VDEV_OP_START)
  && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_PENDING ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(is_delegable_physical_address(old_s, rd)) ||
       !(GranuleAt(old_s, rd).rd_state == RD_STATE) ||
       !(is_delegable_physical_address(old_s, vdev_ptr)) ||
       !(GranuleAt(old_s, vdev_ptr).vdev_state == VDEV_STATE) ||
       !(VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd)))
    ==> result.is_Err())
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).vdev_op == VDEV_OP_START)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).vdev_op == VdevAt(old_s, vdev_ptr).vdev_op)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && ((!(result.is_Err()) &&
       is_delegable_physical_address(old_s, rd) &&
       GranuleAt(old_s, rd).rd_state == RD_STATE &&
       is_delegable_physical_address(old_s, vdev_ptr) &&
       GranuleAt(old_s, vdev_ptr).vdev_state == VDEV_STATE &&
       VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd))
    ==> VdevAt(new_s, vdev_ptr).vdev_op == VDEV_OP_START)
  && ((!(result.is_Err()) &&
       is_delegable_physical_address(old_s, rd) &&
       GranuleAt(old_s, rd).rd_state == RD_STATE &&
       is_delegable_physical_address(old_s, vdev_ptr) &&
       GranuleAt(old_s, vdev_ptr).vdev_state == VDEV_STATE &&
       VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd))
    ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_op == VdevAt(old_s, vdev_ptr).vdev_op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (VdevAt(new_s, vdev_ptr).vdev_op == VdevAt(old_s, vdev_ptr).vdev_op
    ==> result.is_Err())
  && (VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state
    ==> result.is_Err())
}