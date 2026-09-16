pub open spec fn rmi_pdev_destroy_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTrackedFine(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED ==> result.status == RMI_ERROR_DEVICE)
  && (result.is_Ok() ==> GranuleAt(new_s, pdev_ptr).state == GRAN_DELEGATED)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTrackedFine(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_STOPPED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, pdev_ptr).state == GranuleAt(old_s, pdev_ptr).state)
}