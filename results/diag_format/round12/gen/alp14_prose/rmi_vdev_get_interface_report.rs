pub open spec fn rmi_vdev_get_interface_report_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(is_rd_delegatable(old_s, rd)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(is_vdev_ptr_delegatable(old_s, vdev_ptr)) ||
       !(GranuleAt(old_s, vdev_ptr).state == VDEV) ||
       !(VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd)) ||
       !(VdevAt(old_s, vdev_ptr).state == VDEV_LOCKED) ||
       !(VdevAt(old_s, vdev_ptr).state == VDEV_STARTED) ||
       !(VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE))
    ==> result.is_Err())
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT)
  && (result.is_Ok()
    ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(is_rd_delegatable(old_s, rd)) &&
       (GranuleAt(old_s, rd).state == RD) &&
       is_vdev_ptr_delegatable(old_s, vdev_ptr) &&
       (GranuleAt(old_s, vdev_ptr).state == VDEV) &&
       (VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd)) &&
       (VdevAt(old_s, vdev_ptr).state == VDEV_LOCKED) &&
       (VdevAt(old_s, vdev_ptr).state == VDEV_STARTED) &&
       (VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
  && (VdevAt(new_s, vdev_ptr).state == VdevAt(old_s, vdev_ptr).state)
}