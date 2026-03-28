pub open spec fn rmi_pdev_aux_count_spec(value: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!RmiPdevFlagsSupported(old_s, RmiPdevFlagsDecode(old_s, value)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (result.is_Ok() ==> aux_count == PdevAuxCount(new_s, RmiPdevFlagsDecode(new_s, value)))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE) &&
       RmiPdevFlagsSupported(old_s, RmiPdevFlagsDecode(old_s, value)))
    ==> result.is_Ok())
}
