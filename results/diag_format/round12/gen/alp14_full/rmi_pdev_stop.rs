pub open spec fn rmi_pdev_stop_spec(pdev_ptr: PdevPtr, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).state == PDEV_STOPPING)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
  && ((!(PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING) &&
       PdevAt(old_s, pdev_ptr).comm_state != DEV_COMM_PENDING)
    ==> result.is_Ok())
  && ((!(PdevAt(old_s, pdev_ptr).state == PDEV_COMMUNICATING) &&
       !(PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING) &&
       !(PdevAt(old_s, pdev_ptr).state == PDEV_STOPPED))
    ==> result.is_Ok())
  && (PdevAt(old_s, pdev_ptr).num_vdevs != 0 ==> result.is_Err())
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).state == PdevAt(old_s, pdev_ptr).state)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).comm_state == PdevAt(old_s, pdev_ptr).comm_state)
  && ((!(result.is_Ok()) &&
       (PdevAt(old_s, pdev_ptr).state == PDEV_STOPPING ||
        PdevAt(old_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
    ==> result.is_Err())
}