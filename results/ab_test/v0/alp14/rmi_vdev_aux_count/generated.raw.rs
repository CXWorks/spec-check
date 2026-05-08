```verus
pub open spec fn RMI_VDEV_AUX_COUNT_spec(s: S, pdev_flags: u64, vdev_flags: u64, result: RmiCommandReturnCode, aux_count: u64) -> bool {
  let pdev_flags_decoded = RmiPdevFlagsDecode(s, pdev_flags);
  let vdev_flags_decoded = RmiVdevFlagsDecode(s, vdev_flags);
  
  ((!s.features.feat_da) ==> (result == RMI_ERROR_NOT_SUPPORTED as RmiCommandReturnCode)) &&
  (s.features.feat_da ==> (aux_count == ToBits64(VdevAuxCount(s, pdev_flags_decoded, vdev_flags_decoded))))
}
```