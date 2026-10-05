pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: UInt64, vdev_flags: UInt64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> aux_count == 0)
}