pub open spec fn rmi_vdev_aux_count_spec(result: RmiCommandReturnCode, pdev_flags: u64, vdev_flags: u64, aux_count: u64, old_s: S, new_s: S) -> bool {
    (!(ImplFeatures(old_s).feat_da == FEATURE_TRUE) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> (result.is_Ok() && aux_count == VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags), RmiVdevFlagsDecode(old_s, vdev_flags))))
}