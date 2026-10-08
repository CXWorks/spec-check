pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: Bits64, vdev_flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> aux_count == 0)
  && (result.is_Ok() ==> aux_count == PdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags)) + VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags), RmiVdevFlagsDecode(old_s, vdev_flags)))
}