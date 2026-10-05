pub open spec fn rmi_pdev_stop_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPED)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_INPUT) ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err() && ResultEqual(result, RMI_ERROR_DEVICE) ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(impl_supports_device_assignment(old_s)) &&
       !(is_aligned_to_granule_size(old_s, pdev_ptr)) &&
       !(is_delegable_physical_address(old_s, pdev_ptr)) &&
       !(PdevAt(old_s, pdev_ptr).state == PDEV) &&
       !(PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING &&
        PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING &&
        PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED) &&
       !(PdevAt(old_s, pdev_ptr).num_vdevs == 0))
    ==> result.is_Err())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (!(result.is_Ok() && (PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING)) ==> result.is_Err())
}