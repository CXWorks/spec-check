pub open spec fn rmi_pdev_set_pubkey_spec(pdev_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsWithin(old_s, params_ptr, 0, 0) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiPublicKeyParamsAt(old_s, params_ptr).key.len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RmiPublicKeyParamsAt(old_s, params_ptr).metadata.len > 1024 ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((AddrIsGranuleAligned(old_s, pdev_ptr) &&
       PaIsDelegable(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != PDEV) &&
       AddrIsGranuleAligned(old_s, params_ptr) &&
       AddrIsWithin(old_s, params_ptr, 0, 0) &&
       !(RmiPublicKeyParamsAt(old_s, params_ptr).key.len > 1024) &&
       !(RmiPublicKeyParamsAt(old_s, params_ptr).metadata.len > 1024) &&
       !(PdevAt(old_s, pdev_ptr).state != PDEV_NEEDS_KEY))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}