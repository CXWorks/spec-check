pub open spec fn rmi_vdev_abort_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).vdev_state == VDEV_NEW ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR)
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).vdev_state == VDEV_KEY_REFRESH ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_LOCKED)
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).vdev_state == VDEV_KEY_PURGE ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_LOCKED)
  && (result.is_Ok() && (!VdevOperationCanRevert(old_s, VdevAt(old_s, vdev_ptr)) && !(VdevAt(old_s, vdev_ptr).vdev_state in [VDEV_NEW, VDEV_KEY_REFRESH, VDEV_KEY_PURGE])) ==> VdevAt(new_s, vdev_ptr).vdev_state == VDEV_ERROR)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_IDLE)
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
       !(VdevAt(old_s, vdev_ptr).comm_state == DEV_COMM_IDLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).comm_state == VdevAt(old_s, vdev_ptr).comm_state)
}