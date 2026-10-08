pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: Bits64, vdev_flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da == FEATURE_FALSE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (result.is_Ok() ==> ResultEqual(result, RMI_SUCCESS))
  && (result.is_Ok() ==> aux_count == VdevAuxCount(new_s, RmiPdevFlagsDecode(new_s, pdev_flags), RmiVdevFlagsDecode(new_s, vdev_flags)))
  && ((!(ImplFeatures(old_s).feat_da == FEATURE_FALSE))
    ==> result.is_Ok())
}