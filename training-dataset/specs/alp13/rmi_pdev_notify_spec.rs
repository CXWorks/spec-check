pub open spec fn rmi_pdev_notify_spec(pdev_ptr: Address, ev: RmiPdevEvent, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (!RmiPdevEventIsValid(old_s, ev) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_COMMUNICATING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_READY) &&
       RmiPdevEventIsValid(old_s, ev))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}
