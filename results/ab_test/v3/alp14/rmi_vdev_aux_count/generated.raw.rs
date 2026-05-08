```verus
pub open spec fn RMI_VDEV_AUX_COUNT_spec(old_s: S, pdev_flags: u64, vdev_flags: u64, result: Result<(), RmiStatusCode>, aux_count: u64) -> bool {
    (
        // Failure case: da_supp
        (ImplFeatures(old_s).feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
        &&
        // Success case: aux_count
        (ImplFeatures(old_s).feat_da == FEATURE_TRUE ==> 
            (result.is_Ok() && aux_count == VdevAuxCount(old_s, RmiPdevFlagsDecode(old_s, pdev_flags), RmiVdevFlagsDecode(old_s, vdev_flags))))
    )
}
```