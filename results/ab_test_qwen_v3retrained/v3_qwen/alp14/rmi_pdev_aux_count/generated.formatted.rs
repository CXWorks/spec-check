pub open spec fn rmi_pdev_aux_count_spec(value: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!RmiPdevFlagsSupported(old_s, RmiPdevFlagsDecode(value)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> RmiPdevAuxCount(new_s, RmiPdevFlagsDecode(value)) == new_s.rmi_pdev_aux_count)
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       RmiPdevFlagsSupported(old_s, RmiPdevFlagsDecode(value)))
    ==> result.is_Ok())
}