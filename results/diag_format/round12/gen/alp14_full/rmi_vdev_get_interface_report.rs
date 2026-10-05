pub open spec fn rmi_vdev_get_interface_report_spec(rd: Rd, vdev_ptr: VdevPtr, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> result.is_Ok())
  && (!is_rd_delegatable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!is_vdev_ptr_delegatable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() && (VdevAt(old_s, vdev_ptr).state == VDEV_LOCKED || VdevAt(old_s, vdev_ptr).state == VDEV_STARTED) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).pending_op == VDEV_OP_GET_REPORT)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da == FEATURE_TRUE) &&
       is_rd_delegatable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       is_vdev_ptr_delegatable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).pending_op == VdevAt(old_s, vdev_ptr).pending_op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}