pub open spec fn rmi_pdev_stop_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!(PdevAt(old_s, pdev_ptr).state IN { PDEV_NEEDS_KEY, PDEV_READY, PDEV_ERROR }) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (PdevAt(old_s, pdev_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (PdevAt(old_s, pdev_ptr).num_vdevs != 0 ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (PdevStreamLive(old_s, PdevAt(old_s, pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).op == PDEV_OP_STOP)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       (PdevAt(old_s, pdev_ptr).state IN { PDEV_NEEDS_KEY, PDEV_READY, PDEV_ERROR }) &&
       !(PdevAt(old_s, pdev_ptr).comm_state != DEV_COMM_IDLE) &&
       !(PdevAt(old_s, pdev_ptr).num_vdevs != 0) &&
       !(PdevStreamLive(old_s, PdevAt(old_s, pdev_ptr))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).op == PdevAt(old_s, pdev_ptr).op)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}