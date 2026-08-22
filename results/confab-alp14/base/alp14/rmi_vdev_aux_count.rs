pub open spec fn rmi_vdev_aux_count_spec(result: Result<(), RmiStatusCode>, pdev_flags: Bits64, vdev_flags: Bits64, aux_count: UInt64, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> result.is_Ok()
        && aux_count as int == VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags), RmiVdevFlagsDecode(old_s, vdev_flags))
        && new_s == old_s)
}