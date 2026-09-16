pub open spec fn rmi_pdev_set_pubkey_spec(pdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!RmiPublicKeyParamsIsValid(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY ==> result.status == RMI_ERROR_DEVICE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr) &&
       RmiPublicKeyParamsIsValid(old_s, params_ptr) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}