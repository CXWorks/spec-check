pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: Bits64, vdev_flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da == RmmFeature::FEATURE_FALSE ==> ResultEqual(result, RmiStatusCode::RMI_ERROR_NOT_SUPPORTED))
    && (ResultEqual(result, RmiStatusCode::RMI_SUCCESS) ==> aux_count == VdevAuxCount(RmiPdevFlagsDecode(old_s, pdev_flags), RmiVdevFlagsDecode(old_s, vdev_flags)))
}