pub open spec fn rmi_vdev_get_measurements_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> result.status == RMI_ERROR_DEVICE)
  && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> result.status == RMI_ERROR_DEVICE)
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_MEAS)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsTracked(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !(VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).op == VdevAt(old_s, vdev_ptr).op)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}