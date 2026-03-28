pub open spec fn rmi_pdev_aux_count_spec(flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (result.is_Ok() ==> aux_count == PdevAuxCount(new_s, RmiPdevFlagsDecode(new_s, flags)))
  && ((!(ImplFeatures(old_s).feat_da != FEATURE_TRUE))
    ==> result.is_Ok())
}
