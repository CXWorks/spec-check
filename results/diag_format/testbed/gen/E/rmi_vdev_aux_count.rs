pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: Bits64, vdev_flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (!FeatureToRmi(old_s, FEATURE_DA) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (result.is_Ok() ==> aux_count == PdevAuxCount(new_s, RmiPdevFlagsDecode(new_s, pdev_flags)))
  && ((!(FeatureToRmi(old_s, FEATURE_DA)))
    ==> result.is_Ok())
}