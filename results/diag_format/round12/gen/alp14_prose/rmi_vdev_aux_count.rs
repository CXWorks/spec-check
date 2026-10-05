pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: Bits64, vdev_flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
  (result.is_Err() ==> aux_count == 0)
  && ((!(ImplFeatures(old_s).feat_da == FEATURE_TRUE)) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && ((result.is_Ok())
    ==> aux_count > 0)
}