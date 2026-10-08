pub open spec fn rmi_vdev_get_interface_report_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (AddrIsGranuleAligned(old_s, rd) ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (AddrIsGranuleAligned(old_s, vdev_ptr) ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state == RD ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state == VDEV ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) ==> result.is_Ok() || result == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).vdev_state == VDEV_LOCKED || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_STARTED ==> result.is_Ok() || result == RMI_ERROR_DEVICE)
  && (VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE ==> result.is_Ok() || result == RMI_ERROR_DEVICE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_REPORT)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(AddrIsGranuleAligned(old_s, rd)) ||
       !(AddrIsGranuleAligned(old_s, vdev_ptr)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(GranuleAt(old_s, vdev_ptr).state == VDEV) ||
       !(VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd)) ||
       !(VdevAt(old_s, vdev_ptr).vdev_state == VDEV_LOCKED || VdevAt(old_s, vdev_ptr).vdev_state == VDEV_STARTED) ||
       !(VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE))
    ==> result == RMI_ERROR_INPUT)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}