pub open spec fn rmi_pdev_ide_reset_spec(pdev_ptr: PhysAddr, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> PDEVAt(new_s, pdev_ptr).state == PDEV_READY)
  && (result.is_Err() ==> PDEVAt(new_s, pdev_ptr).comm_state == DEV_COMM_IDLE)
  && (result.is_Ok() ==> PDEVAt(new_s, pdev_ptr).state == PDEV_IDE_RESETTING)
  && (result.is_Ok() ==> PDEVAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(PDEVAt(old_s, pdev_ptr).state == PDEV_READY) &&
       PDEVAt(old_s, pdev_ptr).comm_state == DEV_COMM_IDLE)
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PDEVAt(new_s, pdev_ptr).state == PDEVAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PDEVAt(new_s, pdev_ptr).comm_state == PDEVAt(old_s, pdev_ptr).comm_state)
}