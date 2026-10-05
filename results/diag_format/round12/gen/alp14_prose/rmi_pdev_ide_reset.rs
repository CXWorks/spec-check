pub open spec fn rmi_pdev_ide_reset_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err() ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_IDE_RESETTING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(DeviceAssignmentSupported(old_s)) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state))
  && ((!(PdevAt(old_s, pdev_ptr).ide_granule_aligned(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state))
  && ((!(PdevAt(old_s, pdev_ptr).is_delegable_physical_address(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state))
  && ((!(PdevAt(old_s, pdev_ptr).ide_granule_in_pdev_state(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state))
  && ((!(PdevAt(old_s, pdev_ptr).ncoh_ide == IDE_TRUE) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state))
  && ((!(PdevAt(old_s, pdev_ptr).state == PDEV_READY) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    ==> (PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state))
  && ((result.is_Ok() &&
       DeviceAssignmentSupported(old_s) &&
       PdevAt(old_s, pdev_ptr).ide_granule_aligned(pdev_ptr) &&
       PdevAt(old_s, pdev_ptr).is_delegable_physical_address(pdev_ptr) &&
       PdevAt(old_s, pdev_ptr).ide_granule_in_pdev_state(pdev_ptr) &&
       PdevAt(old_s, pdev_ptr).ncoh_ide == IDE_TRUE &&
       PdevAt(old_s, pdev_ptr).state == PDEV_READY)
    ==> result.is_Ok())
  && (((!(DeviceAssignmentSupported(old_s))) ||
       (PdevAt(old_s, pdev_ptr).ide_granule_aligned(pdev_ptr)) ||
       (PdevAt(old_s, pdev_ptr).is_delegable_physical_address(pdev_ptr)) ||
       (PdevAt(old_s, pdev_ptr).ide_granule_in_pdev_state(pdev_ptr)) ||
       (PdevAt(old_s, pdev_ptr).ncoh_ide == IDE_TRUE) ||
       (PdevAt(old_s, pdev_ptr).state == PDEV_READY))
    ==> result.is_Ok())
}