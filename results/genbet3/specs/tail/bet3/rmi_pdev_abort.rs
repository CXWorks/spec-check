pub open spec fn rmi_pdev_abort_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm(old_s).static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).state == PDEV_NEW ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR)
  && (result.is_Ok() && PdevAt(old_s, pdev_ptr).state == PDEV_HAS_KEY ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR)
  && (result.is_Ok() && (PdevOperationIsStream(old_s, PdevAt(old_s, pdev_ptr).op) && PdevStreamOperationCanRevert(old_s, PdevAt(old_s, pdev_ptr))) ==> PdevAt(new_s, pdev_ptr).op == PDEV_OP_STREAM_ABORTED)
  && (result.is_Ok() && (PdevOperationIsStream(old_s, PdevAt(old_s, pdev_ptr).op) && !PdevStreamOperationCanRevert(old_s, PdevAt(old_s, pdev_ptr))) ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR)
  && (result.is_Ok() && (PdevOperationIsStream(old_s, PdevAt(old_s, pdev_ptr).op) && !PdevStreamOperationCanRevert(old_s, PdevAt(old_s, pdev_ptr))) ==> PdevAt(new_s, pdev_ptr).op == PDEV_OP_STREAM_COMPLETE)
  && (result.is_Ok() && (!PdevOperationCanRevert(old_s, PdevAt(old_s, pdev_ptr)) && !(PdevAt(old_s, pdev_ptr).state IN { PDEV_NEW, PDEV_HAS_KEY }) && !PdevOperationIsStream(old_s, PdevAt(old_s, pdev_ptr).op)) ==> PdevAt(new_s, pdev_ptr).state == PDEV_ERROR)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_IDLE)
  && ((!(Rmm(old_s).static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       !(PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_IDLE))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).op == PdevAt(old_s, pdev_ptr).op)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).op == PdevAt(old_s, pdev_ptr).op)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}