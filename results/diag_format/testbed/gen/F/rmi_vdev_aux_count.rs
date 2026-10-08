pub open spec fn rmi_vdev_aux_count_spec(pdev_flags: Bits64, vdev_flags: Bits64, result: Result<(), RmiStatusCode>, aux_count: UInt64, old_s: S, new_s: S) -> bool {
    (!ResultEqual(result, RMI_ERROR_NOT_SUPPORTED) ==> (impl_features(old_s).feat_da == RMM_FEATURE_TRUE))
    && (result.is_Ok() ==> (aux_count == PdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags))))
    && (result.is_Ok() ==> (aux_count == VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags), RmiVdevFlagsDecode(old_s, vdev_flags))))
    && (result.is_Ok() ==> (new_s == old_s))
}