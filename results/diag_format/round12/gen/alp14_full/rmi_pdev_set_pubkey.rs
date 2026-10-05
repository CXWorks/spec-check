pub open spec fn rmi_pdev_set_pubkey_spec(pdev_ptr: PdevPtr, params_ptr: PdevPtr, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEEDS_KEY)
  && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_IDLE)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(pdev_ptr.align_to(Granule::size()) as int) ||
       !PdevAt(old_s, pdev_ptr).is_delegable ||
       !PdevAt(old_s, pdev_ptr).is_in_pdev_state ||
       !(params_ptr.align_to(Granule::size()) as int) ||
       !PdevAt(old_s, params_ptr).accessible_in_ns ||
       PdevAt(old_s, params_ptr).key_length > 1024 ||
       PdevAt(old_s, params_ptr).metadata_length > 1024)
    ==> result.is_Err())
  && (result.is_Ok()
    ==> PdevAt(new_s, pdev_ptr).state == PDEV_HAS_KEY)
  && (result.is_Ok()
    ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result.is_Ok()) &&
       PdevAt(old_s, pdev_ptr).state == PDEV_NEEDS_KEY)
    ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEEDS_KEY)
  && (result.is_Ok()
    ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
}