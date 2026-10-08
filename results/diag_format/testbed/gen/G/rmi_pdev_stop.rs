pub open spec fn rmi_pdev_stop_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) && !( (pdev_ptr as int) % RMM_GRANULE_SIZE == 0) ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) && !PaIsDelegable(new_s, pdev_ptr) ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) && !(GranuleAt(new_s, pdev_ptr).state == PDEV) ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && PdevAt(new_s, pdev_ptr).state == PDEV_COMMUNICATING ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && PdevAt(new_s, pdev_ptr).state == PDEV_STOPPED ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && PdevAt(new_s, pdev_ptr).num_vdevs != 0 ==> PdevAt(new_s, pdev_ptr).state == PDEV_NEW)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(result.is_Err() && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED)) &&
       (result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) && ((pdev_ptr as int) % RMM_GRANULE_SIZE == 0))) &&
       (result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) && PaIsDelegable(old_s, pdev_ptr))) &&
       (result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) && (GranuleAt(old_s, pdev_ptr).state == PDEV))) &&
       (result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && !(PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING))) &&
       (result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && !(PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING))) &&
       (result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && !(PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED))) &&
       (result.is_Ok() || (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) && PdevAt(old_s, pdev_ptr).num_vdevs == 0)))
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
}