pub open spec fn rmi_vdev_start_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> result.get_Err_0() == RMI_ERROR_NOT_SUPPORTED)
  && ((!(AddrIsGranuleAligned(old_s, rd)) ||
       !(PaIsDelegable(old_s, rd)) ||
       !(GranuleAt(old_s, rd).state == RD) ||
       !(AddrIsGranuleAligned(old_s, vdev_ptr)) ||
       !(PaIsDelegable(old_s, vdev_ptr)) ||
       !(GranuleAt(old_s, vdev_ptr).state == VDEV) ||
       !(VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd)))
    ==> result.get_Err_0() == RMI_ERROR_INPUT)
  && (result.get_Err_0() == RMI_ERROR_DEVICE ==> VdevAt(old_s, vdev_ptr).vdev_state == VDEV_LOCKED)
  && (result.get_Err_0() == RMI_ERROR_DEVICE ==> VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_START)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result.is_Err()) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       GranuleAt(old_s, rd).state == RD &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       GranuleAt(old_s, vdev_ptr).state == VDEV &&
       VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) &&
       VdevAt(old_s, vdev_ptr).vdev_state == VDEV_LOCKED &&
       VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
    ==> result.is_Ok())
  && ((result.is_Err())
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && ((result.is_Err())
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}